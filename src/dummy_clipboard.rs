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

pub trait ClipboardExt: Clipboard {}

pub struct DummyClipboard {
    _priv: ()
}

impl Clipboard for DummyClipboard {
    fn get() -> Result<Self> {
        debug_println!("Calling DummyClipboard::get()");
        Ok(DummyClipboard {
            _priv: ()
        })
    }

    fn copy(&mut self, item: &Item) -> Result<()> {
        debug_println!("calling DummyClipboard::copy(): this operation is not available");
        Ok(())
    }

    fn copy_items(&mut self, items: &[Item]) -> Result<()> {
        debug_println!("calling DummyClipboard::copy_items(): this operation is not available");
        Ok(())
    }

    fn get_paste_text(&self) -> Result<&str> {
        debug_println!("calling DummyClipboard::paste(): this operation is not available");
        Ok("")
    }

    fn get_items(&self) -> &[Item] {
        debug_println!("calling DummyClipboard::get_items(): this operation is not available");
        Ok([])
    }
}

impl ClipboardExt for DummyClipboard { }
