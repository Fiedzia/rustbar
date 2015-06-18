extern crate rustbar;

use std::thread::sleep_ms;

use rustbar::rustbars::ProgressBar;

pub fn main(){

    let mut pbar = rustbar::rustbars::PercentageProgressBar::new();
    println!("\nPercentageProgressBar demo");
    for idx in 0..101 {
        pbar.set_value(idx);
        pbar.set_msg("Calculating...");
        pbar.render();
        sleep_ms(50);
    }

    println!("\nInifiniteProgressBar demo");
    let mut infbar  = rustbar::rustbars::InfiniteProgressBar::new();
    for _ in 0..101 {
        infbar.set_msg("Thinking...");
        infbar.render();
        sleep_ms(50);
    }

}
