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

#[cfg(target_os = "windows")]
pub type NativeClipboard = windows_clipboard::WindowsClipboard;

#[cfg(any(target_os = "linux", target_os = "dragonfly", target_os = "freebsd", target_os = "openbsd"))]
pub type NativeClipboard = unix_clipboard::UnixClipboard;

use std::error::Error;

pub trait Image {
    fn get_bytes(&self) -> &[u8];
    fn from_bytes(bytes: &[u8]) -> Result<Self, Box<Error + Send + Sync>> where Self: Sized;
}

pub enum ClipboardCopy<'a> {
    Text(&'a str),
    Image(&'a Image),
    Other(*mut libc::c_void)
}

pub trait Clipboard {
    fn copy(&mut self, item: ClipboardCopy);
    fn get_paste_text(&self) -> &str;
}

#[cfg(all(test, any(target_os = "macos", target_os = "windows", target_os = "linux", target_os = "dragonfly",
                    target_os = "freebsd", target_os = "openbsd")))]
mod tests {
    use ::{Clipboard, ClipboardCopy, NativeClipboard};
    #[test]
    fn test_native_clipboard() {
        const TEST_TEXT: &'static str = "BoomShakalaka";
        let mut clipboard = NativeClipboard::default();

        // Save the current clipboard text
        let current_clipboard_text = clipboard.get_paste_text().to_string();

        clipboard.copy(ClipboardCopy::Text(TEST_TEXT));
        assert_eq!(clipboard.get_paste_text(), TEST_TEXT);

        // And restore the clipboard to its previous state after the test
        clipboard.copy(ClipboardCopy::Text(&current_clipboard_text));
    }
}
