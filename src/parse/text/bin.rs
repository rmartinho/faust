use std::{
    collections::HashMap,
    io::{Cursor, Read},
};

use anyhow::{Result, bail};
use byteorder::{LittleEndian as LE, ReadBytesExt};

pub fn parse(data: impl AsRef<[u8]>) -> Result<HashMap<String, String>> {
    let mut r = Cursor::new(data.as_ref());
    let typ = r.read_u16::<LE>()?;
    let magic = r.read_u16::<LE>()?;
    if typ != 2 && magic != 0x800 {
        bail!("invalid strings.bin file")
    }
    let count = r.read_u32::<LE>()?;
    (0..count)
        .map(|_| -> Result<_> {
            let tag = read_string(&mut r)?.to_lowercase();
            let value = read_string(&mut r)?;
            Ok((tag, value))
        })
        .collect()
}

fn read_string(r: &mut impl Read) -> Result<String> {
    let len = r.read_u16::<LE>()?;
    let mut buf = vec![0; 2 * len as usize];
    r.read_exact(&mut buf)?;
    Ok(String::from_utf16le_lossy(&buf))
}
