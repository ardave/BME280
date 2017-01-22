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

pub struct Compensations {
    dig_t1 : u16,
    dig_t2 : i16,
    dig_t3 : i16,

    dig_p1 : u16,
    dig_p2 : i16,
    dig_p3 : i16,
    dig_p4 : i16,
    dig_p5 : i16,
    dig_p6 : i16,
    dig_p7 : i16,
    dig_p8 : i16,
    dig_p9 : i16,

    dig_h1 : u8,
    dig_h2 : i16,
    dig_h3 : u8,
    dig_h7 : i8    
}

pub struct Bme280 {
}

fn load_calibration(dev: &mut LinuxI2CDevice) {
    // Still need to consider signed-ness and endianness:
    let dig_t1 = dev.smbus_read_word_data(bme280_register_dig_t1);
    let dig_t2 = dev.smbus_read_word_data(bme280_register_dig_t2);
    let dig_t3 = dev.smbus_read_word_data(bme280_register_dig_t3);

    let dig_p1 = dev.smbus_read_word_data(bme280_register_dig_p1);
    let dig_p2 = dev.smbus_read_word_data(bme280_register_dig_p2);
    let dig_p3 = dev.smbus_read_word_data(bme280_register_dig_p3);
    let dig_p4 = dev.smbus_read_word_data(bme280_register_dig_p4);
    let dig_p5 = dev.smbus_read_word_data(bme280_register_dig_p5);
    let dig_p6 = dev.smbus_read_word_data(bme280_register_dig_p6);
    let dig_p7 = dev.smbus_read_word_data(bme280_register_dig_p7);
    let dig_p8 = dev.smbus_read_word_data(bme280_register_dig_p8);
    let dig_p9 = dev.smbus_read_word_data(bme280_register_dig_p9);

    let dig_h1 = dev.smbus_read_byte_data(bme280_register_dig_h1);
    let dig_h2 = dev.smbus_read_word_data(bme280_register_dig_h2);
    let dig_h3 = dev.smbus_read_byte_data(bme280_register_dig_h3);
    let dig_h7 = dev.smbus_read_byte_data(bme280_register_dig_h7);
}

fn create(i2c_addr: u16, busnum: u8) -> Result<Bme280, LinuxI2CError> {
    let devname = format!("/dev/i2c-{}", busnum);
    let mut device = try!(LinuxI2CDevice::new(devname, i2c_addr));
    load_calibration(&mut device);
    Ok(Bme280 { })
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
