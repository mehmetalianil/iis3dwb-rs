use crate::register::*;


// 
// In IIS3DWB, Interrupts are distinct from Wake up sources
// Therefore, we need to implement a wake up lib. 
// 

#[derive(Debug, Copy, Clone)]
pub struct Interrupt1;

#[derive(Debug, Copy, Clone)]
pub struct Interrupt2;

pub trait Interrupt {
    fn ctrl_reg() -> Register;
    fn ths_reg() -> Register;
    fn src_reg() -> Register;
    fn duration_reg() -> Register;
    fn lir_int_bit() -> u8;
    fn d4d_int_bit() -> u8;
}

impl Interrupt for Interrupt1 {
}

impl Interrupt for Interrupt2 {
}

/// When to generate an interrupt.
///
/// Internal representation:
///
/// | AOI | 6D | Interrupt mode |
/// | - | - | --- |
/// | 0 | 0 | OR combination of interrupt events  |
/// | 0 | 1 | 6-direction movement recognition  |
/// | 1 | 0 | AND combination of interrupt events  |
/// | 1 | 1 | 6-direction position recognition  |

/// Configure which events on which axes trigger an interrupt.
#[derive(Debug, Copy, Clone, Default)]
#[doc(alias = "INT1_CFG")]
#[doc(alias = "INT2_CFG")]
pub struct InterruptConfig {
    pub z_axis_high: bool,
    pub z_axis_low: bool,

    pub y_axis_high: bool,
    pub y_axis_low: bool,

    pub x_axis_high: bool,
    pub x_axis_low: bool,
}
