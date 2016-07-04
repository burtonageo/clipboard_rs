use x11_dl::Atom;
use {Clipboard, ClipboardCopy};

pub struct UnixClipboard;

impl Default for UnixClipboard {
    fn default() -> Self {
        unimplemented!();
    }
}

impl Clipboard for UnixClipboard {
    fn copy(&mut self, item: Item) {
        unimplemented!();
    }
    
    fn get_paste_text(&self) -> &str {
        unimplemented!();
    }
}