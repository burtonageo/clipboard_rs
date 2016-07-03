use std::ffi::CStr;
use std::ptr;
use kernel32::{GlobalAlloc, GlobalLock, GlobalUnlock};
use user32::{CloseClipboard, EmptyClipboard, GetClipboardData, OpenClipboard, SetClipboardData};
use winapi::minwindef::{FALSE, HGLOBAL};

const GMEM_MOVEABLE: usize = 0x0002;
const CF_UNICODETEXT: usize = 0x000C;

pub struct WindowsClipboard {
    _priv: ()
}

impl Default for WindowsClipboard {
    fn default() -> Self {
        WindowsClipboard {
            _priv: ()
        }
    }
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

impl ::Clipboard for WindowsClipboard {
    fn copy(&mut self, text: &str) {
        unsafe {
            let _guard = ClipboardGuard::default();

            let clip_buf = GlobalLockHandle::new(GlobalAlloc(GMEM_MOVEABLE, text.len()));
            ptr::copy_nonoverlapping(text.as_ptr(), clip_buf.get(), text.len());

            let empty_result = EmptyClipboard();
            assert!(empty_result != FALSE);
            SetClipboardData(CF_UNICODETEXT, clip_buf);
        }
    }

    fn get_paste_text(&self) -> &str {
        use std::slice;
        unsafe {
            let _guard = ClipboardGuard::default();
            let clip_buf = GlobalLockGuard::new(GetClipboardData(CF_UNICODETEXT));

            CStr::from_ptr(clip_buf.get()).to_str().unwrap_or("")
        }
    }
}