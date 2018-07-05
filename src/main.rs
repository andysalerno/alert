#[cfg(target_os = "macos")]
mod macosalert;
#[cfg(target_os = "macos")]
use macosalert::alert;

#[cfg(windows)]
mod winalert;

#[cfg(windows)]
use winalert::alert;

fn main() {
    alert();
}
