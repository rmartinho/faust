use std::{
    collections::HashMap,
    io::{Cursor, Read, Seek},
};

use anyhow::{Result, bail, ensure};
use byteorder::{LittleEndian as LE, ReadBytesExt};

use crate::parse::manifest::ParserMode;

pub fn parse(data: impl AsRef<[u8]>, _: ParserMode) -> Result<HashMap<String, Sprite>> {
    let mut r = Cursor::new(data.as_ref());
    let magic = r.read_u32::<LE>()?;
    if magic != 0x6 {
        bail!("invalid .sd file")
    }
    let pages = r.read_u32::<LE>()?;
    let entries = r.read_u32::<LE>()?;
    let page_files: Vec<_> = (0..pages)
        .map(|_| read_page(&mut r))
        .collect::<Result<_>>()?;
    (0..entries)
        .map(|_| read_sprite(&mut r, &page_files))
        .collect()
}

fn read_page<R>(r: &mut R) -> Result<String>
where
    R: Read + Seek,
{
    let file = read_string(r, true)?;
    let _width = r.read_u32::<LE>()?;
    let _height = r.read_u32::<LE>()?;
    let bitmask_len = r.read_u32::<LE>()?;
    r.seek_relative(bitmask_len as _)?;
    Ok(file)
}

fn read_sprite(r: &mut impl Read, pages: &[String]) -> Result<(String, Sprite)> {
    let key = read_string(r, false)?;
    let idx = r.read_u16::<LE>()? as usize;
    let left = r.read_u16::<LE>()?;
    let right = r.read_u16::<LE>()?;
    let top = r.read_u16::<LE>()?;
    let bottom = r.read_u16::<LE>()?;
    let _alpha = r.read_u8()?;
    let _cursor = r.read_u8()?;
    let _x = r.read_u16::<LE>()?;
    let _y = r.read_u16::<LE>()?;
    Ok((
        key,
        Sprite {
            file: pages[idx].clone(),
            top,
            left,
            bottom,
            right,
        },
    ))
}

fn read_string(r: &mut impl Read, null_terminated: bool) -> Result<String> {
    let len = r.read_u32::<LE>()?;
    let mut buf = vec![0; len as usize];
    r.read_exact(&mut buf)?;
    if null_terminated {
        ensure!(r.read_u8()? == 0, "missing null terminator in string");
    }
    Ok(String::from_utf8(buf)?)
}

pub struct Sprite {
    pub file: String,
    pub top: u16,
    pub left: u16,
    pub bottom: u16,
    pub right: u16,
}
