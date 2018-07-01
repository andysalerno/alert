#[cfg(windows)]
extern crate winapi;

#[cfg(windows)]
extern crate kernel32;

#[cfg(windows)]
extern crate user32;

use std::ptr;

fn main() {
    println!("Hello, world!");
    std::thread::sleep(std::time::Duration::from_secs(3));
    flash_windows_cmd_prompt();
    println!("done.");
}

#[cfg(windows)]
fn flash_windows_cmd_prompt() {
    use winapi::winuser::FLASHWINFO;

    let window = unsafe { kernel32::GetConsoleWindow() };

    if window == ptr::null_mut() {
        panic!("couldn't find windows console window");
    }

    let mut x = FLASHWINFO {
        hwnd: window,
        cbSize: std::mem::size_of::<FLASHWINFO>() as u32,

        dwFlags: 0xC,

        // rate at which to flash window, in ms
        // if 0, uses the OS default
        dwTimeout: 0,

        // count of flashes to perform
        uCount: 3,
    };

    let x_ptr = &mut x as *mut FLASHWINFO;

    unsafe {
        user32::FlashWindowEx(x_ptr);
    }
}
