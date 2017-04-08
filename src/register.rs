use std::fmt::Display;
use std::fmt::Result;
use std::fmt::Formatter;

/// Enum mapping sensor hex addresses to human-readable values.
#[derive(Debug)]
pub enum Register {
    T1 = 0x88,
    T2 = 0x8A,
    T3 = 0x8C,

    P1 = 0x8E,
    P2 = 0x90,
    P3 = 0x92,
    P4 = 0x94,
    P5 = 0x96,
    P6 = 0x98,
    P7 = 0x9A,
    P8 = 0x9C,
    P9 = 0x9E,

    H1 = 0xA1,
    H2 = 0xE1,
    H3 = 0xE3,
    H4 = 0xE4,
    H5 = 0xE5,
    H6 = 0xE6,
    H7 = 0xE7,

    ChipId = 0xD0,
    Version = 0xD1,
    SoftReset = 0xE0,

    ControlHum = 0xF2,
    Control = 0xF4,
    Config = 0xF5,
    PressureData = 0xF7,
    PressureData1 = 0xF7 + 1,
    PressureData2 = 0xF7 + 2,
    TemperatureData = 0xFA,
    TemperatureData1 = 0xFA + 1,
    TemperatureData2 = 0xFA + 2,
    HumidityData = 0xFD,
    HumidityData1 = 0xFD + 1,
}

impl Display for Register {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}
