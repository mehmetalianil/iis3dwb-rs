mod register;

use Register::*; 

pub enum OffsetWeight{
    TenthBitIsOneG = 0b0,
    SixthBitInOneG = 0b1,
}

pub struct UserOffset(u8);
impl UserOffset{
    fn from_mg(offsetweight: OffsetWeight, offset: u16){

    }
}