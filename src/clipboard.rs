pub fn set_contents(text: &str) {
    #[cfg(target_os = "windows")]
    win32::set_text(text);

    #[cfg(not(target_os = "windows"))]
    {
        use std::process::{Command, Stdio};
        use std::io::Write;
        if let Ok(mut child) = Command::new("wl-copy")
            .stdin(Stdio::piped())
            .spawn()
        {
            if let Some(mut stdin) = child.stdin.take() {
                let _ = stdin.write_all(text.as_bytes());
            }
            let _ = child.wait();
        } else if let Ok(mut child) = Command::new("xclip")
            .args(&["-selection", "clipboard"])
            .stdin(Stdio::piped())
            .spawn()
        {
            if let Some(mut stdin) = child.stdin.take() {
                let _ = stdin.write_all(text.as_bytes());
            }
            let _ = child.wait();
        }
    }
}

pub fn get_contents() -> String {
    #[cfg(target_os = "windows")]
    return win32::get_text();

    #[cfg(not(target_os = "windows"))]
    {
        use std::process::Command;
        if let Ok(output) = Command::new("wl-paste").arg("--no-newline").output() {
            return String::from_utf8_lossy(&output.stdout).to_string();
        } else if let Ok(output) = Command::new("xclip")
            .args(&["-selection", "clipboard", "-o"])
            .output()
        {
            return String::from_utf8_lossy(&output.stdout).to_string();
        }
        String::new()
    }
}

#[cfg(target_os = "windows")]
mod win32 {
    use std::ffi::c_void;
    use std::ptr::null_mut;

    const GMEM_MOVEABLE: u32 = 0x0002;
    const CF_UNICODETEXT: u32 = 13;

    #[link(name = "kernel32")]
    extern "system" {
        fn GlobalAlloc(uFlags: u32, dwBytes: usize) -> *mut c_void;
        fn GlobalFree(hMem: *mut c_void) -> *mut c_void;
        fn GlobalLock(hMem: *mut c_void) -> *mut c_void;
        fn GlobalUnlock(hMem: *mut c_void) -> i32;
        fn RtlCopyMemory(Destination: *mut c_void, Source: *const c_void, Length: usize);
    }

    #[link(name = "user32")]
    extern "system" {
        fn OpenClipboard(hWndNewOwner: *mut c_void) -> i32;
        fn CloseClipboard() -> i32;
        fn EmptyClipboard() -> i32;
        fn SetClipboardData(uFormat: u32, hMem: *mut c_void) -> *mut c_void;
        fn GetClipboardData(uFormat: u32) -> *mut c_void;
    }

    pub fn set_text(text: &str) {
        let utf16: Vec<u16> = text.encode_utf16().chain(Some(0)).collect();
        let bytes_len = utf16.len() * 2;
        unsafe {
            if OpenClipboard(null_mut()) != 0 {
                EmptyClipboard();
                let h_mem = GlobalAlloc(GMEM_MOVEABLE, bytes_len);
                if !h_mem.is_null() {
                    let ptr = GlobalLock(h_mem);
                    if !ptr.is_null() {
                        RtlCopyMemory(ptr, utf16.as_ptr() as *const c_void, bytes_len);
                        GlobalUnlock(h_mem);
                        SetClipboardData(CF_UNICODETEXT, h_mem);
                    } else {
                        GlobalFree(h_mem);
                    }
                }
                CloseClipboard();
            }
        }
    }

    pub fn get_text() -> String {
        let mut result = String::new();
        unsafe {
            if OpenClipboard(null_mut()) != 0 {
                let h_mem = GetClipboardData(CF_UNICODETEXT);
                if !h_mem.is_null() {
                    let ptr = GlobalLock(h_mem) as *const u16;
                    if !ptr.is_null() {
                        let mut len = 0;
                        while *ptr.add(len) != 0 {
                            len += 1;
                        }
                        let slice = std::slice::from_raw_parts(ptr, len);
                        result = String::from_utf16_lossy(slice);
                        GlobalUnlock(h_mem);
                    }
                }
                CloseClipboard();
            }
        }
        result
    }
}