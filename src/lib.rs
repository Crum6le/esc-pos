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
        let _ = self.sink.flush();
    }

    pub fn set_rightSide_character_spacing(&mut self, spacing: u8) {
        let _ = self.sink.write(gen_fixed_cmd!{
            0x03,
            SET_RIGHT_SIDE_CHARACTER_SPACING,
            [spacing]    
        });
        let _ = self.sink.flush();
    }

    //-------------

    pub fn test_print(&mut self) {
        let _ = self.sink.write(gen_fixed_cmd! {
            0x07,
            EXECUTE_TEST_PRINT,
            2u16.to_le_bytes(),
            [0x02, 0x40]
        });
        let _ = self.sink.flush();
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

impl_trait!(FFinPageMode, [EUM30, TMJ2000, TMJ2100, TML90, TML90LFC, TML100, TMm10, TMm30, TMm30II, TMm30IIH, TMm30IINT, TMm30IIS, TMm30IISL, TMm30III, TMm30IIIH, TMm50, TMm50II, TMm50IIH, TMP20, TMP20II, TMP60, TMP60II, TMP80, TMP80II, TMT100, TMT20, TMT20II, TMT20III, TMT20IIIL, TMT20X, TMT70, TMT70II, TMT81III, TMT82II, TMT82III, TMT82IIIL, TMT82X, TMT83III, TMT88IV, TMT88V, TMT88VI, TMT88VII, TMT90]);

impl<T: Write, Model:FFinPageMode> Printer<T, Model> {
    pub fn print_and_return_mode(&mut self){ //works in page mode
        let _ = self.sink.write(PRINT_AND_RETURN_TO_STANDARD_MODE.as_slice());
        let _ = self.sink.flush();
    }

    pub fn execute_power_off_sequence(&mut self) {
        let _ = self.sink.write(gen_fixed_cmd!{
            0x05,
            EXECUTE_POWER_OFF,
            [0x01, 0x08]
        });
        let _ = self.sink.flush();
    }

    pub fn clear_buffer(&mut self) {
        let _ = self.sink.write(gen_fixed_cmd!{
            0x0a,
            CLEAR_BUFFER,
            [0x01, 0x03, 0x14, 0x01, 0x06, 0x02, 0x08]
        });
        let _ = self.sink.flush();
    }

    pub fn cancel_print_data(&mut self) {
        let _ = self.sink.write([CAN].as_slice());
        let _ = self.sink.flush();
    }

    pub fn print_data_in_pagemode(&mut self) {
        let _ = self.sink.write(PRINT_DATA_IN_PAGEMODE.as_slice());
        let _ = self.sink.flush();
    }

    pub fn set_absolute_print_position(&mut self, position: u16) {
        let _ = self.sink.write(gen_fixed_cmd!{
            0x04,
            SET_ABSOLUTE_PRINT_POSITION,
            position.to_le_bytes()
        });
        let _ = self.sink.flush();
    }
}

impl_trait!(FFinStandardMode, [EUM30, TML100, TMm30, TMm30II, TMm30IIH, TMm30IINT, TMm30IIS, TMm30IISL, TMm30III, TMm30IIIH, TMm50, TMm50II, TMm50IIH, TMP20II, TMP80II]);

impl <T: Write, Model:FFinStandardMode> Printer<T, Model> {
    pub fn end_job(&mut self){
        let _ = self.sink.write(END_JOB.as_slice());
        let _ = self.sink.flush();
    }
}

impl_trait!(PrintAndReturnCarriage, [EUM30, TMJ2000, TML90, TML90LFC, TML100, TMm10, TMm30, TMm30II, TMm30IIH, TMm30IINT, TMm30IIS, TMm30IISL, TMm30III, TMm30IIIH, TMm50, TMm50II, TMm50IIH, TMT100, TMT20, TMT20II, TMT20III, TMT20IIIL, TMT20X, TMT70, TMT70II, TMT81III, TMT82II, TMT82III, TMT82IIIL, TMT82X, TMT83III, TMT88IV, TMT88V, TMT88VI, TMT88VII, TMT90, TMU220, TMU230]);

impl<T: Write, Model:PrintAndReturnCarriage> Printer<T, Model> {
    pub fn print_and_return_carriage(&mut self){
        let _ = self.sink.write(PRINT_AND_CARRIAGE_RETURN.as_slice());
        let _ = self.sink.flush();
    }
}

impl_trait!(GeneratePulseInRealTime, [EUM30, TMJ2000, TML90, TML90LFC, TML100, TMm10, TMm30, TMm30II, TMm30IIH, TMm30IINT, TMm30IIS, TMm30IISL, TMm30III, TMm30IIIH, TMm50, TMm50II, TMm50IIH, TMT100, TMT20, TMT20II, TMT20III, TMT20IIIL, TMT20X, TMT70, TMT70II, TMT81III, TMT82II, TMT82III, TMT82IIIL, TMT82X, TMT83III, TMT88IV, TMT88V, TMT88VI, TMT88VII, TMT90, TMU220]);

impl<T: Write, Model:GeneratePulseInRealTime> Printer<T, Model> {
    pub fn generate_pulse_in_realtime(&mut self, connectorPin: u8, time: u8) {
        let _ = self.sink.write(gen_fixed_cmd!{
            0x05,
            GENERATE_PULSE_IN_REAL_TIME,
            [connectorPin, time]
        });
        let _ = self.sink.flush();
    }
}

impl_trait!(SoundBuzzerInRealTime, [EUM30, TML100, TMm10, TMm30, TMm30II,  TMm30IIH, TMm30IINT, TMm30IIS, TMm30IISL, TMm30III, TMm30IIIH, TMm50, TMm50II, TMm50IIH, TMT20, TMT20II, TMT20III, TMT20IIIL, TMT20X, TMT70II, TMT81III, TMT82II, TMT82III, TMT82IIIL, TMT82X, TMT83III, TMT88V, TMT88VI, TMT88VII]);

impl <T: Write, Model:SoundBuzzerInRealTime> Printer<T, Model> {
    pub fn sound_buzzer_in_realtime(&mut self, soundpattern: u8, n: u8, times: u8, t1: u8, t2: u8){ //TODO change parameters to better names
        let _ = self.sink.write(gen_fixed_cmd!{
            0x08,
            SOUND_BUZZER,
            [soundpattern, n, times, t1, t2]
        });
        let _ = self.sink.flush();
    }
}

impl_trait!(TransmitSpecifiedStatusInRealtime, [EUM30, TML100, TMm10, TMm30, TMm30II, TMm30IIH, TMm30IINT, TMm30IIS, TMm30IISL, TMm30III, TMm30IIIH, TMm50, TMm50II, TMm50IIH, TMP20, TMP20II, TMP60, TMP60II, TMP80, TMP80II, TMT20IIIL, TMT20X, TMT81III, TMT82IIIL, TMT82X, TMT83III, TMT100]);

impl <T: Write, Model:TransmitSpecifiedStatusInRealtime> Printer<T, Model> {
    pub fn transmit_specified_status(&mut self, function: u8){
        let _ = self.sink.write(gen_fixed_cmd!{
            0x04,
            TRANSMIT_SPECIFIED_STATUS,
            [function]
        });
        let _ = self.sink.flush();
    }
}

impl_trait!(SelectPrintMode, [EUM30, TMJ2000, TML90, TML90LFC, TML100, TMm10, TMm30, TMm30II, TMm30IIH, TMm30IINT, TMm30IIS, TMm30IISL, TMm30III, TMm30IIIH, TMm50, TMm50II, TMm50IIH, TMP20, TMP20II, TMP60II, TMP80, TMP80II, TMT100, TMT20, TMT20II, TMT20III, TMT20IIIL, TMT20X, TMT70, TMT70II, TMT81III, TMT82II, TMT82III, TMT82IIIL, TMT82X, TMT83III, TMT88IV, TMT88V, TMT88VI, TMT88VII, TMT90, TMU220, TMU230]);

impl <T: Write, Model:SelectPrintMode> Printer<T, Model> {
    pub fn select_print_mode(&mut self, mode: u8){
        let _ = self.sink.write(gen_fixed_cmd!{
            0x03,
            SELECT_PRINT_MODE,
            [mode]
        });
        let _ = self.sink.flush();
    }
}

impl_trait!(SelectUserDefinedCharacterSet, [EUM30, TMJ2000, TML90, TML90LFC, TML100, TMm10, TMm30, TMm30II, TMm30IIH, TMm30IINT, TMm30IIS, TMm30IISL, TMm30III, TMm30IIIH, TMm50, TMm50II, TMm50IIH, TMP20, TMP20II, TMP80, TMP80II, TMT100, TMT20, TMT20II, TMT20III, TMT20IIIL, TMT20X, TMT70, TMT70II, TMT81III, TMT82II, TMT82III, TMT82IIIL, TMT82X, TMT83III, TMT88IV, TMT88V, TMT88VI, TMT88VII, TMT90, TMU220, TMU230]);

impl <T: Write, Model:SelectUserDefinedCharacterSet> Printer<T, Model> {
    pub fn select_userdefined_character_set(&mut self, mode: u8){
        let _ = self.sink.write(gen_fixed_cmd!{
            0x03,
            SELECT_USERDEFINED_CHARACTER_SET,
            [mode]
        });
        let _ = self.sink.flush();
    }

    pub fn define_userdefined_characters(&mut self){
        //TODO this function
    }
}

impl_trait!(BeepTheBuzzer, [TMP20, TMP20II, TMP60, TMP60II, TMP80, TMP80II]);

impl <T: Write, Model:BeepTheBuzzer> Printer<T, Model> {
    pub fn beep_the_buzzer(&mut self, soundType: u8, beepCounts: u8, cycle: u8){
        let _ = self.sink.write(gen_fixed_cmd!{
            0x09,
            CONTROL_BEEPER_TONES,
            [0x04, 0x00, 0x30, soundType, beepCounts, cycle]
        });
        let _ = self.sink.flush();
    }
}

impl_trait!(ModelSpecificBuzzerControl, [EUM30, TML100, TMm10, TMm30, TMm30II, TMm30IIH, TMm30III, TMm30IIIH, TMm30IINT, TMm30IIS, TMm30IISL, TMm50, TMm50II, TMm50IIH, TMT20, TMT20II, TMT20III, TMT20IIIL, TMT20X, TMT70II, TMT81III, TMT82II, TMT82III, TMT82IIIL, TMT82X, TMT83III, TMT88V, TMT88VI, TMT88VII, TMT100]);

impl <T: Write, Model:ModelSpecificBuzzerControl> Printer<T, Model> {
    pub fn control_buzzer(&mut self, pattern: u8, time: u8){
        let _ = self.sink.write(gen_fixed_cmd!{
            0x08,
            CONTROL_BEEPER_TONES,
            [0x03, 0x00, 0x61, pattern, time]
        });
        let _ = self.sink.flush();
    }
}

impl<T: Write> Printer<T, TMU230> {
    pub fn beep_the_internal_buzzer(&mut self, count: u8, on_time: u8, off_time: u8) {
        let _ = self.sink.write(gen_fixed_cmd!{
            0x0a,
            CONTROL_BEEPER_TONES,
            [0x05, 0x00, 0x61, 0x64, count, on_time, off_time]
        });
        let _ = self.sink.flush();
    }
    
    pub fn beep_the_internal_buzzer_when_offline(&mut self, factor: u8, beep_type: u8, on_time: u8, off_time: u8) {
        let _ = self.sink.write(gen_fixed_cmd!{
            0x0c,
            CONTROL_BEEPER_TONES,
            [0x07, 0x00, 0x62, factor, 0x01, 0x64, beep_type, on_time, off_time]
        });
        let _ = self.sink.flush(); 
    } 

    pub fn beep_the_internal_buzzer_when_not_offline(&mut self, beep_type: u8, on_time: u8, off_time: u8){
        let _ = self.sink.write(gen_fixed_cmd!{
            0x0c,
            CONTROL_BEEPER_TONES,
            [0x07, 0x00, 0x63, 0x30, 0x01, 0x64, beep_type, on_time, off_time]
        });
        let _ = self.sink.flush(); 
    }
}

impl_trait!(SpecifyBatchPrint, [EUM30, TML100, TMm30II, TMm30IIH, TMm30III, TMm30IIIH, TMm30IINT, TMm30IIS, TMm30IISL, TMm50, TMm50II, TMm50IIH, TMP20II, TMP80, TMT88VII]);

impl<T: Write, Model: SpecifyBatchPrint> Printer<T, Model>{
    pub fn specify_batch_print(&mut self, mode: u8, direction: u8){
        let _ = self.sink.write(gen_fixed_cmd!{
            0x07,
            SPECIFY_BATCH_PRINT,
            [0x02, 0x00, mode, direction]
        });
        let _ = self.sink.flush(); 
    }
}

impl<T: Write> Printer<T, TMT88IV> {

}

impl<T: Write> Printer<T, TMT88V> {
    
}

//EUM30, TMJ2000, TML90, TML90LFC, TML100, TMm10, TMm30, TMm30II, TMm30IIH, TMm30IINT, TMm30IIS, TMm30IISL, TMm30III, TMm30IIIH, TMm50, TMm50II, TMm50IIH, TMP20, TMP20II, TMP60, TMP60II, TMP80, TMP80II, TMT100, TMT20, TMT20II, TMT20III, TMT20IIIL, TMT20X, TMT70, TMT70II, TMT81III, TMT82II, TMT82III, TMT82IIIL, TMT82X, TMT83III, TMT88IV, TMT88V, TMT88VI, TMT88VII, TMT90, TMT100, TMU220, TMU230
