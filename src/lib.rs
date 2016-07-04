extern crate libc;

#[cfg(target_os = "macos")]
extern crate cocoa;

#[cfg(target_os = "windows")]
extern crate winapi;

#[cfg(target_os = "windows")]
extern crate user32;

#[cfg(target_os = "windows")]
extern crate kernel32;

#[cfg(target_os = "linux")]
extern crate x11_dl;

#[cfg(target_os = "macos")]
mod mac_clipboard;

#[cfg(target_os = "windows")]
mod windows_clipboard;

#[cfg(any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd", target_os = "openbsd"))]
mod unix_clipboard;

#[cfg(target_os = "macos")]
pub type NativeClipboard = mac_clipboard::CocoaClipboard;

#[cfg(target_os = "macos")]
pub use mac_clipboard::ClipboardExt;

#[cfg(target_os = "windows")]
pub type NativeClipboard = windows_clipboard::WindowsClipboard;

#[cfg(target_os = "windows")]
pub use windows_clipboard::ClipboardExt;

#[cfg(any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd", target_os = "openbsd"))]
pub type NativeClipboard = unix_clipboard::UnixClipboard;

#[cfg(any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd", target_os = "openbsd"))]
pub use unix_clipboard::ClipboardExt;

use std::error::Error;
use std::fmt;

pub trait Image {
    fn get_bytes(&self) -> &[u8];
    fn from_bytes(bytes: &[u8]) -> Result<Self, Box<Error + Send + Sync>> where Self: Sized;
}

pub enum Item<'a> {
    Text(&'a str),
    Image(&'a Image),
    Other(*mut libc::c_void)
}

pub trait Clipboard {
    type CreateError;
    type CopyError;
    type PasteError;

    fn get() -> Result<Self, Self::CreateError> where Self: Sized;
    fn copy(&mut self, item: Item) -> Result<(), Self::CopyError>;
    fn get_paste_text(&self) -> Result<&str, Self::PasteError>;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StringError(String);

impl<'a> From<&'a str> for StringError {
    fn from(s: &'a str) -> Self {
        StringError(s.into())
    }
}

impl fmt::Display for StringError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(&self.0)
    }
}

impl Error for StringError {
    fn description(&self) -> &str {
        &self.0
    }
}

#[derive(Debug)]
pub enum NoError {}

impl fmt::Display for NoError {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        unreachable!()
    }
}

impl Error for NoError {
    fn description(&self) -> &str {
        unreachable!()
    }
}


#[cfg(all(test, any(target_os = "macos", target_os = "windows", target_os = "linux", target_os = "dragonfly",
                    target_os = "freebsd", target_os = "openbsd")))]
mod tests {
    use ::{Clipboard, Item, NativeClipboard};
    #[test]
    fn test_native_clipboard() {
        const TEST_TEXT: &'static str = "BoomShakalaka";
        let mut clipboard = NativeClipboard::get().unwrap();

        // Save the current clipboard text
        let current_clipboard_text = clipboard.get_paste_text().unwrap().to_string();

        clipboard.copy(Item::Text(TEST_TEXT)).unwrap();
        assert_eq!(clipboard.get_paste_text().unwrap(), TEST_TEXT);

        // And restore the clipboard to its previous state after the test
        clipboard.copy(Item::Text(&current_clipboard_text)).unwrap();
    }
}
