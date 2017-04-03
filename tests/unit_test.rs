extern crate nix;
extern crate i2cdev;
extern crate bme280;

use i2cdev::core::I2CDevice;
use i2cdev::linux::LinuxI2CError;
use std::io::{Error, ErrorKind};
use bme280::bme280::Bme280;
use bme280::register::Register;

struct FakeDevice {}

impl I2CDevice for FakeDevice {
    type Error = LinuxI2CError;

    #[warn(unused_variables)]
    fn read(&mut self, data: &mut [u8]) -> Result<(), Self::Error> {
        Ok(())
    }

    #[warn(unused_variables)]
    fn write(&mut self, data: &[u8]) -> Result<(), Self::Error> {
        Ok(())
    }

    #[warn(unused_variables)]
    fn smbus_write_quick(&mut self, bit: bool) -> Result<(), Self::Error> {
        Ok(())
    }

    #[warn(unused_variables)]
    fn smbus_read_block_data(&mut self, register: u8) -> Result<Vec<u8>, Self::Error> {
        Ok(vec![1, 2, 3])
    }

    #[warn(unused_variables)]
    fn smbus_read_i2c_block_data(&mut self, register: u8, len: u8) -> Result<Vec<u8>, Self::Error> {
        Ok(vec![1, 2, 3])
    }

    #[warn(unused_variables)]
    fn smbus_write_block_data(&mut self, register: u8, values: &[u8]) -> Result<(), Self::Error> {
        Ok(())
    }

    #[warn(unused_variables)]
    fn smbus_process_block(&mut self, register: u8, values: &[u8]) -> Result<(), Self::Error> {
        Ok(())
    }

    fn smbus_read_word_data(&mut self, register: u8) -> Result<u16, LinuxI2CError> {
        match register {
            x if x == Register::T1 as u8 => Ok(28960),
            x if x == Register::T2 as u8 => Ok(26619),
            x if x == Register::T3 as u8 => Ok(50),

            x if x == Register::P1 as u8 => Ok(34988),
            x if x == Register::P2 as u8 => Ok(54823),
            x if x == Register::P3 as u8 => Ok(3024),
            x if x == Register::P4 as u8 => Ok(5831),
            x if x == Register::P5 as u8 => Ok(96),
            x if x == Register::P6 as u8 => Ok(65529),
            x if x == Register::P7 as u8 => Ok(9900),
            x if x == Register::P8 as u8 => Ok(55306),
            x if x == Register::P9 as u8 => Ok(4285),

            x if x == Register::H1 as u8 => Ok(75),
            x if x == Register::H2 as u8 => Ok(355),
            x if x == Register::H3 as u8 => Ok(0),

            x if x == Register::H4 as u8 => Ok(21),
            x if x == Register::H5 as u8 => Ok(0),
            x if x == Register::H6 as u8 => Ok(0),

            x if x == Register::H7 as u8 => Ok(28960),
            _ => Err(LinuxI2CError::Nix(nix::Error::InvalidPath)),
        }
    }

    fn smbus_read_byte_data(&mut self, register: u8) -> Result<u8, Self::Error> {
        match register {
            x if x == Register::TEMP_DATA as u8 => Ok(129),
            x if x == Register::TEMP_DATA_1 as u8 => Ok(142),
            x if x == Register::TEMP_DATA_2 as u8 => Ok(0),
            x if x == Register::PRESSURE_DATA as u8 => Ok(92),
            x if x == Register::PRESSURE_DATA_1 as u8 => Ok(215),
            x if x == Register::PRESSURE_DATA_2 as u8 => Ok(112),
            x if x == Register::HUMIDITY_DAT as u8 => Ok(111),
            x if x == Register::HUMIDITY_DAT_1 as u8 => Ok(159),     
            x if x == Register::H1 as u8 => Ok(75),
            x if x == Register::H2 as u8 => Ok(355),
            x if x == Register::H3 as u8 => Ok(0),

            x if x == Register::H4 as u8 => Ok(21),
            x if x == Register::H5 as u8 => Ok(0),
            x if x == Register::H6 as u8 => Ok(0),

            x if x == Register::H7 as u8 => Ok(28960),        
            _ => Err(LinuxI2CError::Nix(nix::Error::InvalidPath)),
        }
    }
}

#[test]
fn set_of_known_calibration_values_should_yield_known_temperature() {
    let bme = Bme280::new(FakeDevice {}).unwrap();

    let t = bme.read_temperature().unwrap();
    println!("Temperature is: {}.", t);
    assert!((t - 70.44).abs() < 0.01);
}

#[test]
fn set_of_known_calibration_values_should_yield_known_pressure() {
    let bme = Bme280::new(FakeDevice {}).unwrap();

    let p = bme.read_pressure().unwrap();
    println!("Pressure is: {} inhg.", p);
    assert!((p - 30.14).abs() < 0.01);
}

#[test]
fn set_of_known_calibration_values_should_yield_known_humidity() {
    let bme = Bme280::new(FakeDevice {}).unwrap();

    let h = bme.read_humidity().unwrap();
    println!("Humidity is {}%.", h);
    assert!((h - 38.68).abs() < 0.01);
}
