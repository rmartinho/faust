use std::fmt::Debug;

use crate::args::{Args, GenerateArgs};

pub fn set_panic_hook() {}

pub fn finish<E: Debug>(res: Result<(), E>) -> Result<(), E> {
    res
}

pub fn prepare_generation_arguments(args: Args) -> GenerateArgs {
    args.generate
}
