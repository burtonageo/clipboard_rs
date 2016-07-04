use x11_dl::Atom;
use {Clipboard, Item, NoError};

pub trait ClipboardExt { }

pub struct UnixClipboard;

impl Clipboard for UnixClipboard {
    type CreateError = NoError;
    type CopyError = NoError;
    type PasteError = NoError;

    fn get() -> Result<Self, Self::CreateError> where Self: Sized {
        unimplemented!();
    }

    fn copy(&mut self, item: Item) -> Result<(), Self::CopyError> {
        unimplemented!();
    }

    fn get_paste_text(&self) -> Result<&str, Self::PasteError> {
        unimplemented!();
    }
}