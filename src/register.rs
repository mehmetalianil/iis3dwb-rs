use num_enum::TryFromPrimitive;

/// Possible I²C slave addresses.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum SlaveAddr {
    /// Default slave address (`0x18`)
    Default = 0x18,

    /// Alternate slave address (`0x19`)
    Alternate = 0x19,
}

impl SlaveAddr {
    pub fn addr(self) -> u8 {
        self as u8
    }
}

/// Enumerate all device registers.
#[allow(dead_code, non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Register {
    PIN_CTRL = 0x02,
    FIFO_CTRL_1 = 0x07,
    FIFO_CTRL_2 = 0x08,
    FIFO_CTRL_3 = 0x09,
    FIFO_CTRL_4 = 0x0A,
    COUNTER_BDR_REG1 = 0x0B,
    COUNTER_BDR_REG2 = 0x0C,
    INT1_CTRL = 0x0D,
    INT2_CTRL = 0x0E,
    WHOAMI = 0x0F,
    CTRL1_XL = 0x10,
    CTRL3_C = 0x12,
    CTRL4_C = 0x13,
    CTRL5_C = 0x14,
    CTRL6_C = 0x15,
    CTRL7_C = 0x16,
    CTRL8_XL = 0x17,
    CTRL10_C = 0x19,
    ALL_INT_SRC = 0x1A,
    WAKE_UP_SRC = 0x1B,
    STATUS_REG = 0x1E,
    OUT_TEMP_L = 0x20,
    OUT_TEMP_H = 0x21,
    OUTX_L_A = 0x28,
    OUTX_H_A = 0x29,
    OUTY_L_A = 0x2A,
    OUTY_H_A = 0x2B,
    OUTZ_L_A = 0x2C,
    OUTZ_H_A = 0x2D,
    FIFO_STATUS1 = 0x3A,
    FIFO_STATUS2 = 0x3B,
    TIMESTAMP0 = 0x40,
    TIMESTAMP1 = 0x41,
    TIMESTAMP2 = 0x42,
    TIMESTAMP3 = 0x43,
    SLOPE_EN = 0x56,
    INTERRUPTS_EN = 0x58,
    WAKE_UP_THS = 0x5B,
    WAKE_UP_DUR = 0x5C,
    MD1_CFG = 0x5E,
    MD2_CFG = 0x5F,
    INTERNAL_FREQ_FINE = 0x63,
    X_OFS_USR = 0x73,
    Y_OFS_USR = 0x74,
    Z_OFS_USR = 0x75,
    FIFO_DATA_OUT_TAG = 0x78,
    FIFO_DATA_OUT_X_L = 0x79,
    FIFO_DATA_OUT_X_H = 0x7A,
    FIFO_DATA_OUT_Y_L = 0x7B,
    FIFO_DATA_OUT_Y_H = 0x7C,
    FIFO_DATA_OUT_Z_L = 0x7D,
    FIFO_DATA_OUT_Z_H = 0x7E,
    
}

impl Register {
    /// Get register address
    pub fn addr(self) -> u8 {
        self as u8
    }

    /// Is the register read-only?
    pub fn read_only(self) -> bool {
        matches!(
            self,
            | Register::WHOAMI
            | Register::ALL_INT_SRC
            | Register::WAKE_UP_SRC
            | Register::STATUS_REG
            | Register::OUT_TEMP_L
            | Register::OUT_TEMP_H
            | Register::OUTX_L_A
            | Register::OUTX_H_A
            | Register::OUTY_L_A
            | Register::OUTY_H_A
            | Register::OUTZ_L_A
            | Register::OUTZ_H_A
            | Register::FIFO_STATUS1
            | Register::FIFO_STATUS2
            | Register::TIMESTAMP0
            | Register::TIMESTAMP1
            | Register::TIMESTAMP2
            | Register::TIMESTAMP3
            | Register::INTERNAL_FREQ_FINE
            | Register::FIFO_DATA_OUT_TAG
            | Register::FIFO_DATA_OUT_X_L
            | Register::FIFO_DATA_OUT_X_H
            | Register::FIFO_DATA_OUT_Y_L
            | Register::FIFO_DATA_OUT_Y_H
            | Register::FIFO_DATA_OUT_Z_L
            | Register::FIFO_DATA_OUT_Z_H
        )
    }
}

/// Full-scale selection.
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum Range {
    /// ±16g
    G16 = 0b11,

    /// ±8g
    G8 = 0b10,

    /// ±4g
    G4 = 0b01,

    /// ±2g (Default)
    G2 = 0b00,
}

impl Range {
    pub const fn bits(self) -> u8 {
        self as u8
    }

    /// Convert the range into an value in mili-g
    pub const fn as_mg(self) -> u8 {
        match self {
            Range::G16 => 186,
            Range::G8 => 62,
            Range::G4 => 32,
            Range::G2 => 16,
        }
    }
}

