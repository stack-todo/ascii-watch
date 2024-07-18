// main.rs
// author: vishalpaudel
// date  : 2024-07-18
// note  : a simple blinking stopwatch* (stopping is C-c for now)

use std::time::{Duration, Instant};

// blinking mode for the colon seperating the HH:MM:SS,
// may have additional modes (Grey) depending on the format of print/draw;
enum BlinkingMode {
    Show,
    Hide,
}

fn main() {
    // print/drawing triggered in these times;
    let refresh_print_milliseconds = 100;
    let refresh_print = Duration::from_millis(refresh_print_milliseconds);

    // mode change of the colon triggered;
    let refresh_chmod_milliseconds = 500;
    let refresh_chmod = Duration::from_millis(refresh_chmod_milliseconds);

    let start = Instant::now();

    let mut last_print = start;
    let mut last_chmod = start;

    let mut blinking_mode = BlinkingMode::Show;

    loop {
        let now = Instant::now();
        let elapsed_start = start.elapsed();

        let seconds = (elapsed_start.as_secs()) % 60;
        let minutes = (seconds / 60) % 60;
        let hours = (minutes / 60) % 24;

        let with_colon = format!("{:02?}:{:02?}:{:02?}", hours, minutes, seconds);
        let wout_colon = format!("{:02?} {:02?} {:02?}", hours, minutes, seconds);

        // printing/drawing the time
        let elapsed_last_print = now - last_print;
        if elapsed_last_print >= refresh_print {
            match blinking_mode {
                BlinkingMode::Show => {
                    println!("{}", with_colon);
                    last_print = Instant::now()
                }
                BlinkingMode::Hide => {
                    println!("{}", wout_colon);
                    last_print = Instant::now()
                } //_ => panic!(),
            }
        }

        // changing (flipping) blinking mode
        let elapsed_colon_chmod = now - last_chmod;
        if elapsed_colon_chmod >= refresh_chmod {
            match blinking_mode {
                BlinkingMode::Show => {
                    blinking_mode = BlinkingMode::Hide;
                    last_chmod = Instant::now();
                }
                BlinkingMode::Hide => {
                    blinking_mode = BlinkingMode::Show;
                    last_chmod = Instant::now();
                } //_ => panic!()
            }
        }
    }
}
