use crate::{def_cmd, def_const_bytes};

def_const_bytes! {
    EOT => 0x04,
    ENQ => 0x05,
    HT => 0x09,
    LF => 0x0a,
    FF => 0x0c,
    CR => 0x0d,
    DLE => 0x10,
    DC4 => 0x14,
    CAN => 0x18,
    ESC => 0x1b,
    FS => 0x1c,
    GS => 0x1d,
    SP => 0x20

}
def_cmd! {
    HORIZONTAL_TAB => [HT],
    LINE_FEED => [LF],
    PRINT_AND_RETURN_TO_STANDARD => [FF],
    PRINT_AND_CARRIAGE_RETURN => [CR],
    TRANSMIT_REALTTIME_STAUTS => [DLE, EOT],
    SEND_REALTIME_REQUEST => [DLE, ENQ],
    GENERATE_PULSE => [DLE, DC4, 1],
    EXECUTE_POWER_OFF => [DLE, DC4, 2],
    CLEAR_BUFFER => [DLE, DC4, 8],
    CANCEL_PRINT_DATA => [CAN],
    PRINT_DATA_IN_PAGEMODE => [ESC, FF],
    SET_RIGHT_SIDE_CHARACTER_SPACING => [ESC, SP],
    SELECT_PRINT_MODE => [ESC, b'!'],
    SET_ABSOLUTE_PRINT_POSITION => [ESC, b'$'],
    SELECT_USERDEFINED_CHARACTER_SET => [ESC, b'%'],
    DEFINE_USERDEFINED_CHARACTER_SET => [ESC, b'&'],
    SELECT_BITIMAGE_MODE => [ESC, b'*'],
    SWITCH_UNDERLINE_MODE => [ESC, b'-'],
    SELECT_DEFAULT_LINE_SPACING => [ESC, 2],
    SET_LINE_SPACING => [ESC, 3],
    SELECT_PEROPHERAL_DEVICE => [ESC, b'='],
    CANCEL_USER_DEFINED_CHARACTERS => [ESC, b'?'],
    INITIALIZE_PRINTER => [ESC, b'@'],
    SET_HORIZONTAL_TAB => [ESC, b'D'],
    SWITCH_EMPHASIZED_MODE => [ESC, b'E'],
    SWITCH_DOUBLESTRIKE_MODE => [ESC, b'G'],
    PRINT_AND_FEED_PAPER => [ESC, b'J'],
    EXECUTE_TEST_PRINT => [GS, b'(',b'A'],
    PAPER_CUT => [GS, 0x56]
}
