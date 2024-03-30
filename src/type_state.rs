pub trait FFinPageMode {}
pub trait FFinStandardMode {}
pub trait PrintAndReturnCarriage {}
pub trait GeneratePulseInRealTime {}
pub trait SoundBuzzerInRealTime {}
pub trait TransmitSpecifiedStatusInRealtime {}
pub trait SelectPrintMode {}
pub trait SelectUserDefinedCharacterSet {
    const X: u8;
    const Y: u8;
}
pub trait BeepTheBuzzer {}
pub trait ModelSpecificBuzzerControl {}
pub trait SpecifyBatchPrint {}
pub trait SelectBitImageMode {}
pub trait SelectDefaultLineSpacing {}
pub trait ReturnHome {}
pub trait CancelUserDefinedCharacters {}
pub trait SetHorizontalTab {}
pub trait ToggleDoubleStrikeMode {}
pub trait PrintAndReverseFeed {}
pub trait SelectPageMode {}
pub trait SelectCharacterFont {}
pub trait SelectStandardMode {}
pub trait SelectPrintDirectionInPageMode {}
pub trait ToggleUnidirectionalPrintMode {}
pub trait Toggle90ClockwiseMode {}

pub struct EUM30;
pub struct TMJ2000;
pub struct TMJ2100;
pub struct TML90;
pub struct TML90LFC;
pub struct TML100;
pub struct TMm10;
pub struct TMm30;
pub struct TMm30II;
pub struct TMm30IIH;
pub struct TMm30IINT;
pub struct TMm30IIS;
pub struct TMm30IISL;
pub struct TMm30III;
pub struct TMm30IIIH;
pub struct TMm50;
pub struct TMm50II;
pub struct TMm50IIH;
pub struct TMP20;
pub struct TMP20II;
pub struct TMP60;
pub struct TMP60II;
pub struct TMP80;
pub struct TMP80II;
pub struct TMT100;
pub struct TMT20;
pub struct TMT20II;
pub struct TMT20III;
pub struct TMT20IIIL;
pub struct TMT20X;
pub struct TMT70;
pub struct TMT70II;
pub struct TMT81III;
pub struct TMT82II;
pub struct TMT82III;
pub struct TMT82IIIL;
pub struct TMT82X;
pub struct TMT83III;
pub struct TMT88IV;
pub struct TMT88V;
pub struct TMT88VI;
pub struct TMT88VII;
pub struct TMT90;
pub struct TMU220;
pub struct TMU230;
