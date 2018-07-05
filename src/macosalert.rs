#[cfg(target_os = "macos")]
use std::process::Command;

#[cfg(target_os = "macos")]
pub fn alert() {
    // MacOS by default will 'bounce' the terminal
    // whenever the BEL is triggered
    // https://en.wikipedia.org/wiki/Bell_character
    // https://superuser.com/questions/246353/how-to-make-terminal-bounce-its-application-dock-icon-in-os-x
    Command::new("tput")
        .arg("bel")
        .spawn()
        .expect("unable to launch executable 'tput'");
}
