
use std::env;
use std::str;

extern crate libc;
extern crate pty;
extern crate pty_shell;

use pty_shell::{winsize, PtyShell, PtyHandler};

mod espeak;
use espeak::ESpeak;

struct ScreenReader {
    speak: ESpeak,
}
impl PtyHandler for ScreenReader {
    fn input(&mut self, input: &[u8]) {
        let _ = self.speak.say(input);
    }

    fn output(&mut self, output: &[u8]) {
//        let _ = self.speak.say(output);
        let string = String::from_utf8_lossy(output).into_owned();
    }

    fn resize(&mut self, winsize: &winsize::Winsize) {
        let _ = self.speak.say("sr resize".as_bytes());
    }

    fn shutdown(&mut self) {
        let _ = self.speak.say("sr SHELL shutdown".as_bytes());
    }
}

fn main() {
    let speak = ESpeak::new().expect("Unable to initialize espeak");
    let shell = env::var("SHELL").expect("Cannot determine SHELL");

    speak.say("sr starting".as_bytes()).unwrap();

    let sr = ScreenReader {
        speak: speak,
    };

    let child = pty::fork().unwrap();
    child.exec("dmesg").unwrap();
    child.proxy(sr).unwrap();
    child.wait().unwrap();
}

