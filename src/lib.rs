#![no_std]

mod register;
mod interrupts;
mod wakeups;

/// Is there a way to improve this to non blocking? Does that require async? 
use embedded_hal::blocking::spi;

use embedded_hal::digital::v2::OutputPin;

pub use accelerometer::{
    Accelerometer,
    RawAccelerometer,
    error,
    Error,
    vector::{I16x3,F32x3,I32x3}
};

// use interrupts::{Interrupt1, Interrupt2};
// use wakeups::{WakeUp};
pub use register::{DataRate, Mode, Range, Register};
use register::{DEVICE_ID,
               XL_EN_MASK,
               FS_EN_MASK,};

use core::fmt::Debug;


const SPI_READ: u8 = 0b1000_0000;
const SPI_WRITE: u8 = 0x0000_0000;

pub struct Config {
    pub mode: Mode,
    pub datarate: DataRate,
    pub enable_x_axis: bool,   
    pub enable_y_axis: bool,   
    pub enable_z_axis: bool,   
    pub enable_temp:bool,
    pub range: Range,
    // pub interrupt1: Range,
    // pub interrupt2: Range,
    // pub wake_up: WakeUp,
}
 
impl Default for Config{
    fn default() -> Self {
        Self {
            mode: Mode::HighResolution,
            datarate: DataRate::Hz_26700,
            enable_x_axis: true,   
            enable_y_axis: true,   
            enable_z_axis: true,   
            enable_temp: true,
            range: Range::G2,
            // interrupt1: Interrupt1::None,
            // interrupt2: Interrupt2::None,
            // wake_up: WakeUp::None,
        }
    }
}

/// Driver's structure
pub struct IIS3DWB <SPI, CS>{
    spi : SPI,
    cs  : CS,
    // configuration 
    range : Range,
    // interrupt1: Interrupt1,
    // interrupt2: Interrupt2,
    // wake_up: WakeUp,
}

/// Driver's implementation for given SPI and CS
impl<SPI, CS, E, PinError> IIS3DWB<SPI,CS>
where 
    SPI: spi::Transfer<u8, Error=E> + spi::Write<u8, Error=E>,
    CS: OutputPin <Error= PinError>
{
    pub fn new(spi: SPI, cs: CS, config: &Config) -> Result<Self,E> {
        let mut iis3dwb = IIS3DWB {
            spi,
            cs, 
            range: config.range,
            // interrupt1: config.interrupt1,
            // interrupt2: config.interrupt2,
            // wake_up: config.wake_up
        };

        let id= iis3dwb.get_device_id();
        if id != DEVICE_ID {
            // raise
        }
        // set_range
        iis3dwb.set_range(iis3dwb.range);
        Ok(iis3dwb)
    }  

    pub fn start(&mut self) {
        self.modify_register( Register::CTRL1_XL.addr(), 
                             XL_EN_MASK, 
                              0b101);
    }

    pub fn set_range(&mut self, range: Range) {
        self.modify_register( Register::CTRL1_XL.addr(), 
                              FS_EN_MASK, 
                              range.bits());
    }

    /// Get the device ID
    pub fn get_device_id(&mut self) -> u8 {
        let reg = Register::WHO_AM_I.addr();
        let mut output = [1u8];
        self.read_reg(reg, &mut output);
        output[0]
    }

      /// Returns the raw contents of the temperature registers
    pub fn read_temp_raw(&mut self) -> u16 {
        let mut bytes = [Register::OUT_TEMP_H.addr() | SPI_READ, 0, 0];
        self.read(&mut bytes);

        let temp_h = ((bytes[1] & 0x0F) as u16) << 8;
        let temp_l = (bytes[2] as u16) & 0x00FF;

        temp_h | temp_l
    }

    fn write_reg(&mut self, reg: u8, value: u8) {
        let mut bytes = [ reg | SPI_WRITE, value];
        self.cs.set_low().ok();
        self.spi.write(&mut bytes).ok();
        self.cs.set_high().ok();
    }

    fn read_reg(&mut self, reg: u8, buffer: &mut [u8]) {
        let mut bytes = [ reg | SPI_READ, 0];
        self.cs.set_low().ok();
        self.spi.transfer(&mut bytes).ok();
        self.cs.set_high().ok();
        buffer[0] = bytes[1];
    }

    fn read(&mut self, bytes: &mut [u8]) {
        self.cs.set_low().ok();
        self.spi.transfer(bytes).ok();
        self.cs.set_high().ok();
    }

    fn read_consecutive_regs(){
        unimplemented!();
    }

    /// Ask TweedeGolf about their implementation, why pass function?
    /// I chose the simplistic way, since this is my first rs dd.
    /// 
    /// Also, test this, it is a slippery slope.
    fn modify_register(&mut self, reg: u8, mask: u8, val: u8) -> Result<(),()>
    {
        if mask == 0 {return Err(());}
        let mut register_value=[0u8]; 
        self.read_reg( reg,&mut register_value);
        let mut bitshift = 0u8;
        let mut sacrificial_mask = mask.clone();
        //ugly dangerous loop calculating shifts 
        while sacrificial_mask != 0 {
            if sacrificial_mask & 0x01 == 0x01 { break; }
            else{
                sacrificial_mask = sacrificial_mask >> 1;
                bitshift += 1;
            }
        }
        // 0x0010_0110 <- init
        // 0x0111_0000 <- mask
        // 0x0000_0101 <- value
        // 
        // clear init with clear mask
        // 0x1000_1111 & 0x0010_0110 = 0x0000_0110
        // 
        // set init with set mask
        // 0x0000_0110 | (val << 4) which is 0b0101_0000 = 0b0101_0110
        let clear_mask = !mask;
        let set_mask = val << bitshift;
        register_value[0] = (register_value[0] & clear_mask) | set_mask;
        self.write_reg( reg, register_value[0]);
        Ok(())  
    }    
}

impl<SPI, CS, E, PinError> RawAccelerometer<I16x3> for IIS3DWB <SPI, CS>
where
    SPI: spi::Transfer<u8, Error=E> + spi::Write<u8, Error=E>,
    CS: OutputPin<Error = PinError>,
    E: Debug
{
    type Error = E;

    /// Gets acceleration vector reading from the accelerometer
    /// Returns a 3D vector with x,y,z, fields in a Result
    fn accel_raw(&mut self) -> Result<I16x3, Error<E>> {
        let mut bytes = [0u8; 6+1];
        bytes[0] = Register::OUTX_L_A.addr() | SPI_READ;
        self.read(&mut bytes);

        // TEST THIS 
        let x = bytes[1] as i16 + (bytes[2] as i16) <<8;
        let y = bytes[3] as i16 + (bytes[4] as i16) <<8;
        let z = bytes[5] as i16 + (bytes[6] as i16) <<8;

        Ok(I16x3::new(x, y, z))
    }

}