use cocoa::appkit::{NSPasteboard, NSPasteboardTypeString};
use cocoa::base::{id, nil};
use cocoa::foundation::{NSArray, NSData, NSString, NSUInteger};
use std::ffi::CStr;
use {Clipboard, Item, Result};

pub trait ClipboardExt: Clipboard {
    fn clipboard_with_name(&self, name: &str) -> Self;

    fn get_clipboard_text_as_nsstring(&self) -> id;
    fn get_raw_clipboard(&self) -> id;
}

#[derive(Debug)]
pub struct CocoaClipboard(id);

impl Clipboard for CocoaClipboard {
    fn get() -> Result<Self> where Self: Sized {
        let pboard = unsafe { NSPasteboard::generalPasteboard(nil) };
        if pboard.is_null() {
            Err(From::from("could not get pasteboard"))
        } else {
            Ok(CocoaClipboard(pboard))
        }
    }

    fn copy(&mut self, item: Item) -> Result<()> {
        unsafe {
            let item = match item {
                Item::Text(ref text) => NSString::alloc(nil).init_str(text),
                Item::Image(ref image) => {
                    let slice = image.bytes();
                    let _data = NSData::dataWithBytes_length_(nil,
                                                              slice.as_ptr() as *const _,
                                                              slice.len() as NSUInteger);
                    unimplemented!();
                }
                _ => return Ok(())
            };

            self.0.clearContents();
            self.0.declareTypes_owner(NSArray::arrayWithObject(nil, NSPasteboardTypeString), nil);
            NSPasteboard::setString_forType(self.0, item, NSPasteboardTypeString);
            Ok(())
        }
    }

    fn get_paste_text(&self)  -> Result<&str> {
        unsafe {
            let text = NSPasteboard::stringForType(self.0, NSPasteboardTypeString);
            Ok(CStr::from_ptr(text.UTF8String()).to_str().unwrap_or(""))
        }
    }
}
