extern crate i2cdev;

use std::{thread, time};

use i2cdev::core::*;
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};

const BME280_REGISTER_DIG_T1 : u8 = 0x88;  // Trimming parameter registers
const BME280_REGISTER_DIG_T2 : u8 = 0x8A;
const BME280_REGISTER_DIG_T3 : u8 = 0x8C; 

const BME280_REGISTER_DIG_P1 : u8 = 0x8E;
const BME280_REGISTER_DIG_P2 : u8 = 0x90;
const BME280_REGISTER_DIG_P3 : u8 = 0x92;
const BME280_REGISTER_DIG_P4 : u8 = 0x94;
const BME280_REGISTER_DIG_P5 : u8 = 0x96;
const BME280_REGISTER_DIG_P6 : u8 = 0x98;
const BME280_REGISTER_DIG_P7 : u8 = 0x9A;
const BME280_REGISTER_DIG_P8 : u8 = 0x9C;
const BME280_REGISTER_DIG_P9 : u8 = 0x9E;

const BME280_REGISTER_DIG_H1 : u8 = 0xA1;
const BME280_REGISTER_DIG_H2 : u8 = 0xE1;
const BME280_REGISTER_DIG_H3 : u8 = 0xE3;
const BME280_REGISTER_DIG_H4 : u8 = 0xE4;
const BME280_REGISTER_DIG_H5 : u8 = 0xE5;
const BME280_REGISTER_DIG_H6 : u8 = 0xE6;
const BME280_REGISTER_DIG_H7 : u8 = 0xE7;

const BME280_REGISTER_CHIPID : u8 = 0xD0;
const BME280_REGISTER_VERSION : u8 = 0xD1;
const BME280_REGISTER_SOFTRESET : u8 = 0xE0;

const BME280_REGISTER_CONTROL_HUM : u8 = 0xF2;
const BME280_REGISTER_CONTROL : u8 = 0xF4;
const BME280_REGISTER_CONFIG : u8 = 0xF5;
const BME280_REGISTER_PRESSURE_DATA : u8 = 0xF7;
const BME280_REGISTER_TEMP_DATA : u8 = 0xFA;
const BME280_REGISTER_HUMIDITY_DAT : u8 = 0xFD;

const BME280OSAMPLE1 : u8 = 1;
const BME280OSAMPLE2 : u8 = 2;
const BME280OSAMPLE4 : u8 = 3;
const BME280OSAMPLE8 : u8 = 4;
const BME280OSAMPLE16 : u8 = 5;

pub struct Calibration {
    // Still need to consider signed-ness and endianness:
    t1 : u16,
    t2 : u16,
    t3 : u16,

    p1 : u16,
    p2 : u16,
    p3 : u16,
    p4 : u16,
    p5 : u16,
    p6 : u16,
    p7 : u16,
    p8 : u16,
    p9 : u16,

    h1 : u8,
    h2 : u16,
    h3 : u8,
    h7 : u8    
}

pub struct Bme280 {
    Calibration: Calibration,
    Device: LinuxI2CDevice,
    Mode: u8
}

impl Bme280 {
    fn read_raw_temp(&mut self) -> Result<f64, LinuxI2CError> { 
        self.Device.smbus_write_byte_data(BME280_REGISTER_CONTROL_HUM, self.Mode);
        let meas = self.Mode << 5 | self.Mode << 2 | 1;
        self.Device.smbus_write_byte_data(BME280_REGISTER_CONTROL, meas);
        let mut sleep_time = 0.00125 + 0.0023 * (1 << self.Mode) as f32;
        sleep_time = sleep_time + 0.0023 * (1 << self.Mode) as f32 + 0.000575;
        sleep_time = sleep_time + 0.0023 * (1 << self.Mode) as f32 + 0.000575;
        let dur = time::Duration::from_millis((sleep_time * 1000.0) as u64);
        thread::sleep(dur);
        let msb = try!(self.Device.smbus_read_byte_data(BME280_REGISTER_TEMP_DATA)) as u64;
        let lsb = try!(self.Device.smbus_read_byte_data(BME280_REGISTER_TEMP_DATA + 1)) as u64;
        let xlsb = try!(self.Device.smbus_read_byte_data(BME280_REGISTER_TEMP_DATA + 2)) as u64;
        let raw = ((msb << 16) | (lsb << 8) | xlsb) >> 4;
        Ok(raw as f64)
    }

