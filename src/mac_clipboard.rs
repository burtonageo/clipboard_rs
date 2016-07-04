use cocoa::appkit::{NSPasteboard, NSPasteboardTypeString};
use cocoa::base::{id, nil};
use cocoa::foundation::{NSArray, NSString};
use std::ffi::CStr;
use {Clipboard, ClipboardCopy};

pub trait ClipboardExt {
    fn clipboard_with_name(&self, name: &str) -> Self;

    fn get_clipboard_text_as_nsstring(&self) -> id;
    fn get_raw_clipboard(&self) -> id;
}

pub struct CocoaClipboard(id);

impl Default for CocoaClipboard {
    fn default() -> Self {
        let pboard = unsafe { NSPasteboard::generalPasteboard(nil) };
        assert!(pboard != nil);
        CocoaClipboard(pboard)
    }
}

impl Clipboard for CocoaClipboard {
    fn copy(&mut self, item: ClipboardCopy) {
        unsafe {
            let item = match item {
                ClipboardCopy::Text(ref text) => NSString::alloc(nil).init_str(text),
                ClipboardCopy::Image(ref _image) => {
                    unimplemented!();
                }
                _ => return
            };

            self.0.clearContents();
            self.0.declareTypes_owner(NSArray::arrayWithObject(nil, NSPasteboardTypeString), nil);
            NSPasteboard::setString_forType(self.0, item, NSPasteboardTypeString);
        }
    }

    fn get_paste_text(&self) -> &str {
        unsafe {
            let text = NSPasteboard::stringForType(self.0, NSPasteboardTypeString);
            CStr::from_ptr(text.UTF8String()).to_str().unwrap_or("")
        }
    }
}