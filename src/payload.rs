use crate::color::ColorCode;
use std::path::Path;

const MAX_TEXT_LENGTH: u16 = 3000;
const ENDING_ADDRESS: u16 = 0; // ?
const MBR_COLOR_ADDRESS: u16 = 0; // ?
const UEFI_COLOR_ADDRESS: u16 = 0;
const MBR_TEXT_ADDRESS: u16 = 0;
const UEFI_TEXT_ADDRESS: u16 = 0;

#[derive(Debug)]
pub enum Ending {
    Bsod,
    Reboot,
    Nothing,
}

impl From<&str> for Ending {
    fn from(value: &str) -> Self {
        match value {
            "BSOD" => Ending::Bsod,
            "Reboot" => Ending::Reboot,
            "Nothing" => Ending::Nothing,
            _ => {
                panic!()
            }
        }
    }
}

#[derive(Debug)]
pub struct ProgramData<'data> {
    pub msg: String,
    pub color: ColorCode,
    pub ending_option: Ending,
    pub icon_path: Option<&'data Path>,
}
