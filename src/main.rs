#[cfg(target_os = "macos")]
mod macosalert;
use macosalert::alert;

#[cfg(windows)]
mod winalert;

fn main() {
    alert();
}
