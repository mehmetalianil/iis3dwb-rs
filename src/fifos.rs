use crate::register::*;


// 
// In IIS3DWB, Interrupts are distinct from Wake up sources
// Therefore, we need to implement a wake up lib. 
// 


// Is this crazy? W8 is at one register and W[7:0] at another.
pub struct Watermark{
    lsb: u8,
    hsb: bool,
}

// Cannot be higher than the FIFO size, 3000 bytes.
impl Watermark {
    pub fn from_bytes(watermark_input: u16){
        
    }
}


#[derive(Copy, Clone, Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum FifoMode{
    Disabled = 0b000,
    FifoMode = 0b001,
    ContinuousToFifo = 0b011,
    BypassToContinuous = 0b100,
    Continuous = 0b110,
    BypassToFifo = 0b111,
}

#[derive(Debug, Copy, Clone)]
pub struct Fifo
{
    enabled: bool,
    mode: FifoMode,
    watermark:Watermark,
    stop_in_watermark: bool,
}

impl Default for Fifo {
    fn default() -> Self {
        Self{
            enabled:false,
            mode: FifoMode::Disabled,
            watermark:0,
            stop_in_watermark:false,
        }
    }
}

impl Fifo {
    
   
}