    fn read_temperature(&mut self) -> Result<f64, LinuxI2CError> {
        let UT = try!(self.read_raw_temp());
        let t1 = self.Calibration.t1 as f64;
        let t2 = self.Calibration.t2 as f64;
        let t3 = self.Calibration.t3 as f64;
        let var1 = (UT / 16384.0 - t1 / 1024.0) * t2;
        let var2 = ((UT / 131072.0 - t1 / 8192.0) * (UT / 131072.0 - t1 / 8192.0)) * t3 ;
        let t_fine = (var1 + var2) as i32;
        let temp = (var1 + var2) / 5120.0;
        Ok(temp)
    }
}

fn load_calibration(dev: &mut LinuxI2CDevice) -> Result<Calibration, LinuxI2CError> {
    // Still need to consider signed-ness and endianness:
    let dig_t1 = try!(dev.smbus_read_word_data(BME280_REGISTER_DIG_T1));
    let dig_t2 = try!(dev.smbus_read_word_data(BME280_REGISTER_DIG_T2));
    let dig_t3 = try!(dev.smbus_read_word_data(BME280_REGISTER_DIG_T3));

    let dig_p1 = try!(dev.smbus_read_word_data(BME280_REGISTER_DIG_P1));
    let dig_p2 = try!(dev.smbus_read_word_data(BME280_REGISTER_DIG_P2));
    let dig_p3 = try!(dev.smbus_read_word_data(BME280_REGISTER_DIG_P3));
    let dig_p4 = try!(dev.smbus_read_word_data(BME280_REGISTER_DIG_P4));
    let dig_p5 = try!(dev.smbus_read_word_data(BME280_REGISTER_DIG_P5));
    let dig_p6 = try!(dev.smbus_read_word_data(BME280_REGISTER_DIG_P6));
    let dig_p7 = try!(dev.smbus_read_word_data(BME280_REGISTER_DIG_P7));
    let dig_p8 = try!(dev.smbus_read_word_data(BME280_REGISTER_DIG_P8));
    let dig_p9 = try!(dev.smbus_read_word_data(BME280_REGISTER_DIG_P9));

    let dig_h1 = try!(dev.smbus_read_byte_data(BME280_REGISTER_DIG_H1));
    let dig_h2 = try!(dev.smbus_read_word_data(BME280_REGISTER_DIG_H2));
    let dig_h3 = try!(dev.smbus_read_byte_data(BME280_REGISTER_DIG_H3));
    let dig_h4 = try!(dev.smbus_read_byte_data(BME280_REGISTER_DIG_H4));
    let dig_h5 = try!(dev.smbus_read_byte_data(BME280_REGISTER_DIG_H5)) as i32;
    let dig_h6 = try!(dev.smbus_read_byte_data(BME280_REGISTER_DIG_H6));
    let dig_h7 = try!(dev.smbus_read_byte_data(BME280_REGISTER_DIG_H7));

    Ok(Calibration {
        t1: dig_t1,
        t2: dig_t2,
        t3: dig_t3,

        p1: dig_p1,
        p2: dig_p2,
        p3: dig_p3,
        p4: dig_p4,
        p5: dig_p5,
        p6: dig_p6,
        p7: dig_p7,
        p8: dig_p8,
        p9: dig_p9,

        h1: dig_h1,
        h2: dig_h2,
        h3: dig_h3,
        h7: dig_h7
    })
}

pub fn create(i2c_addr: u16, busnum: u8) -> Result<Bme280, LinuxI2CError> {
    let devname = format!("/dev/i2c-{}", busnum);
    let mut device = try!(LinuxI2CDevice::new(devname, i2c_addr));
    let calibration = try!(load_calibration(&mut device));
    let maxOverSampling_and_NormalMode = 0x3F;
    device.smbus_write_byte_data(BME280_REGISTER_CONTROL, maxOverSampling_and_NormalMode);
    let mut bme280 = Bme280 { Device: device, Mode: BME280OSAMPLE1, Calibration: calibration };
    Ok(bme280)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
