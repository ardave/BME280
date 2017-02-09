pub enum Register {
    T1 = 0x88,
    T2  = 0x8A,
    T3 = 0x8C,

    P1  = 0x8E,
    P2  = 0x90,
    P3  = 0x92,
    P4  = 0x94,
    P5  = 0x96,
    P6  = 0x98,
    P7 = 0x9A,
    P8  = 0x9C,
    P9  = 0x9E,

    H1  = 0xA1,
    H2  = 0xE1,
    H3 = 0xE3,
    H4  = 0xE4,
    H5  = 0xE5,
    H6  = 0xE6,
    H7  = 0xE7,

    CHIPID = 0xD0,
    VERSION  = 0xD1,
    SOFTRESET  = 0xE0,

    CONTROL_HUM  = 0xF2,
    CONTROL  = 0xF4,
    CONFIG = 0xF5,
    PRESSURE_DATA = 0xF7,
    PRESSURE_DATA_1 = 0xF7 + 1,
    PRESSURE_DATA_2 = 0xF7 + 2,
    TEMP_DATA = 0xFA,
    TEMP_DATA_1 = 0xFA + 1,
    TEMP_DATA_2 = 0xFA + 2,
    HUMIDITY_DAT  = 0xFD
}

// pub const BME280_REGISTER_DIG_T1 : u8 = 0x88;  // Trimming parameter registers
// const BME280_REGISTER_DIG_T2 : u8 = 0x8A;
// const BME280_REGISTER_DIG_T3 : u8 = 0x8C; 

// const BME280_REGISTER_DIG_P1 : u8 = 0x8E;
// const BME280_REGISTER_DIG_P2 : u8 = 0x90;
// const BME280_REGISTER_DIG_P3 : u8 = 0x92;
// const BME280_REGISTER_DIG_P4 : u8 = 0x94;
// const BME280_REGISTER_DIG_P5 : u8 = 0x96;
// const BME280_REGISTER_DIG_P6 : u8 = 0x98;
// const BME280_REGISTER_DIG_P7 : u8 = 0x9A;
// const BME280_REGISTER_DIG_P8 : u8 = 0x9C;
// const BME280_REGISTER_DIG_P9 : u8 = 0x9E;

// const BME280_REGISTER_DIG_H1 : u8 = 0xA1;
// const BME280_REGISTER_DIG_H2 : u8 = 0xE1;
// const BME280_REGISTER_DIG_H3 : u8 = 0xE3;
// const BME280_REGISTER_DIG_H4 : u8 = 0xE4;
// const BME280_REGISTER_DIG_H5 : u8 = 0xE5;
// const BME280_REGISTER_DIG_H6 : u8 = 0xE6;
// const BME280_REGISTER_DIG_H7 : u8 = 0xE7;

// const BME280_REGISTER_CHIPID : u8 = 0xD0;
// const BME280_REGISTER_VERSION : u8 = 0xD1;
// const BME280_REGISTER_SOFTRESET : u8 = 0xE0;

// const BME280_REGISTER_CONTROL_HUM : u8 = 0xF2;
// const BME280_REGISTER_CONTROL : u8 = 0xF4;
// const BME280_REGISTER_CONFIG : u8 = 0xF5;
// const BME280_REGISTER_PRESSURE_DATA : u8 = 0xF7;
// const BME280_REGISTER_TEMP_DATA : u8 = 0xFA;
// const BME280_REGISTER_HUMIDITY_DAT : u8 = 0xFD;