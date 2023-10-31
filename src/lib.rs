#![feature(iter_next_chunk)]
use std::{io::Write, marker::PhantomData};

pub mod type_state;

pub mod consts;
#[macro_use]
mod macros;

use type_state::*;

use crate::consts::*;

pub struct Printer<T: Write, Model> {
    sink: T,
    _model: PhantomData<Model>
}

impl<T: Write, Model> Printer<T, Model> {
    pub fn new(mut sink: T, _model: Model) -> Self {
        let _ = sink.write(&[ESC, 0x40]);
        let _ = sink.write(&[LF]);
        Self {
            sink,
            _model: PhantomData::default()
        }
    }

    pub fn horizontal_tab(&mut self){
        let _ = self.sink.write(HORIZONTAL_TAB.as_slice());
        let _ = self.sink.flush();
    }

    pub fn line_feed(&mut self) {
        let _ = self.sink.write(LINE_FEED.as_slice());
        let _ = self.sink.flush();    
    }

    pub fn transmit_real_time_status(&mut self, funcmode: u8, secfuncmode: Option<u8>) -> u8{
        0x00
    }//TODO

    pub fn send_realtime_request(&mut self, funcmode: u8) {
        let _ = self.sink.write(gen_fixed_cmd! {
            0x03,
            SEND_REALTIME_REQUEST,
            [funcmode]
        });
    }
    //-------------

    pub fn test_print(&mut self) {
        let _ = self.sink.write(gen_fixed_cmd! {
            0x07,
            EXECUTE_TEST_PRINT,
            2u16.to_le_bytes(),
            [0x02, 0x40]
        });
    }

    pub fn paper_cut(&mut self, cut_mode: u8, vertical_motion: Option<u8>) {
        let _ = self.sink.write(gen_fixed_cmd! {
            0x04,
            SELECT_PAPER_CUT_MODE_AND_CUT,
            [cut_mode, vertical_motion.unwrap_or(0x00)]
        });
        let _ = self.sink.flush();
    }

    #[deprecated(note="Use paper_cut() instead.")]
    pub fn partial_paper_cut(&mut self){
        let _ = self.sink.write(PARTIAL_CUT_ONE_POINT.as_slice());
        let _ = self.sink.flush();
    }

    /*pub fn print(&mut self, string: &str) {
        let _ = self.sink.write(string.as_bytes());
    }*/
}

impl FFinPageMode for (EUM30, TMJ2000, TML90, TML90LFC, TML100, TMm10, TMm30, TMm30II, TMm30IIH, TMm30IINT, TMm30IIS, TMm30IISL, TMm30III, TMm30IIIH, TMm50, TMm50II, TMm50IIH, TMP20, TMP20II, TMP60, TMP60II, TMP80, TMP80II, TMT100, TMT20, TMT20II, TMT20III, TMT20IIIL, TMT20X, TMT70, TMT70II, TMT81III, TMT82II, TMT82III, TMT82IIIL, TMT82X, TMT83III, TMT88IV, TMT88V, TMT88VI, TMT88VII, TMT90){}

impl<T: Write, Model:FFinPageMode> Printer<T, Model> {
    pub fn print_and_return_mode(&mut self){ //works in page mode
        let _ = self.sink.write(PRINT_AND_RETURN_TO_STANDARD_MODE.as_slice());
        let _ = self.sink.flush();
    }
}

impl FFinStandardMode for (EUM30, TML100, TMm30, TMm30II, TMm30IIH, TMm30IINT, TMm30IIS, TMm30IISL, TMm30III, TMm30IIIH, TMm50, TMm50II, TMm50IIH, TMP20II, TMP80II){}

impl <T: Write, Model:FFinStandardMode> Printer<T, Model> {
    pub fn end_job(&mut self){
        let _ = self.sink.write(END_JOB.as_slice());
        let _ = self.sink.flush();
    }
}

impl PrintAndReturnCarriage for (EUM30, TMJ2000, TML90, TML90LFC, TML100, TMm10, TMm30, TMm30II, TMm30IIH, TMm30IINT, TMm30IIS, TMm30IISL, TMm30III, TMm30IIIH, TMm50, TMm50II, TMm50IIH, TMT100, TMT20, TMT20II, TMT20III, TMT20IIIL, TMT20X, TMT70, TMT70II, TMT81III, TMT82II, TMT82III, TMT82IIIL, TMT82X, TMT83III, TMT88IV, TMT88V, TMT88VI, TMT88VII, TMT90, TMU220, TMU230){}

impl<T: Write, Model:PrintAndReturnCarriage> Printer<T, Model> {
    pub fn print_and_return_carriage(&mut self){
        let _ = self.sink.write(PRINT_AND_CARRIAGE_RETURN.as_slice());
        let _ = self.sink.flush();
    }
}

impl GeneratePulseInRealTime for (EUM30, TMJ2000, TML90, TML90LFC, TML100, TMm10, TMm30, TMm30II, TMm30IIH, TMm30IINT, TMm30IIS, TMm30IISL, TMm30III, TMm30IIIH, TMm50, TMm50II, TMm50IIH, TMT100, TMT20, TMT20II, TMT20III, TMT20IIIL, TMT20X, TMT70, TMT70II, TMT81III, TMT82II, TMT82III, TMT82IIIL, TMT82X, TMT83III, TMT88IV, TMT88V, TMT88VI, TMT88VII, TMT90, TMU220){}

impl<T: Write, Model:GeneratePulseInRealTime> Printer<T, Model> {
    pub fn generate_pulse_in_realtime(&mut self, connectorPin: u8, time: u8) {
        let _ = self.sink.write(gen_fixed_cmd!{
            0x05,
            GENERATE_PULSE_IN_REAL_TIME,
            [connectorPin, time]
        });
    }
}

impl<T: Write> Printer<T, TMT88IV> {

}

impl<T: Write> Printer<T, TMT88V> {
    
}

//EUM30, TMJ2000, TML90, TML90LFC, TML100, TMm10, TMm30, TMm30II, TMm30IIH, TMm30IINT, TMm30IIS, TMm30IISL, TMm30III, TMm30IIIH, TMm50, TMm50II, TMm50IIH, TMP20, TMP20II, TMP60, TMP60II, TMP80, TMP80II, TMT100, TMT20, TMT20II, TMT20III, TMT20IIIL, TMT20X, TMT70, TMT70II, TMT81III, TMT82II, TMT82III, TMT82IIIL, TMT82X, TMT83III, TMT88IV, TMT88V, TMT88VI, TMT88VII, TMT90, TMU220, TMU230
