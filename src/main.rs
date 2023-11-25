use std::fs;
use std::thread;
use std::time::Duration;

use mki::Keyboard;

fn main() {
    println!("If this program doesn't work, try running it as root");

    let mut d_s = false;
    let mut b_ps = false;

    loop {
        let c_ps = Keyboard::CapsLock.is_pressed();
        if c_ps && !b_ps {
            d_s = !d_s;

            if !d_s {
                match fs::read_dir("/sys/class/leds") {
                    Ok(leds) => {
                        println!("Disabling capslock");

                        leds
                            .filter_map(|d| d.ok())
                            .map(|d| d.file_name().to_string_lossy().to_string())
                            .filter(|n| n.contains("capslock"))
                            .for_each(|led| fs::write(format!("/sys/class/leds/{}/brightness", led), b"0").unwrap());

                        Keyboard::CapsLock.click();
                    }
                    Err(err) => {
                        eprintln!("Something went wrong when querying leds: {:?}", err);
                    }
                }
            }
        }

        b_ps = c_ps;

        thread::sleep(Duration::from_millis(10));
    }
}
