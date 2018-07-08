# Alert

Alert is a simple (really, really simple) tool written in Rust that does one thing:

It flashes the command prompt window it is executed in (on Windows), or hops the Terminal dock icon (on MacOS). (Linux version is in progress).

The idea is that if you have some long-running console task, you can just throw Alert at the end to get notified when it's done.

On Windows, this is accomplished by calling into the WinApi to get the currently running console window handle and invoking [`FlashWindowEx`](https://docs.microsoft.com/en-us/windows/desktop/api/winuser/nf-winuser-flashwindowex) on it.

On MacOS, Alert is simply a glorified shell script, because MacOS by default jumps the Terminal icon when it receives the [BEL character](https://en.wikipedia.org/wiki/Bell_character). The Alert executable invokes the command `tput bel` to do this.

Example on MacOS: {todo image}
![](/images/alert_example_macos.gif)
