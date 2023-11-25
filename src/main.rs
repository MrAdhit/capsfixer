use std::fs;
use std::thread;
use std::time::Duration;

use mki::Keyboard;

fn main() {
    println!("If this program doesn't work, try running it as root");

    thread::spawn(move || {
        let mut d_s = false;
        let mut b_ps = false;
        loop {
            let c_ps = Keyboard::CapsLock.is_pressed();
            if c_ps {
                if !b_ps {
                    d_s = !d_s;
                }
            }

            if !d_s && c_ps && !b_ps {
                fs::write("/sys/class/leds/input7::capslock/brightness", b"0").unwrap();
                Keyboard::CapsLock.click();
            }

            b_ps = c_ps;
        }
    });

    loop { thread::sleep(Duration::from_secs(99999)) }
}
