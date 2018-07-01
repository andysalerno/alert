#[cfg(windows)]
extern crate winapi;

#[cfg(windows)]
extern crate kernel32;

#[cfg(windows)]
extern crate user32;

use std::ptr;

fn main() {
    notify();
}

#[cfg(unix)]
fn notify() {
    println!("todo");
}

#[cfg(macos)]
fn notify() {
    println!("todo");
}

#[cfg(windows)]
fn notify() {
    // https://docs.microsoft.com/en-us/windows/desktop/api/winuser/ns-winuser-flashwinfo
    const FLASH_COUNT: u32 = 3;
    const DEFAULT_TIMEOUT: u32 = 0;
    const FLASH_UNTIL_FOCUSED: u32 = 0xC;

    use winapi::winuser::FLASHWINFO;

    let window = unsafe { kernel32::GetConsoleWindow() };

    if window == ptr::null_mut() {
        return;
    }

    let mut flash_info = FLASHWINFO {
        hwnd: window,
        cbSize: std::mem::size_of::<FLASHWINFO>() as u32,
        dwFlags: FLASH_UNTIL_FOCUSED,
        dwTimeout: DEFAULT_TIMEOUT,
        uCount: FLASH_COUNT,
    };

    let flash_info_ptr = &mut flash_info as *mut FLASHWINFO;

    unsafe {
        user32::FlashWindowEx(flash_info_ptr);
    }
}
