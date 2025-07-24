use std::fmt::Debug;

use crate::args::{Args, Command, GenerateArgs};

pub fn set_panic_hook() {}

pub fn finish<E: Debug>(res: Result<(), E>) -> Result<(), E> {
    res
}

pub fn prepare_generation_arguments(args: Args) -> GenerateArgs {
    match args.command {
        Some(Command::Generate(a)) => a,
        _ => args.generate,
    }
}
