use cocoa::appkit::{NSPasteboard, NSPasteboardTypeString};
use cocoa::base::{id, nil};
use cocoa::foundation::{NSArray, NSString};
use std::ffi::CStr;
use {Clipboard, Item, StringError};

pub trait ClipboardExt {
    fn clipboard_with_name(&self, name: &str) -> Self;

    fn get_clipboard_text_as_nsstring(&self) -> id;
    fn get_raw_clipboard(&self) -> id;
}

#[derive(Debug)]
pub struct CocoaClipboard(id);

impl Clipboard for CocoaClipboard {
    type CreateError = StringError;
    type CopyError = StringError;
    type PasteError = StringError;

    fn get() -> Result<Self, Self::CreateError> where Self: Sized {
        let pboard = unsafe { NSPasteboard::generalPasteboard(nil) };
        if pboard.is_null() {
            Err(StringError("could not get pasteboard".into()))
        } else {
            Ok(CocoaClipboard(pboard))
        }
    }

    fn copy(&mut self, item: Item) -> Result<(), Self::CopyError> {
        unsafe {
            let item = match item {
                Item::Text(ref text) => NSString::alloc(nil).init_str(text),
                Item::Image(ref _image) => {
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

    fn get_paste_text(&self)  -> Result<&str, Self::PasteError> {
        unsafe {
            let text = NSPasteboard::stringForType(self.0, NSPasteboardTypeString);
            Ok(CStr::from_ptr(text.UTF8String()).to_str().unwrap_or(""))
        }
    }
}
