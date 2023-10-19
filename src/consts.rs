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
def_cmd! { //Every Command has to be checked for Printer
    HORIZONTAL_TAB => [HT],
    LINE_FEED => [LF],
    PRINT_AND_RETURN_TO_STANDARD_MODE => [FF], //only in Page Mode
    END_JOB => [FF], //only in Standard Mode
    PRINT_AND_CARRIAGE_RETURN => [CR],
    //DLE
    TRANSMIT_REALTTIME_STAUTS => [DLE, EOT],
    SEND_REALTIME_REQUEST => [DLE, ENQ],
    GENERATE_PULSE => [DLE, DC4, 1],
    EXECUTE_POWER_OFF => [DLE, DC4, 2],
    SOUND_BUZZER => [DLE, DC4, 3],
    TRANSMIT_SPECIFIED_STATUS => [DLE, DC4, 7],
    CLEAR_BUFFER => [DLE, DC4, 8],
    //CAN
    CANCEL_PRINT_DATA => [CAN],
    //ESC
    PRINT_DATA_IN_PAGEMODE => [ESC, FF],
    SET_RIGHT_SIDE_CHARACTER_SPACING => [ESC, SP],
    SELECT_PRINT_MODE => [ESC, b'!'],
    SET_ABSOLUTE_PRINT_POSITION => [ESC, b'$'],
    SELECT_USERDEFINED_CHARACTER_SET => [ESC, b'%'],
    DEFINE_USERDEFINED_CHARACTER_SET => [ESC, b'&'],
    CONTROL_BEEPER_TONES => [ESC, b'(', b'A'],  //48->Beep the buzzer 
                                                //97-99->Model specific buzzer control 
    SPECIFY_BATCH_PRINT => [ESC, b'(', b'Y'],
    SELECT_BITIMAGE_MODE => [ESC, b'*'],
    SWITCH_UNDERLINE_MODE => [ESC, b'-'],
    SELECT_DEFAULT_LINE_SPACING => [ESC, 2],
    SET_LINE_SPACING => [ESC, 3],
    RETURN_HOME => [ESC, b'<'],
    SELECT_PEROPHERAL_DEVICE => [ESC, b'='],
    CANCEL_USER_DEFINED_CHARACTERS => [ESC, b'?'],
    INITIALIZE_PRINTER => [ESC, b'@'],
    SET_HORIZONTAL_TAB => [ESC, b'D'],
    SWITCH_EMPHASIZED_MODE => [ESC, b'E'],
    SWITCH_DOUBLESTRIKE_MODE => [ESC, b'G'],
    PRINT_AND_FEED_PAPER => [ESC, b'J'],
    PRINT_AND_REVERSE_FEED => [ESC, b'K'],
    SELECT_PAGE_MODE => [ESC, b'L'],
    SLECT_CHARACTER_FONT => [ESC, b'M'],
    SELECT_AN_INTERNATIONAL_CHARACTER_SET => [ESC, b'R'],
    SELECT_STANDARD_MODE => [ESC, b'S'],
    SELECT_PRINT_DIRECTION_IN_PAGE_MODE => [ESC, b'T'],
    SWITCH_UNIDIRECTIONAL_PRINT_MODE => [ESC, b'U'],
    SWITCH_90_CLOCKWISE_ROTATION_MODE => [ESC, b'V'],
    SET_PRINT_AREA_IN_PAGE_MODE => [ESC, b'W'],
    SET_RELATIVE_PRINT_POSITION => [ESC, b'\\'],
    SELECT_JUSTIFICATION => [ESC, b'a'],
    SELECT_PAPER_SENSOR_OUTPUT_PAPEREND_SIGNAL => [ESC, b'c', 3],
    SELECT_PAPER_SEONSOR_TO_STOP_PRINTIG => [ESC, b'c', 4],
    SWITCH_PANEL_BUTTONS => [ESC, b'c', 5],
    PRINT_AND_FEED_LINES => [ESC, b'd'],
    PRINT_AND_REVERSE_FEED_LINES => [ESC, b'e'],
    PARTIAL_CUT_ONE_POINT => [ESC, b'i'],
    PARTIAL_CUT_THREE_POINTS => [ESC, b'm'],
    GENERATE_PULSE => [ESC, b'p'],
    SELECT_PRINT_COLOR => [ESC, b'r'],
    SELECT_CHARACTER_CODE_TABLE => [ESC, b't'],
    TRANSMIT_PERIPHERAL_DEVICE_STATUS => [ESC, b'u'],
    TRANSMIT_PAPER_SENSOR_STATUS => [ESC, b'v'],
    SWITCH_UPSIDEDOWN_PRINT_MODE => [ESC, b'{'],
    //FS
    SELECT_PRINT_MODE_FOR_KANJI_CHARACTERS => [FS, b'!'],
    SELECT_KANJI_CHARACTER_MODE => [FS, b'&'],
    //SELECT_KANJI_CHARACTER_MODE => [FS, b'('], 
    SELECT_KANJI_CHARACTER_STYLES => [FS, b'(', b'A'],  //48->Select Kanji character font
    SELECT_CODE_CONVERSION_METHOD => [FS, b'(', b'C'],  //48->Select character encode system 
                                                        //60->Set font priority
    GROUP_OF_COMMANDS_FOR_RECEIPT_ENHANCEMENT_CONTROL => [FS, b'(', b'E'],  //60->Cancel set values for top/bottom logo printing 
                                                                            //61->Transmit set values for top/bottom logo printing 
                                                                            //62->Set logo printing | 63->Set bottom logo printing 
                                                                            //64->Make extended settings for top/bottom logo printing 
                                                                            //65-> Enable/Disable top/bottom logo printing
    SELECT_LABEL_AND_BLACK_MARK_CONTROL_FUNCTION => [FS, b'(', b'L'],   //33->Paper layout setting 
                                                                        //34->Paper layout information transmission 
                                                                        //48->Transmit the positioning information 
                                                                        //65->Feed paper to the label peeling position 
                                                                        //66->Feed paper to the cutting position 
                                                                        //67-> Feed paper to the print starting position 
                                                                        //80->Paper layout error special margin setting
    SWITCH_AUTOMATIC_STATUS_BACK_FOR_OPTIONAL_FUNCTIONS => [FS, b'(', b'e'],
    SWITCH_UNDERLINE_MODE_FOR_KANJI_CHARACTERS => [FS, b'-'],
    CANCEL_KANJI_CHARACTER_MODE => [FS, b'.'],
    DEFINE_USERDEFINED_KANJI_CHARACTERS => [FS, 2],
    CANCEL_USER_DEFINED_KANJI_CHARACTERS => [FS, b'?'],
    SELECT_KANJI_CHARACTER_CODE_SYSTEM => [FS, b'C'],
    SET_KANJI_CHARACTER_SPACING => [FS, b'S'],
    SWITCH_QUADRUPLESIZE_MODE_FOR_KANJI_CHARACTERS => [FS, b'W'],
    WRITE_TO_NV_USER_MEMORY => [FS, b'g', 1],
    READ_FROM_NV_USER_MEMORY => [FS, b'g', 2],
    PRINT_NV_BIT_IMAGE => [FS, b'p'],
    DEFINE_NV_BIT_IMAGE => [FS, b'q'],
    //GS
    SELECT_CHARACTER_SIZE => [GS, b'!'],
    SET_ABSOLUTE_VERTICAL_PRINT_POSITION_IN_PAGE_MODE => [GS, b'$'],
    EXECUTE_TEST_PRINT => [GS, b'(',b'A'],
    EDIT_NV_USER_MEMORY => [GS, b'(',b'C'], //0->Delete the specified record 
                                            //1->Store the data in the specified record 
                                            //2->Transmit the data in the specified record
                                            //3->Transmit the remaining cpacity of the NV user memory 
                                            //5->Transmit the key code list 
                                            //6->Delete all data in the NV user memory
    SWITCH_REAL_TIME_COMMAND => [GS, b'(', b'D'],
    SET_USER_SETUP_COMMANDS => [GS, b'(', b'E'],    //1->Change into the user setting mode 
                                                    //2->End the user setting mode session
                                                    //3->Change the memory switch
                                                    //4->Transmit the settings of the memory switch
                                                    //5->Set the customized settings values
                                                    //6->Transmit the customized setting values
                                                    //7->Copy the user defined page
                                                    //8->Define the data (Column format) for the character code page
                                                    //9->Define the data (Raster format) for the character code page
                                                    //10->Delete the data for the character code page
                                                    //11->Set the configuration item for the serial interface
                                                    //12->Transmit the configuration item for the Serial interface
                                                    //13->Set the configuration item for the Bluetooth interface
                                                    //14->Transmit the configuration item for the Bluetooth interface
                                                    //15->Set conditions for USB interface communication
                                                    //16->Transmit conditions for USB interface communication 
                                                    //48->Delete the paper layout 
                                                    //49->Set the paper layout
                                                    //50->Transmit the paper layout information
                                                    //51->Set the control for label paper and paper with black marks
                                                    //52->Transmit the control settings for label paper and paper with black marks
                                                    //99->Set internal buzzer patterns
                                                    //100->Transmit internal buzzer patterns
    SELECT_PRINT_CONTROL_METHOD => [GS, b'(', b'K'],//48->Select the print control mode
                                                    //49->Select the print density
                                                    //50->Select the print speed
                                                    //97->Select the number of parts for the thermal head energizing
    SET_GRAPHICS_DATA_SMALL => [GS, b'(',b'L'], //48->Transmit the NV graphics memory capacity
                                                //49->Set the reference dot density for graphics
                                                //50->Print the graphics data in the print buffer
                                                //51->Transmit the remaining capacity of the NV graphics memory
                                                //52->Transmit the remaining capacity of the download graphics memory
                                                //64->Transmit the key code list for defined NV graphics
                                                //65->Delete all NV graphics data
                                                //66->Delete the specified NV graphics data
                                                //67->Define the NV graphics data(raster format)
                                                //68->Defube the NV graphics data(column fomrat)
                                                //69->Print specified NV graphics data
                                                //80->Transmit the key Code list for defined download graphics
                                                //81->Delete all download graphiccs data
                                                //82->Delete the specified download graphics data
                                                //83->Define the download graphics data (Raster format)
                                                //84->Define the download graphics data (Column format)
                                                //85->Print the specified download graphics data
                                                //112->Store the graphics data in the print buffer(raster format)
                                                //113->Store the graphics data in the print buffer(column format)
    SET_GRAPHICS_DATA_BIG => [GS, b'8', b'L'],  //67->Define the NV graphics data(raster format)
                                                //68->Defube the NV graphics data(column fomrat)
                                                //83->Define the download graphics data (Raster format)
                                                //84->Define the download graphics data (Column format)
                                                //112->Store the graphics data in the print buffer(raster format)
                                                //113->Store the graphics data in the print buffer(column format)

    PAPER_CUT => [GS, 0x56]
}
