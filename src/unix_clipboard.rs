extern crate x11_dl;

use self::x11_dl::Atom;
use {Clipboard as SuperClipboard, Item, Result};

pub struct Clipboard;

impl Clipboard {
    pub fn get() -> Result<Self>
        where Self: Sized
    {
        unimplemented!();
    }

    pub fn copy(&mut self, item: &Item) -> Result<()> {
        unimplemented!();
    }

    pub fn copy_items(&mut self, items: &[Item]) -> Result<()> {
        unimplemented!();
    }

    pub fn get_paste_text(&self) -> Result<&str> {
        unimplemented!();
    }

    pub fn get_items(&self) -> &[Item] {
        unimplemented!();
    }
}

pub trait ClipboardExt: Clipboard {}

impl ClipboardExt for SuperClipboard {}
