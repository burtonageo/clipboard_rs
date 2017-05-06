extern crate cocoa;

use {Clipboard as SuperClipboard, Item, Result};
use self::cocoa::appkit::{NSPasteboard, NSPasteboardTypeString};
use self::cocoa::base::{id, nil};
use self::cocoa::foundation::{NSArray, NSData, NSFastEnumeration, NSString, NSUInteger};
use std::borrow::Cow;
use std::ffi::CStr;

#[derive(Debug)]
pub struct Clipboard(id);

impl Clipboard {
    pub fn get() -> Result<Self> {
        let pboard = unsafe { NSPasteboard::generalPasteboard(nil) };
        if pboard.is_null() {
            Err(From::from("could not get pasteboard"))
        } else {
            Ok(Clipboard(pboard))
        }
    }

    pub fn copy(&mut self, item: &Item) -> Result<()> {
        self.copy_items(&[*item])
    }

    pub fn copy_items(&mut self, items: &[Item]) -> Result<()> {
        let items = items
            .iter()
            .map(|i| i.native_representation())
            .collect::<Cow<[_]>>();
        let array = unsafe { NSArray::arrayWithObjects(nil, &items) };
        unsafe {
            self.0.clearContents();
            self.0.writeObjects(array);
        }
        Ok(())
    }

    pub fn get_paste_text(&self) -> Result<&str> {
        unsafe {
            let text = NSPasteboard::stringForType(self.0, NSPasteboardTypeString);
            Ok(CStr::from_ptr(text.UTF8String()).to_str().unwrap_or(""))
        }
    }

    pub fn get_items(&self) -> Cow<[Item]> {
        unsafe {
            self.0
                .pasteboardItems()
                .iter()
                .map(|x| Item::Text(""))
                .collect()
        }
    }
}

pub trait ClipboardExt {
    fn clipboard_with_name(&self, name: &str) -> Self;
    fn get_items_nsarray(&self) -> id;
    fn get_raw_clipboard(&self) -> id;
}

impl ClipboardExt for SuperClipboard {
    fn clipboard_with_name(&self, name: &str) -> Self {
        unimplemented!();
    }

    fn get_items_nsarray(&self) -> id {
        unimplemented!();
    }

    #[inline]
    fn get_raw_clipboard(&self) -> id {
        (self.0).0
    }
}

#[derive(Debug)]
pub struct Error;

impl From<&'static str> for Error {
    #[inline]
    fn from(_: &'static str) -> Self {
        Error
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
            _ => nil,
        }
    }
}
