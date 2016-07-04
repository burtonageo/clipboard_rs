use std::ffi::CStr;
use std::ptr;
use kernel32::{GlobalAlloc, GlobalLock, GlobalUnlock};
use user32::{CloseClipboard, EmptyClipboard, GetClipboardData, OpenClipboard, SetClipboardData};
use winapi::minwindef::{FALSE, HGLOBAL, UINT};
use {Clipboard, Item, Result};

bitflags! {
    pub flags GlobalAllocFlags: UINT {
        const GHND = GMEM_MOVEABLE | GMEM_ZEROINIT,
        const GMEM_FIXED = 0x0000,
        const GMEM_MOVEABLE = 0x0002,
        const GMEM_ZEROINIT = 0x0040,
        const GPTR = GMEM_FIXED | GMEM_ZEROINIT
    }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ClipboardFormats {
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
    CF_WAVE = 0x000D
}

pub trait ClipboardExt: Clipboard { }

#[derive(Debug)]
pub struct WindowsClipboard {
    _priv: ()
}

struct ClipboardGuard;
impl Default for ClipboardGuard {
    fn default() -> Self {
        let result = OpenClipboard(ptr::null_mut());
        assert!(result != FALSE);
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
    ptr: *mut u8
}

impl GlobalLockGuard {
    fn new(data: HANDLE) -> Self {
        assert!(data != ptr::null_mut());
        let ptr = GlobalLock(data) as *mut u8;
        assert!(ptr != ptr::null_mut());
        GlobalLockGuard {
            data: data,
            ptr: ptr
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

impl Clipboard for WindowsClipboard {
    fn get() -> Result<Self, Box<Error + Send + Sync>> {
        Ok(WindowsClipboard {
            _priv: ()
        })
    }

    fn copy(&mut self, item: Item) -> Result<(), Box<Error + Send + Sync>> {
        unsafe {
            let text = match item {
                Item::Text(ref t) => t,
                _ => return Ok(())
            };
            let _guard = ClipboardGuard::default();

            let clip_buf = GlobalLockHandle::new(GlobalAlloc(GMEM_MOVEABLE, text.len()));
            ptr::copy_nonoverlapping(text.as_ptr(), clip_buf.get(), text.len());

            let empty_result = EmptyClipboard();
            assert!(empty_result != FALSE);
            SetClipboardData(ClipboardFormats::CF_UNICODETEXT as UINT, clip_buf);
            Ok(())
        }
    }

    fn get_paste_text(&self) -> Result<&str, Box<Error + Send + Sync>> {
        use std::slice;
        unsafe {
            let _guard = ClipboardGuard::default();
            let clip_buf = GlobalLockGuard::new(GetClipboardData(CF_UNICODETEXT));

            Ok(CStr::from_ptr(clip_buf.get()).to_str().unwrap_or(""))
        }
    }
}