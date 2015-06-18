extern crate rustbar;

use std::thread::sleep_ms;

use rustbar::rustbars::ProgressBar;

pub fn main(){

    let mut pbar = rustbar::rustbars::PercentageProgressBar::new();
    println!("PercentageProgressBar demo");
    for idx in 0..101 {
        pbar.set_value(idx);
        pbar.show();
        sleep_ms(50);
    }
}
