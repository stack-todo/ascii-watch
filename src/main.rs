// main.rs
// author: vishalpaudel
// date  : 2024-07-20
// note  : A simple stdout ~stop~watch

use std::io::{self, Write};
use std::thread;
use std::time::{Duration, Instant};

struct Watch {
    secns: u64,
    mints: u64,
    hours: u64,
}

#[derive(Debug)]
enum BlinkingMode {
    Show,
    Hide,
}

fn main() {
    let mut watch = Watch {
        secns: 0,
        mints: 0,
        hours: 0,
    };

    let start: Instant = Instant::now();
    let mut mode: BlinkingMode = BlinkingMode::Show;

    loop {
        update_watch(start, &mut watch);
        update_blinking_mode(&mut mode);

        draw_watch(&mut watch, &mode);
        thread::sleep(Duration::from_millis(500));
    }
}

fn update_watch(start: Instant, watch: &mut Watch) {
    // watch.secns += 1;
    let elapsed_start = start.elapsed().as_secs();

    let secns: u64 = (elapsed_start) % 60;
    let mints: u64 = ((elapsed_start) / 60) % 60;
    let hours: u64 = ((elapsed_start / 60) / 60) % 24;

    watch.secns = secns;
    watch.mints = mints;
    watch.hours = hours;
}

fn format_watch(watch: &Watch, mode: &BlinkingMode) -> String {
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

fn draw_watch(watch: &Watch, mode: &BlinkingMode) {
    let formatted_time: String = format_watch(watch, mode);

    print!("\r\x1B[K{}", formatted_time);
    io::stdout().flush().unwrap();
}

fn update_blinking_mode(blinking_mode: &mut BlinkingMode) {
    *blinking_mode = match blinking_mode {
        BlinkingMode::Show => BlinkingMode::Hide,
        BlinkingMode::Hide => BlinkingMode::Show,
    };
}
