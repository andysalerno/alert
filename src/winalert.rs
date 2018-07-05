#[cfg(windows)]
extern crate winapi;

#[cfg(windows)]
extern crate kernel32;

#[cfg(windows)]
extern crate user32;

#[cfg(windows)]
use std::ptr;

use self::winapi::HWND;

pub fn alert() {
    let console_window_result = get_console_window();

    if let Ok(console_window) = console_window_result {
        flash_window(console_window);
    } else {
        panic!("Unable to find console window.");
    }
}

fn get_console_window() -> Result<HWND, ()> {
    let window = unsafe { kernel32::GetConsoleWindow() };

    if window == ptr::null_mut() {
        return Err(());
    }

    Ok(window)
}

fn flash_window(window: HWND) {
    use self::winapi::winuser::FLASHWINFO;

    // https://docs.microsoft.com/en-us/windows/desktop/api/winuser/ns-winuser-flashwinfo
    const FLASH_COUNT: u32 = 3;
    const DEFAULT_TIMEOUT: u32 = 0;
    const FLASH_UNTIL_FOCUSED: u32 = 0xC;

    let mut flash_info = FLASHWINFO {
        hwnd: window,
        cbSize: ::std::mem::size_of::<FLASHWINFO>() as u32,
        dwFlags: FLASH_UNTIL_FOCUSED,
        dwTimeout: DEFAULT_TIMEOUT,
        uCount: FLASH_COUNT,
    };

    let flash_info_ptr = &mut flash_info as *mut FLASHWINFO;

    unsafe {
        user32::FlashWindowEx(flash_info_ptr);
    }
}
