use std::{env, fmt::Debug, mem::size_of};

use windows::{
    Win32::{
        Foundation::MAX_PATH,
        UI::{
            Controls::Dialogs::{GetOpenFileNameW, OPENFILENAMEW},
            WindowsAndMessaging::{MB_ICONERROR, MB_OK, MessageBoxW},
        },
    },
    core::w,
};
use windows_strings::{HSTRING, PWSTR};

use crate::args::{Args, Command, GenerateArgs};

pub fn set_panic_hook() {
    std::panic::set_hook(Box::new(move |info| {
        use std::fmt::Write as _;
        let mut message = String::from("An error has occurred.\n\n");
        if let Some(s) = info.payload_as_str() {
            let _ = write!(message, "{s:?}\n");
        } else {
            let _ = write!(message, "Panic occurred {info:?}.\n");
        };
        if let Some(loc) = info.location() {
            let _ = write!(message, "\t@ {loc}");
        }

        unsafe {
            MessageBoxW(
                None,
                &HSTRING::from(message),
                w!("Error"),
                MB_OK | MB_ICONERROR,
            )
        };
        dont_disappear::enter_to_continue::default();
    }));
}

pub fn finish<E: Debug>(res: Result<(), E>) -> Result<(), E> {
    if let Err(ref e) = res {
        eprintln!("{:?}", e);
    }
    dont_disappear::enter_to_continue::default();
    res
}

pub fn prepare_generation_arguments(args: Args) -> GenerateArgs {
    if has_args() {
        match args.command {
            Some(Command::Generate(a)) => a,
            _ => args.generate,
        }
    } else {
        let mut file = vec![0; MAX_PATH as _];
        let mut ofn = OPENFILENAMEW {
            lStructSize: size_of::<OPENFILENAMEW>() as _,
            lpstrFilter: w!("Manifest file\0faust.yml\0"),
            lpstrTitle: w!("Select a manifest file"),
            nMaxFile: file.len() as _,
            lpstrFile: PWSTR(file.as_mut_ptr()),
            ..Default::default()
        };
        let success: bool = unsafe { GetOpenFileNameW(&mut ofn) }.into();
        if !success {
            panic!("canceled open file dialog")
        }
        let n = file.iter().position(|c| *c == 0).expect("missing null terminator");
        file.truncate(n);
        let path = String::from_utf16(&file).expect("invalid file name").into();
        GenerateArgs {
            manifest: Some(path),
            out_dir: None,
            base_game_path: None,
            serve: true,
        }
    }
}

fn has_args() -> bool {
    env::args().len() > 1
}
