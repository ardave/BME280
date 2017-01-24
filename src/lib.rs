extern crate i2cdev;

use i2cdev::core::*;
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};

const bme280_register_dig_t1 : u8 = 0x88;  // Trimming parameter registers
const bme280_register_dig_t2 : u8 = 0x8A;
const bme280_register_dig_t3 : u8 = 0x8C; 

const bme280_register_dig_p1 : u8 = 0x8E;
const bme280_register_dig_p2 : u8 = 0x90;
const bme280_register_dig_p3 : u8 = 0x92;
const bme280_register_dig_p4 : u8 = 0x94;
const bme280_register_dig_p5 : u8 = 0x96;
const bme280_register_dig_p6 : u8 = 0x98;
const bme280_register_dig_p7 : u8 = 0x9A;
const bme280_register_dig_p8 : u8 = 0x9C;
const bme280_register_dig_p9 : u8 = 0x9E;

const bme280_register_dig_h1 : u8 = 0xA1;
const bme280_register_dig_h2 : u8 = 0xE1;
const bme280_register_dig_h3 : u8 = 0xE3;
const bme280_register_dig_h4 : u8 = 0xE4;
const bme280_register_dig_h5 : u8 = 0xE5;
const bme280_register_dig_h6 : u8 = 0xE6;
const bme280_register_dig_h7 : u8 = 0xE7;

const bme280_register_chipid : u8 = 0xD0;
const bme280_register_version : u8 = 0xD1;
const bme280_register_softreset : u8 = 0xE0;

const bme280_register_control_hum : u8 = 0xF2;
const bme280_register_control : u8 = 0xF4;
const bme280_register_config : u8 = 0xF5;
const bme280_register_pressure_data : u8 = 0xF7;
const bme280_register_temp_data : u8 = 0xFA;
const bme280_register_humidity_dat : u8 = 0xFD;

const Bme280Osample1 : u8 = 1;
const Bme280Osample2 : u8 = 2;
const Bme280Osample4 : u8 = 3;
const Bme280Osample8 : u8 = 4;
const Bme280Osample16 : u8 = 5;

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
    Device: LinuxI2CDevice,
    Mode: u8
}

impl Bme280 {
    fn read_temperature(&mut self) -> f64 { 
        self.Device.smbus_write_byte_data(bme280_register_control_hum, self.Mode);
        let meas = self.Mode << 5 | self.Mode << 2 | 1;
        self.Device.smbus_write_byte_data(bme280_register_control, meas);

        // self._device.write8(BME280_REGISTER_CONTROL, meas)
        // sleep_time = 0.00125 + 0.0023 * (1 << self._mode)
        // sleep_time = sleep_time + 0.0023 * (1 << self._mode) + 0.000575
        // sleep_time = sleep_time + 0.0023 * (1 << self._mode) + 0.000575
        // time.sleep(sleep_time)  # Wait the required time
        // msb = self._device.readU8(BME280_REGISTER_TEMP_DATA)
        // lsb = self._device.readU8(BME280_REGISTER_TEMP_DATA + 1)
        // xlsb = self._device.readU8(BME280_REGISTER_TEMP_DATA + 2)
        // raw = ((msb << 16) | (lsb << 8) | xlsb) >> 4
        // return raw
        5.55 
    }
}

fn load_calibration(dev: &mut LinuxI2CDevice) -> Result<Calibration, LinuxI2CError> {
    // Still need to consider signed-ness and endianness:
    let dig_t1 = try!(dev.smbus_read_word_data(bme280_register_dig_t1));
    let dig_t2 = try!(dev.smbus_read_word_data(bme280_register_dig_t2));
    let dig_t3 = try!(dev.smbus_read_word_data(bme280_register_dig_t3));

    let dig_p1 = try!(dev.smbus_read_word_data(bme280_register_dig_p1));
    let dig_p2 = try!(dev.smbus_read_word_data(bme280_register_dig_p2));
    let dig_p3 = try!(dev.smbus_read_word_data(bme280_register_dig_p3));
    let dig_p4 = try!(dev.smbus_read_word_data(bme280_register_dig_p4));
    let dig_p5 = try!(dev.smbus_read_word_data(bme280_register_dig_p5));
    let dig_p6 = try!(dev.smbus_read_word_data(bme280_register_dig_p6));
    let dig_p7 = try!(dev.smbus_read_word_data(bme280_register_dig_p7));
    let dig_p8 = try!(dev.smbus_read_word_data(bme280_register_dig_p8));
    let dig_p9 = try!(dev.smbus_read_word_data(bme280_register_dig_p9));

    let dig_h1 = try!(dev.smbus_read_byte_data(bme280_register_dig_h1));
    let dig_h2 = try!(dev.smbus_read_word_data(bme280_register_dig_h2));
    let dig_h3 = try!(dev.smbus_read_byte_data(bme280_register_dig_h3));
    let dig_h4 = try!(dev.smbus_read_byte_data(bme280_register_dig_h4));
    let dig_h5 = try!(dev.smbus_read_byte_data(bme280_register_dig_h5)) as i32;
    let dig_h6 = try!(dev.smbus_read_byte_data(bme280_register_dig_h6));
    let dig_h7 = try!(dev.smbus_read_byte_data(bme280_register_dig_h7));

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
    let calibration = load_calibration(&mut device);
    let maxOverSampling_and_NormalMode = 0x3F;
    device.smbus_write_byte_data(bme280_register_control, maxOverSampling_and_NormalMode);
    let mut bme280 = Bme280 { Device: device, Mode: Bme280Osample1 };
    Ok(bme280)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
