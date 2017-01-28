extern crate i2cdev;

use std::{thread, time};
use std::fmt;

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

impl fmt::Display for Calibration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(t1:{}, t2:{}, t3:{}, p1:{}, p2:{}, p3:{}, p4:{}, p5:{}, p6:{}, p7:{}, p8:{}, p9:{}, h1:{}, h2:{}, h3:{}, h7:{})", 
        self.t1, self.t2, self.t3, 
        self.p2, self.p2, self.p3, self.p4, self.p5, self.p6, self.p7, self.p8, self.p9, 
        self.h1, self.h2, self.h3, self.h7)
    }
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

    fn calc_t_fine(&mut self) -> Result<f64, LinuxI2CError> {
        let UT = try!(self.read_raw_temp());
        let t1 = self.Calibration.t1 as f64;
        let t2 = self.Calibration.t2 as f64;
        let t3 = self.Calibration.t3 as f64;
        let var1 = (UT / 16384.0 - t1 / 1024.0) * t2;
        let var2 = ((UT / 131072.0 - t1 / 8192.0) * (UT / 131072.0 - t1 / 8192.0)) * t3;
        let t_fine = (var1 + var2);
        Ok(t_fine)
    }

    pub fn read_temperature(&mut self) -> Result<f64, LinuxI2CError> {
        // Technically I'm skipping the step of casting to an integer, which would
        // result in rounding down of the var1 and var2 that were used in the original
        // calculation of t_fine:
        let temp = try!(self.calc_t_fine()) / 5120.0;
        Ok(temp)
    }

    fn read_raw_pressure(&mut self) -> Result<u32, LinuxI2CError> {
        let msb = try!(self.Device.smbus_read_byte_data(BME280_REGISTER_PRESSURE_DATA)) as u32;
        let lsb = try!(self.Device.smbus_read_byte_data(BME280_REGISTER_PRESSURE_DATA + 1)) as u32;
        let xlsb = try!(self.Device.smbus_read_byte_data(BME280_REGISTER_PRESSURE_DATA + 2)) as u32;
        let raw = ((msb << 16) | (lsb << 8) | xlsb) >> 4;
        println!("Raw is: {}", raw);
        Ok(raw)
    }

    // def read_raw_pressure(self):
    //     """Reads the raw (uncompensated) pressure level from the sensor."""
    //     """Assumes that the temperature has already been read """
    //     """i.e. that enough delay has been provided"""
    //     msb = self._device.readU8(BME280_REGISTER_PRESSURE_DATA)
    //     lsb = self._device.readU8(BME280_REGISTER_PRESSURE_DATA + 1)
    //     xlsb = self._device.readU8(BME280_REGISTER_PRESSURE_DATA + 2)
    //     raw = ((msb << 16) | (lsb << 8) | xlsb) >> 4
    //     return raw

    pub fn read_pressure(&mut self) -> Result<f64, LinuxI2CError> {
        let p1 = self.Calibration.p1 as f64;
        let p2 = self.Calibration.p2 as f64;
        let p3 = self.Calibration.p3 as f64;
        let p4 = self.Calibration.p4 as f64;
        let p5 = self.Calibration.p5 as f64;
        let p6 = self.Calibration.p6 as f64;
        let p7 = self.Calibration.p7 as f64;
        let p8 = self.Calibration.p8 as f64;
        let p9 = self.Calibration.p9 as f64;

        let adc = try!(self.read_raw_pressure()) as f64;
        let t_fine = try!(self.calc_t_fine());
        
        let var1 = t_fine / 2.0 - 64000.0;
        let var2 = var1 * var1 * p6 / 32768.0;
        let var2_2 = var2 + var1 + p5 * 2.0;
        let var2_3 = var2_2 / 4.0 + p4 * 65536.0;
        let var1_2 = (p3 * var1 * var1 / 524288.0 + p2 * var1) / 524288.0;
        let var1_3 = (1.0 + var1_2 / 32768.0) * p1;

        if var1_3 == 0.0 {
            return Ok(0.0);
        }

        let p = 1048576.0 - adc;
        let p_2 = ((p - var2_3 / 4096.0) * 6250.0) / var1_3;
        let var1_4 = p9 * p_2 * p_2 / 2147483648.0;
        let var2_4 = p_2 * p8 / 32768.0;
        let p_3 = p_2 + (var1_4 + var2_4 + p7) / 16.0;

        println!("t_fine: {}", t_fine);

        println!("var1: {}", var1);
        println!("var1_2: {}", var1_2);
        println!("var1_3: {}", var1_3);
        println!("var1_4: {}", var1_4);

        println!("var2: {}", var2);
        println!("var2_2: {}", var2_2);
        println!("var2_3: {}", var2_3);
        println!("var2_4: {}", var2_4);

        println!("p: {}", p);
        println!("p: {}", p_2);
        println!("p: {}", p_3);

        println!("Calibration: {}", self.Calibration);

        Ok(p_3)
    }

    // def read_pressure(self):
    //     """Gets the compensated pressure in Pascals."""
    //     adc = self.read_raw_pressure()
    //     var1 = self.t_fine / 2.0 - 64000.0
    //     var2 = var1 * var1 * self.dig_P6 / 32768.0
    //     var2 = var2 + var1 * self.dig_P5 * 2.0
    //     var2 = var2 / 4.0 + self.dig_P4 * 65536.0
    //     var1 = (
    //            self.dig_P3 * var1 * var1 / 524288.0 + self.dig_P2 * var1) / 524288.0
    //     var1 = (1.0 + var1 / 32768.0) * self.dig_P1
    //     if var1 == 0:
    //         return 0
    //     p = 1048576.0 - adc
    //     p = ((p - var2 / 4096.0) * 6250.0) / var1
    //     var1 = self.dig_P9 * p * p / 2147483648.0
    //     var2 = p * self.dig_P8 / 32768.0
    //     p = p + (var1 + var2 + self.dig_P7) / 16.0
    //     return p
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
