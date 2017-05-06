#[cfg(any(windows, unix))]
extern crate libc;

#[cfg(target_os = "macos")]
mod mac_clipboard;

#[cfg(target_os = "macos")]
use mac_clipboard as inner;

#[cfg(windows)]
mod windows_clipboard;

#[cfg(windows)]
use windows_clipboard as inner;

#[cfg(all(not(target_os = "macos"), unix))]
mod unix_clipboard;

#[cfg(all(not(target_os = "macos"), unix))]
use unix_clipboard as inner;

#[cfg(not(any(windows, unix, target_os = "macos")))]
mod dummy_clipboard;

#[cfg(not(any(windows, unix, target_os = "macos")))]
use dummy_clipboard as inner;

pub use inner::ClipboardExt;

use std::borrow::Cow;
use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub struct Clipboard(inner::Clipboard);

impl Clipboard {
    #[inline]
    pub fn get() -> Result<Self> {
        inner::Clipboard::get().map(|cb| Clipboard(cb))
    }

    #[inline]
    pub fn copy(&mut self, item: &Item) -> Result<()> {
        self.0.copy(item)
    }

    #[inline]
    pub fn copy_items(&mut self, items: &[Item]) -> Result<()> {
        self.0.copy_items(items)
    }

    #[inline]
    pub fn get_paste_text(&self) -> Result<&str> {
        self.0.get_paste_text()
    }

    #[inline]
    pub fn get_items(&self) -> Cow<[Item]> {
        self.0.get_items()
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ImageFormat {
    Jpg,
    Png,
}

pub trait Image {
    fn bytes(&self) -> &[u8];
    fn format(&self) -> ImageFormat;
    fn from_bytes(bytes: &[u8]) -> Result<Self> where Self: Sized;
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum SoundFormat {
    Wav,
    Aiff,
    Alac,
    Flac,
    Mp3,
    OggVorbis,
}

pub trait Sound {
    fn bytes(&self) -> &[u8];
    fn format(&self) -> SoundFormat;
    fn from_bytes(bytes: &[u8]) -> Result<Self> where Self: Sized;
}

#[derive(Clone, Copy)]
pub enum Item<'a> {
    Text(&'a str),
    Image(&'a Image),
    Sound(&'a Sound),
    Other(*mut libc::c_void),
}

pub type Result<T> = ::std::result::Result<T, inner::Error>;

#[cfg(all(test, any(unix, windows)))]
mod tests {
    use {Clipboard, Item};
    #[test]
    fn test_native_clipboard() {
        const TEST_TEXT: &'static str = "BoomShakalaka";
        let mut clipboard = Clipboard::get().unwrap();

        // Save the current clipboard text
        let current_clipboard_text = clipboard.get_paste_text().unwrap().to_string();

        clipboard.copy(&Item::Text(TEST_TEXT)).unwrap();
        assert_eq!(clipboard.get_paste_text().unwrap(), TEST_TEXT);

        // And restore the clipboard to its previous state after the test
        clipboard
            .copy(&Item::Text(&current_clipboard_text))
            .unwrap();
    }
}
