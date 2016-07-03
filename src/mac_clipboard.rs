use cocoa::appkit::{NSPasteboard, NSPasteboardTypeString};
use cocoa::base::{id, nil};
use cocoa::foundation::{NSArray, NSString};
use std::ffi::CStr;

pub struct CocoaClipboard(id);

impl Default for CocoaClipboard {
    fn default() -> Self {
        let pboard = unsafe { NSPasteboard::generalPasteboard(nil) };
        assert!(pboard != nil);
        CocoaClipboard(pboard)
    }
}

impl ::Clipboard for CocoaClipboard {
    fn copy(&mut self, text: &str) {
        unsafe {
            self.0.clearContents();
            let nsstr = NSString::alloc(nil).init_str(text);
            self.0.declareTypes_owner(NSArray::arrayWithObject(nil, NSPasteboardTypeString), nil);
            NSPasteboard::setString_forType(self.0, nsstr, NSPasteboardTypeString);
        }
    }

    fn get_paste_text(&self) -> &str {
        unsafe {
            let text = NSPasteboard::stringForType(self.0, NSPasteboardTypeString);
            CStr::from_ptr(text.UTF8String()).to_str().unwrap_or("")
        }
    }
}