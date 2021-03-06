extern crate winapi;
extern crate user32;
extern crate kernel32;

use std::ffi::CStr;
use std::ptr;
use self::kernel32::{GlobalAlloc, GlobalLock, GlobalUnlock};
use self::user32::{CloseClipboard, EmptyClipboard, GetClipboardData, OpenClipboard, SetClipboardData};
use self::winapi::minwindef::{FALSE, HGLOBAL, UINT};
use {Clipboard as SuperClipboard, Item, Result};

bitflags! {
    flags GlobalAllocFlags: UINT {
        const GHND = GMEM_MOVEABLE | GMEM_ZEROINIT,
        const GMEM_FIXED = 0x0000,
        const GMEM_MOVEABLE = 0x0002,
        const GMEM_ZEROINIT = 0x0040,
        const GPTR = GMEM_FIXED | GMEM_ZEROINIT
    }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum ClipboardFormats {
    CF_BITMAP = 0x0002,
    CF_DIB = 0x0008,
    CF_DIBV5 = 0x0011,
    CF_DIF = 0x0005,
    CF_DSPBITMAP = 0x0082,
    CF_DSPENHMETAFILE = 0x008E,
    CF_DSPMETAFILEPICT = 0x0083,
    CF_DSPTEXT = 0x0081,
    CF_ENHMETAFILE = 0x0E,
    CF_GDIOBJFIRST = 0x0300,
    CF_GDIOBJLAST = 0x03FF,
    CF_HDROP = 0x000F,
    CF_LOCALE = 0x0010,
    CF_METAFILEPICT = 0x0003,
    CF_OEMTEXT = 0x0007,
    CF_OWNERDISPLAY = 0x0080,
    CF_PALETTE = 0x0009,
    CF_PENDATA = 0x000A,
    CF_PRIVATEFIRST = 0x0200,
    CF_PRIVATELAST = 0x02FF,
    CF_RIFF = 0x000B,
    CF_SYLK = 0x0004,
    CF_TEXT = 0x0001,
    CF_TIFF = 0x0006,
    CF_UNICODETEXT = 0x000C,
    CF_WAVE = 0x000D,
}

#[derive(Debug)]
pub struct Clipboard {
    _priv: (),
}

impl Clipboard {
    pub fn get() -> Result<Self, Box<Error + Send + Sync>> {
        Ok(WindowsClipboard { _priv: () })
    }

    pub fn copy(&mut self, item: &Item) -> Result<(), Box<Error + Send + Sync>> {
        unsafe {
            let text = match *item {
                Item::Text(ref t) => t,
                _ => return Ok(()),
            };
            let _guard = ClipboardGuard::default();

            let clip_buf = GlobalLockHandle::new(GlobalAlloc(GMEM_MOVEABLE, text.len()));
            ptr::copy_nonoverlapping(text.as_ptr(), clip_buf.get(), text.len());

            let empty_result = EmptyClipboard();
            assert_ne!(empty_result, FALSE);
            SetClipboardData(ClipboardFormats::CF_UNICODETEXT as UINT, clip_buf);
            Ok(())
        }
    }

    pub fn copy_items(&mut self, items: &[Item]) -> Result<()> {
        unimplemented!();
    }

    pub fn get_paste_text(&self) -> Result<&str, Box<Error + Send + Sync>> {
        use std::slice;
        unsafe {
            let _guard = ClipboardGuard::default();
            let clip_buf = GlobalLockGuard::new(GetClipboardData(CF_UNICODETEXT));

            Ok(CStr::from_ptr(clip_buf.get()).to_str().unwrap_or(""))
        }
    }

    pub fn get_items(&self) -> &[Item] {
        unimplemented!();
    }
}

pub trait ClipboardExt {}

impl ClipboardExt for SuperClipboard {}

struct ClipboardGuard;

impl Default for ClipboardGuard {
    fn default() -> Self {
        let result = OpenClipboard(ptr::null_mut());
        assert_ne!(result, FALSE);
        ClipboardGuard
    }
}

impl Drop for ClipboardGuard {
    fn drop(&mut self) {
        CloseClipboard();
    }
}

struct GlobalLockGuard {
    data: HANDLE,
    ptr: *mut u8,
}

impl GlobalLockGuard {
    fn new(data: HANDLE) -> Self {
        assert_ne!(data, ptr::null_mut());
        let ptr = GlobalLock(data) as *mut u8;
        assert_ne!(ptr, ptr::null_mut());
        GlobalLockGuard {
            data: data,
            ptr: ptr,
        }
    }

    fn get(&self) -> *mut u8 {
        self.ptr
    }
}

impl Drop for GlobalLockGuard {
    fn drop(&mut self) {
        GlobalUnlock(self.data);
        self.ptr = ptr::null_mut();
    }
}
