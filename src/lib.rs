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

#[cfg(not(any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd", target_os = "openbsd",
              target_os = "windows", target_os = "macos")))]
mod dummy_clipboard;

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

#[cfg(not(any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd", target_os = "openbsd",
              target_os = "windows", target_os = "macos")))]
pub type NativeClipboard = dummy_clipboard::DummyClipboard;

#[cfg(not(any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd", target_os = "openbsd",
              target_os = "windows", target_os = "macos")))]
pub use dummy_clipboard::ClipboardExt;

use std::error::Error;

pub trait Image {
    fn bytes(&self) -> &[u8];
    fn from_bytes(bytes: &[u8]) -> Result<Self> where Self: Sized;
}

pub trait Sound {
    fn bytes(&self) -> &[u8];
    fn is_wav(&self) -> bool;
    fn from_bytes(bytes: &[u8]) -> Result<Self> where Self: Sized;
}

pub enum Item<'a> {
    Text(&'a str),
    Image(&'a Image),
    Sound(&'a Sound),
    Other(*mut libc::c_void)
}

pub trait Clipboard {
    fn get() -> Result<Self> where Self: Sized;
    fn copy(&mut self, item: Item) -> Result<()>;
    fn copy_items(&mut self, items: Vec<Item>) -> Result<()>;
    fn get_paste_text(&self) -> Result<&str>;
}

pub type Result<T> = ::std::result::Result<T, Box<Error + Send + Sync>>;

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
