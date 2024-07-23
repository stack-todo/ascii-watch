// main.rs
// author: vishalpaudel
// date  : 2024-07-20
// note  : A simple stdout ~stop~watch

use std::io::{self, Write};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy)]
struct Watch {
    secns: u8,
    mints: u8,
    hours: u8,
}

#[derive(Debug, Clone, Copy)]
enum BlinkingMode {
    Show,
    Hide,
}

fn main() {
    let mut watch;
    let start: Instant = Instant::now();
    let mut mode: BlinkingMode = BlinkingMode::Show;
    loop {
        watch = return_updated_watch(start);
        mode = return_updated_mode(mode);
        draw_watch(watch, mode);
        thread::sleep(Duration::from_millis(500));
    }
}

fn return_updated_watch(start: Instant) -> Watch {
    let elapsed_start = start.elapsed().as_secs();
    let secns: u8 = ((elapsed_start) % 60) as u8;
    let mints: u8 = (((elapsed_start) / 60) % 60) as u8;
    let hours: u8 = (((elapsed_start / 60) / 60) % 24) as u8;
    return Watch {
        secns,
        mints,
        hours,
    };
}

fn return_updated_mode(blinking_mode: BlinkingMode) -> BlinkingMode {
    return match blinking_mode {
        BlinkingMode::Show => BlinkingMode::Hide,
        BlinkingMode::Hide => BlinkingMode::Show,
    };
}

fn format_watch(watch: Watch, mode: BlinkingMode) -> String {
    let with_colon: String = format!(
        "{:02?}:{:02?}:{:02?}",
        watch.hours, watch.mints, watch.secns
    );
    let wout_colon: String = format!(
        "{:02?} {:02?} {:02?}",
        watch.hours, watch.mints, watch.secns
    );
    return match mode {
        BlinkingMode::Show => with_colon,
        BlinkingMode::Hide => wout_colon,
    };
}

fn draw_watch(watch: Watch, mode: BlinkingMode) {
    let formatted_time: String = format_watch(watch, mode);
    print!("\r\x1B[K{}", formatted_time);
    io::stdout().flush().unwrap();
}
