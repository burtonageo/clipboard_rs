use x11_dl::Atom;
use {Clipboard, Item, Result};

pub trait ClipboardExt: Clipboard { }

pub struct UnixClipboard;

impl Clipboard for UnixClipboard {
    fn get() -> Result<Self> where Self: Sized {
        unimplemented!();
    }

    fn copy(&mut self, item: &Item) -> Result<()> {
        unimplemented!();
    }

    fn copy_items(&mut self, items: &[Item]) -> Result<()> {
        unimplemented!();
    }

    fn get_paste_text(&self) -> Result<&str> {
        unimplemented!();
    }

    fn get_items(&self) -> &[Item] {
        unimplemented!();
    }
}

impl ClipboardExt for UnixClipboard { }