impl Default for Range {
    fn default() -> Self {
        Range::G2
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Threshold(pub(crate) u8);

impl Threshold {
    /// Convert a value in multiples of the `g` constant (roughly 9.81) to a threshold.
    ///
    ///     assert_eq!(Threshold::g(Range::G2, 1.1), 69);
    #[inline(always)]
    pub fn g(range: Range, gs: f32) -> Self {
        Self::mg(range, gs * 1000.0)
    }

    #[inline(always)]
    pub fn mg(range: Range, mgs: f32) -> Self {
        let value = mgs / (range.as_mg() as f32);
        let truncated = value as u64;

        let round_up = value - (truncated as f32) > 0.5;

        let result = if round_up { truncated + 1 } else { truncated };

        Threshold(result as u8)
    }
}

/// Output data rate.
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum DataRate {
    /// 400Hz (Default)
    Hz_400 = 0b0111,

    /// 200Hz
    Hz_200 = 0b0110,

    /// 100Hz
    Hz_100 = 0b0101,

    /// 50Hz
    Hz_50 = 0b0100,

    /// 25Hz
    Hz_25 = 0b0011,

    /// 10Hz
    Hz_10 = 0b0010,

    /// 1Hz
    Hz_1 = 0b0001,

    /// Power down
    PowerDown = 0b0000,
}

impl DataRate {
    pub const fn bits(self) -> u8 {
        self as u8
    }

    pub const fn sample_rate(self) -> f32 {
        match self {
            DataRate::Hz_400 => 400.0,
            DataRate::Hz_200 => 200.0,
            DataRate::Hz_100 => 100.0,
            DataRate::Hz_50 => 50.0,
            DataRate::Hz_25 => 25.0,
            DataRate::Hz_10 => 10.0,
            DataRate::Hz_1 => 1.0,
            DataRate::PowerDown => 0.0,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Duration(pub(crate) u8);

impl Duration {
    /// Convert a number of seconds into a duration. Internally a duration is represented
    /// as a multiple of `1 / ODR` where ODR (the output data rate) is of type [`DataRate`].
    #[inline(always)]
    pub fn seconds(output_data_rate: DataRate, seconds: f32) -> Self {
        let duration = output_data_rate.sample_rate() * seconds;

        Self(duration as u8)
    }

    /// Convert a number of miliseconds into a duration. Internally a duration is represented
    /// as a multiple of `1 / ODR` where ODR (the output data rate) is of type [`DataRate`].
    ///
    ///     assert_eq!(Duration::miliseconds(DataRate::Hz_400, 25.0), 10);
    #[inline(always)]
    pub fn miliseconds(output_data_rate: DataRate, miliseconds: f32) -> Self {
        Self::seconds(output_data_rate, miliseconds * 1000.0)
    }
}

/// Data status structure. Decoded from the `STATUS_REG` register.
///
/// `STATUS_REG` has the following bit fields:
///   * `ZYXOR` - X, Y and Z-axis data overrun
///   * `ZOR` - Z-axis data overrun
///   * `YOR` - Y-axis data overrun
///   * `XOR` - X-axis data overrun
///   * `ZYXDA` - X, Y and Z-axis new data available
///   * `ZDA` - Z-axis new data available
///   * `YDA` Y-axis new data available
///   * `XDA` X-axis new data available
///
/// This struct splits the fields into more convenient groups:
///  * `zyxor` -> `ZYXOR`
///  * `xyzor` -> (`XOR`, `YOR`, `ZOR`)
///  * `zyxda` -> `ZYXDA`
///  * `xyzda` -> (`XDA`, `YDA`, `ZDA`)
#[derive(Debug)]
pub struct DataStatus {
    /// ZYXOR bit
    pub zyxor: bool,

    /// (XOR, YOR, ZOR) bits
    pub xyzor: (bool, bool, bool),

    /// ZYXDA bit
    pub zyxda: bool,

    /// (XDA, YDA, ZDA) bits
    pub xyzda: (bool, bool, bool),
}

/// Operating mode.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Mode {
    /// High-resolution mode (12-bit data output)
    HighResolution,

    /// Normal mode (10-bit data output)
    Normal,

    /// Low-power mode (8-bit data output)
    LowPower,
}

// === WHO_AMI_I (0Fh) ===

/// `WHO_AM_I` device identification register
pub const DEVICE_ID: u8 = 0x33;

// === TEMP_CFG_REG (1Fh) ===

pub const ADC_EN: u8 = 0b1000_0000;
pub const TEMP_EN: u8 = 0b0100_0000;

// === CTRL_REG1 (20h) ===

pub const ODR_MASK: u8 = 0b1111_0000;
pub const LP_EN: u8 = 0b0000_1000;
pub const Z_EN: u8 = 0b0000_0100;
pub const Y_EN: u8 = 0b0000_0010;
pub const X_EN: u8 = 0b0000_0001;

// === CTRL_REG4 (23h) ===

pub const BDU: u8 = 0b1000_0000;
pub const FS_MASK: u8 = 0b0011_0000;
pub const HR: u8 = 0b0000_1000;

// === STATUS_REG (27h) ===

pub const ZYXOR: u8 = 0b1000_0000;
pub const ZOR: u8 = 0b0100_0000;
pub const YOR: u8 = 0b0010_0000;
pub const XOR: u8 = 0b0001_0000;
pub const ZYXDA: u8 = 0b0000_1000;
pub const ZDA: u8 = 0b0000_0100;
pub const YDA: u8 = 0b0000_0010;
pub const XDA: u8 = 0b0000_0001;
