#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use self::windows::*;

#[cfg(not(windows))]
mod nix;
#[cfg(not(windows))]
pub use self::nix::*;