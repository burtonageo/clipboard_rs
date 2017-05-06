use {Clipboard, Item, Result};

#[cfg(Debug)]
macro_rules! debug_println {
    ($msg:expr) => {
        println!($msg)
    },
    ($msg:expr, $rest:tt) => {
        println!($msg, $rest)
    }
}

#[cfg(not(Debug))]
macro_rules! debug_println {
    ($msg:expr) => { },
    ($msg:expr, $rest:tt) => { }
}

pub struct Clipboard {
    _priv: (),
}

impl Clipboard {
    #[inline]
    pub fn get() -> Result<Self> {
        debug_println!("Calling DummyClipboard::get()");
        Ok(DummyClipboard { _priv: () })
    }

    #[inline]
    pub fn copy(&mut self, item: &Item) -> Result<()> {
        debug_println!("calling DummyClipboard::copy(): this operation is not available");
        Ok(())
    }

    #[inline]
    pub fn copy_items(&mut self, items: &[Item]) -> Result<()> {
        debug_println!("calling DummyClipboard::copy_items(): this operation is not available");
        Ok(())
    }

    #[inline]
    pub fn get_paste_text(&self) -> Result<&str> {
        debug_println!("calling DummyClipboard::paste(): this operation is not available");
        Ok("")
    }

    #[inline]
    pub fn get_items(&self) -> &[Item] {
        debug_println!("calling DummyClipboard::get_items(): this operation is not available");
        Ok([])
    }
}

pub trait ClipboardExt {}

impl ClipboardExt for Clipboard {}

impl ClipboardExt for DummyClipboard {}
