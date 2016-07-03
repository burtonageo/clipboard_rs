use x11_dl::Atom;

pub struct UnixClipboard;

impl Default for UnixClipboard {
    fn default() -> Self {
        unimplemented!();
    }
}

impl ::Clipboard for UnixClipboard {
    fn copy(&mut self, text: &str) {
        unimplemented!();
    }
    
    fn get_paste_text(&self) -> &str {
        unimplemented!();
    }
}