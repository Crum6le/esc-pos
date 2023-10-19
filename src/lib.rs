#![feature(iter_next_chunk)]
use std::io::Write;

pub mod consts;
#[macro_use]
mod macros;

use crate::consts::*;

pub struct Printer<T: Write> {
    sink: T,
}

impl<T: Write> Printer<T>{
    pub fn new(mut sink: T) -> Self {
        let _ = sink.write(&[ESC, 0x40]);
        let _ = sink.write(&[LF]);
        Self {
            sink
        }
    }

    pub fn test_print(&mut self) {
        let _ = self.sink.write(gen_fixed_cmd! {
            0x07,
            EXECUTE_TEST_PRINT,
            2u16.to_le_bytes(),
            [0x02, 0x40]
        });
    }

    pub fn paper_cut(&mut self, cut_mode: u8, vertical_motion: u8) {
        let _ = self.sink.write(gen_fixed_cmd! {
            0x04,
            SELECT_PAPER_CUT_MODE_AND_CUT,
            [cut_mode],
            [vertical_motion]
        });
        let _ = self.sink.flush();
    }

    #[deprecated(note="Use paper_cut() instead.")]
    pub fn partial_paper_cut(&mut self){
        let _ = self.sink.write(gen_fixed_cmd!{
            0x02,
            PARTIAL_CUT_ONE_POINT
        });
        let _ = self.sink.flush();
    }

    pub fn line_feed(&mut self) {
        let _ = self.sink.write(LINE_FEED.as_slice());
    }

    pub fn print(&mut self, string: &str) {
        let _ = self.sink.write(string.as_bytes());
    }
}