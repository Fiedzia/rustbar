use std::io::{stdout, stderr, Write, Result};

//use term_utils;

fn write_to_stdout(buf: String)-> Result<()>{
    let mut output = stdout();
    try!(output.write(buf.as_bytes()));
    try!(output.flush());
    Ok(())
}

fn write_to_stderr(buf: String) -> Result<()> {
    let mut output = stderr();
    try!(output.write(buf.as_bytes()));
    try!(output.flush());
    Ok(())
}

///all progressbars will implement it
pub trait ProgressBar<T> {
    fn new() -> T;
    fn to_stderr(mut self) -> T;
    fn write(&self, buf: String) -> Result<()>;
}


pub struct PercentageProgressBar {
    value: u8, //0..100
    msg:   String,
    write_fn: fn(String) -> Result<()>,
}

impl ProgressBar<PercentageProgressBar> for PercentageProgressBar {

    fn new() -> PercentageProgressBar {
        PercentageProgressBar { value: 0, msg: "".to_owned(),  write_fn: write_to_stdout}
    }

    fn to_stderr(mut self) ->  PercentageProgressBar{
        self.write_fn = write_to_stderr;
        self
    }

    fn write(&self, buf: String) -> Result<()> {
        try!((self.write_fn)(buf));
        Ok(())
    }

}


impl PercentageProgressBar {

    pub fn render(&mut self) -> Result<()> {

        let s:String = format!("\r{msg} {value}%", msg=self.msg, value=self.value);
        try!(self.write(s));
        Ok(())
    }

    pub fn set_value(&mut self, value: u8) { if value <= 100 { self.value = value } }
    pub fn get_value(&self) -> u8 { self.value }

    pub fn set_msg(&mut self, msg: &str) { self.msg = msg.to_owned() }
    pub fn get_msg(&self) -> &str { self.msg.as_ref() }


    pub fn inc(&mut self) { if self.value < 100 { self.value += 1; } }
    pub fn dec(&mut self) { if self.value > 0 { self.value -= 1; } }

}

pub struct InfiniteProgressBar {
    msg:   String,
    marker_position:  i8,
    step: i8,
    write_fn: fn(String) -> Result<()>,
}

impl Default for InfiniteProgressBar {
    fn default() -> InfiniteProgressBar {
        InfiniteProgressBar {
            step: 1,
            msg: "".to_owned(),
            marker_position: 0,
            write_fn: write_to_stdout
        }
    }
}

impl ProgressBar<InfiniteProgressBar> for InfiniteProgressBar {

    fn new() -> InfiniteProgressBar {
        InfiniteProgressBar { ..Default::default()}
    }

    fn to_stderr(mut self) -> InfiniteProgressBar {
        self.write_fn = write_to_stderr;
        self
    }

    fn write(&self, buf: String) -> Result<()> {
        try!((self.write_fn)(buf));
        Ok(())
    }

}

impl InfiniteProgressBar {

    pub fn set_msg(&mut self, msg: &str) { self.msg = msg.to_owned() }
    pub fn get_msg(&self) -> &str { self.msg.as_ref() }

    pub fn render(&mut self) -> Result<()> {


        //let (screen_w, screen_h) = term_utils::get_winsize().unwrap();

        if self.marker_position <= 0 {
            self.marker_position = 0;
            self.step = 1;
        } else if self.marker_position > 9 {
            self.marker_position = 10;
            self.step = -1;
        }
        self.marker_position = self.marker_position + self.step;


        let mut bar:String = "..........".to_owned(); //10 dots
        bar.insert(self.marker_position as usize, '#');


        try!(self.write(format!("\r{msg} [{bar}]", msg=self.msg, bar=bar)));
        Ok(())

    }

}
