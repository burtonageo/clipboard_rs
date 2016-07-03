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

pub trait Clipboard {
    fn copy(&mut self, text: &str);
    fn get_paste_text(&self) -> &str;
}

#[cfg(all(test, any(target_os = "macos", target_os = "windows", target_os = "linux", target_os = "dragonfly",
                    target_os = "freebsd", target_os = "openbsd")))]
mod tests {
    use super::NativeClipboard;
    use ::Clipboard;
    #[test]
    fn test_native_clipboard() {
        const TEST_TEXT: &'static str = "BoomShakalaka";
        let mut clipboard = NativeClipboard::default();

        // Save the current clipboard text
        let current_clipboard_text = clipboard.get_paste_text().to_string();

        clipboard.copy(TEST_TEXT);
        assert_eq!(clipboard.get_paste_text(), TEST_TEXT);

        // And restore the clipboard to its previous state after the test
        clipboard.copy(&current_clipboard_text);
    }
}
