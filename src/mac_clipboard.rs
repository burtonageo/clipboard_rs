use cocoa::appkit::{NSPasteboard, NSPasteboardTypeString};
use cocoa::base::{id, nil};
use cocoa::foundation::{NSArray, NSData, NSString, NSUInteger};
use std::borrow::Cow;
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

    fn copy(&mut self, item: &Item) -> Result<()> {
        unsafe {
            let item = item.native_representation();
            if item.is_null() {
                return Ok(())
            }

            self.0.clearContents();
            self.0.declareTypes_owner(NSArray::arrayWithObject(nil, NSPasteboardTypeString), nil);
            NSPasteboard::setString_forType(self.0, item, NSPasteboardTypeString);
            Ok(())
        }
    }

    fn copy_items(&mut self, items: &[Item]) -> Result<()> {
        let items = items.iter().map(|i| i.native_representation()).collect::<Cow<[_]>>();
        let array = unsafe { NSArray::arrayWithObjects(nil, &items) };
        unsafe {
            self.0.clearContents();
            self.0.writeObjects(array);
        }
        Ok(())
    }

    fn get_paste_text(&self)  -> Result<&str> {
        unsafe {
            let text = NSPasteboard::stringForType(self.0, NSPasteboardTypeString);
            Ok(CStr::from_ptr(text.UTF8String()).to_str().unwrap_or(""))
        }
    }
}

impl ClipboardExt for CocoaClipboard {
    fn clipboard_with_name(&self, name: &str) -> Self {
        unimplemented!();
    }

    fn get_clipboard_text_as_nsstring(&self) -> id {
        unimplemented!();
    }

    fn get_raw_clipboard(&self) -> id {
        unimplemented!();
    }
}

impl<'a> Item<'a> {
    fn native_representation(&self) -> id {
        match *self {
            Item::Text(text) => unsafe { NSString::alloc(nil).init_str(text) },
            Item::Image(image) => unsafe {
                let slice = image.bytes();
                let _data = NSData::dataWithBytes_length_(nil,
                                                          slice.as_ptr() as *const _,
                                                          slice.len() as NSUInteger);
                unimplemented!();
            },
            Item::Sound(sound) => {
                unimplemented!();
            }
            _ => nil
        }
    }
}
