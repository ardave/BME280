use std::{thread, time};
use i2cdev::core::I2CDevice;
use i2cdev::linux::{LinuxI2CError};

use super::calibration::Calibration;

pub struct Bme280<'a, T: I2CDevice<Error=LinuxI2CError> + Sized + 'a> {
    calibration: Calibration,
    device: &'a mut T,
    mode: u8
}

impl<'a, T: I2CDevice<Error=LinuxI2CError> + Sized + 'a> Bme280<'a, T> {

    pub fn new(dev: &'a mut T) -> Result<Bme280<'a, T>, LinuxI2CError> {
        let cal = try!(Bme280::get_calibration(dev));
        try!(dev.smbus_write_byte_data(BME280_REGISTER_CONTROL, MAX_OVER_SAMPLING_AND_NORMAL_MODE));
        Ok(Bme280 { calibration: cal, device: dev, mode: BME280OSAMPLE1 })
    }

    pub fn print_calibration(&mut self) {
        println!("{}", self.calibration);
    }

    fn get_calibration(dev: &mut T) -> Result<Calibration, LinuxI2CError> {
        // Still need to consider signed-ness and endianness:
        let dig_t1 = try!(dev.smbus_read_word_data(BME280_REGISTER_DIG_T1));
        let dig_t2 = try!(dev.smbus_read_word_data(BME280_REGISTER_DIG_T2));
        let dig_t3 = try!(dev.smbus_read_word_data(BME280_REGISTER_DIG_T3));

        let dig_p1 = try!(dev.smbus_read_word_data(BME280_REGISTER_DIG_P1));
        let dig_p2 = try!(dev.smbus_read_word_data(BME280_REGISTER_DIG_P2)) as i16;
        let dig_p3 = try!(dev.smbus_read_word_data(BME280_REGISTER_DIG_P3));
        let dig_p4 = try!(dev.smbus_read_word_data(BME280_REGISTER_DIG_P4));
        let dig_p5 = try!(dev.smbus_read_word_data(BME280_REGISTER_DIG_P5));
        let dig_p6 = try!(dev.smbus_read_word_data(BME280_REGISTER_DIG_P6)) as i16;
        let dig_p7 = try!(dev.smbus_read_word_data(BME280_REGISTER_DIG_P7));
        let dig_p8 = try!(dev.smbus_read_word_data(BME280_REGISTER_DIG_P8)) as i16;
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

    fn read_raw_temp(&mut self) -> Result<f64, LinuxI2CError> { 
        try!(self.device.smbus_write_byte_data(BME280_REGISTER_CONTROL_HUM, self.mode));
        let meas = self.mode << 5 | self.mode << 2 | 1;
        try!(self.device.smbus_write_byte_data(BME280_REGISTER_CONTROL, meas));
        let mut sleep_time = 0.00125 + 0.0023 * (1 << self.mode) as f32;
        sleep_time = sleep_time + 0.0023 * (1 << self.mode) as f32 + 0.000575;
        sleep_time = sleep_time + 0.0023 * (1 << self.mode) as f32 + 0.000575;
        let dur = time::Duration::from_millis((sleep_time * 1000.0) as u64);
        thread::sleep(dur);
        let msb = try!(self.device.smbus_read_byte_data(BME280_REGISTER_TEMP_DATA)) as u64;
        let lsb = try!(self.device.smbus_read_byte_data(BME280_REGISTER_TEMP_DATA + 1)) as u64;
        let xlsb = try!(self.device.smbus_read_byte_data(BME280_REGISTER_TEMP_DATA + 2)) as u64;
        let raw = ((msb << 16) | (lsb << 8) | xlsb) >> 4;
        Ok(raw as f64)
    }

    fn calc_t_fine(&mut self) -> Result<f64, LinuxI2CError> {
        let ut = try!(self.read_raw_temp());
        let t1 = self.calibration.t1 as f64;
        let t2 = self.calibration.t2 as f64;
        let t3 = self.calibration.t3 as f64;
        let var1 = (ut / 16384.0 - t1 / 1024.0) * t2;
        let var2 = ((ut / 131072.0 - t1 / 8192.0) * (ut / 131072.0 - t1 / 8192.0)) * t3;
        let t_fine = var1 + var2;
        Ok(t_fine)
    }

    pub fn read_temperature(&mut self) -> Result<f64, LinuxI2CError> {
        // Technically I'm skipping the step of casting to an integer, which would
        // result in rounding down of the var1 and var2 that were used in the original
        // calculation of t_fine:
        let celcius = try!(self.calc_t_fine()) / 5120.0;
        let fahrenheit = celcius * 1.8 + 32.0;
        Ok(fahrenheit)
    }

    fn read_raw_pressure(&mut self) -> Result<u32, LinuxI2CError> {
        let msb = try!(self.device.smbus_read_byte_data(BME280_REGISTER_PRESSURE_DATA)) as u32;
        let lsb = try!(self.device.smbus_read_byte_data(BME280_REGISTER_PRESSURE_DATA + 1)) as u32;
        let xlsb = try!(self.device.smbus_read_byte_data(BME280_REGISTER_PRESSURE_DATA + 2)) as u32;
        let raw = ((msb << 16) | (lsb << 8) | xlsb) >> 4;
        // println!("Raw is: {}", raw);
        Ok(raw)
    }

    pub fn read_pressure(&mut self) -> Result<f64, LinuxI2CError> {
        let p1 = self.calibration.p1 as f64;
        let p2 = self.calibration.p2 as f64;
        let p3 = self.calibration.p3 as f64;
        let p4 = self.calibration.p4 as f64;
        let p5 = self.calibration.p5 as f64;
        let p6 = self.calibration.p6 as f64;
        let p7 = self.calibration.p7 as f64;
        let p8 = self.calibration.p8 as f64;
        let p9 = self.calibration.p9 as f64;

        let adc = try!(self.read_raw_pressure()) as f64;
        let t_fine = try!(self.calc_t_fine());
        // println!("t_fine: {}", t_fine);
        
        let var1 = t_fine / 2.0 - 64000.0;
        // println!("var1: {}", var1);

        let var2 = var1 * var1 * p6 / 32768.0;
        // println!("var2: {}", var2);

        let var2_2 = var2 + var1 * p5 * 2.0;
        // println!("var2_2: {}", var2_2);
    
        let var2_3 = var2_2 / 4.0 + p4 * 65536.0;
        // println!("var2_3: {}", var2_3);

        let var1_2 = (p3 * var1 * var1 / 524288.0 + p2 * var1) / 524288.0;
        // println!("var1_2: {}", var1_2);

        let var1_3 = (1.0 + var1_2 / 32768.0) * p1;
        // println!("var1_3: {}", var1_3);

        if var1_3 == 0.0 {
            return Ok(0.0);
        }

        let p = 1048576.0 - adc;
        // println!("p: {}", p);

        let p_2 = ((p - var2_3 / 4096.0) * 6250.0) / var1_3;
        // println!("p_2: {}", p_2);

        let var1_4 = p9 * p_2 * p_2 / 2147483648.0;
        // println!("var1_4: {}", var1_4);

        let var2_4 = p_2 * p8 / 32768.0;
        // println!("var2_4: {}", var2_4);

        let pascals = p_2 + (var1_4 + var2_4 + p7) / 16.0;
        // println!("pascals: {}", pascals);

        // println!("Calibration: {}", self.Calibration);

        let in_hg = pascals *  0.000295299830714;

        Ok(in_hg)
    }    
}

pub const BME280_REGISTER_DIG_T1 : u8 = 0x88;  // Trimming parameter registers
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

const MAX_OVER_SAMPLING_AND_NORMAL_MODE : u8 = 0x3F;

#[cfg(test)]
mod tests {

    use i2cdev::core::I2CDevice;
    use i2cdev::linux::{LinuxI2CError};
    use std::io::{Error, ErrorKind};
    use nix;
    use bme280;

    struct FakeDevice {

    }

    impl I2CDevice for FakeDevice {
        type Error = LinuxI2CError;

        /// Read data from the device to fill the provided slice
        fn read(&mut self, data: &mut [u8]) -> Result<(), Self::Error> {
            Ok(())
        }

        /// Write the provided buffer to the device
        fn write(&mut self, data: &[u8]) -> Result<(), Self::Error> {
            Ok(())
        }

        /// This sends a single bit to the device, at the place of the Rd/Wr bit
        fn smbus_write_quick(&mut self, bit: bool) -> Result<(), Self::Error> {
            Ok(())
        }

        /// Read a block of up to 32 bytes from a device
        ///
        /// The actual number of bytes available to read is returned in the count
        /// byte.  This code returns a correctly sized vector containing the
        /// count bytes read from the device.
        fn smbus_read_block_data(&mut self, register: u8) -> Result<Vec<u8>, Self::Error> {
            Ok(vec![1,2,3])
        }

        /// Read a block of up to 32 bytes from a device
        ///
        /// Uses read_i2c_block_data instead read_block_data.
        fn smbus_read_i2c_block_data(&mut self, register: u8, len: u8) -> Result<Vec<u8>, Self::Error> {
            Ok(vec![1,2,3])
        }

        /// Write a block of up to 32 bytes to a device
        ///
        /// The opposite of the Block Read command, this writes up to 32 bytes to
        /// a device, to a designated register that is specified through the
        /// Comm byte. The amount of data is specified in the Count byte.
        fn smbus_write_block_data(&mut self, register: u8, values: &[u8]) -> Result<(), Self::Error> {
            Ok(())
        }

        /// Select a register, send 1 to 31 bytes of data to it, and reads
        /// 1 to 31 bytes of data from it.
        fn smbus_process_block(&mut self, register: u8, values: &[u8]) -> Result<(), Self::Error> {
            Ok(())
        }
    
        /// Read 2 bytes form a given register on a device
        fn smbus_read_word_data(&mut self, register: u8) -> Result<u16, LinuxI2CError> {
            if register == bme280::BME280_REGISTER_DIG_T1 {
                return Ok(28960)
            }

            let nix = LinuxI2CError::Nix(nix::Error::InvalidPath);
            Err(nix)
        }

        // test pressure_reading_should_be_reasonable ... The pressure is: 29.61 in hg.
        // test print_the_calibration ... (t1:28960, t2:26619, t3:50, p1:-10713, p2:-10713, p3:3024, p4:5831, p5:96, p6:-7, p7:9900, p8:-10230, p9:4285, h1:75, h2:355, h3:0, h7:30)
        // test temperature_reading_should_be_reasonable ... The temperature is: 67.03


    }

    #[test]
    fn fn_set_of_known_calibration_values_should_yield_known_temperature() {
    }
}