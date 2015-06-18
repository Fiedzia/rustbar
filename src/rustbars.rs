use std::io::{stdout, Write};

pub trait ProgressBar<T> {
    fn new() -> T;
}


#[derive(Default)]
pub struct PercentageProgressBar {
    value: u8, //0..100
}

impl ProgressBar<PercentageProgressBar> for PercentageProgressBar {

    fn new() -> PercentageProgressBar {
        PercentageProgressBar { ..Default::default()}
    }
}

impl PercentageProgressBar {

    pub fn show(&self) {
        print!("\r{}%", self.value);
        stdout().flush().ok().expect("");
    }

    pub fn set_value(&mut self, value: u8) { if value <= 100 { self.value = value } }
    pub fn get_value(&self) -> u8 { self.value }
    pub fn inc(&mut self) { if self.value < 100 { self.value += 1; } }
    pub fn dec(&mut self) { if self.value > 0 { self.value -= 1; } }

}

#[test]
fn test_percentage_progressbar() {
    let mut ppb = PercentageProgressBar { ..Default::default()};
}
