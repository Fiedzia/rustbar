//taken mostly from http://hermanradtke.com/2015/01/12/terminal-window-size-with-rust-ffi.html
use libc::{c_ushort, c_ulong};
use libc::funcs::bsd44::ioctl;
use libc::consts::os::posix88::STDOUT_FILENO;

//const TIOCGWINSZ: i32 = 0x40087468;
const TIOCGWINSZ: i32 = 0x5413;

#[repr(C)]
struct winsize {
    ws_row: c_ushort, /* rows, in characters */
    ws_col: c_ushort, /* columns, in characters */
    ws_xpixel: c_ushort, /* horizontal size, pixels */
    ws_ypixel: c_ushort /* vertical size, pixels */
}


pub fn get_winsize() -> Result<(usize, usize), String> {
    let mut w = winsize { ws_row: 0, ws_col: 0, ws_xpixel: 0, ws_ypixel: 0 };
    let r = unsafe { ioctl(STDOUT_FILENO, TIOCGWINSZ, &w) };
     
    match r {
        0 => Ok((w.ws_col as usize, w.ws_row as usize)),
        _ => {
            return Err("cannot get screen size".to_owned())
        }
    }
}


