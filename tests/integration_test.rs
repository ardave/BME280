extern crate i2cdev;
extern crate bme280;

use std::error::Error;
use i2cdev::core::I2CDevice;
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};
use bme280::bme280::{Bme280};

#[test]
#[ignore]
fn it_can_initialize() {
    let i2c_addr = 0x77;
    let busnum = 2;
    let devname = format!("/dev/i2c-{}", busnum);

    let mut linuxi2cdevice = LinuxI2CDevice::new(devname, i2c_addr).unwrap();
    let mut debugDevice = DebugDeviceDecorator {device: &mut linuxi2cdevice};
    let result = Bme280::new(&mut debugDevice);
    
    match result {
        Ok(_device) => assert!(true),
        Err(err) => {
            println!("Cause");
            println!("{}", err.cause().unwrap());
            println!("Description");
            println!("{}", err.description());    
            assert!(false);        
        }
    }
}

#[test]
#[ignore]
fn temperature_reading_should_be_reasonable() {
    let i2c_addr = 0x77;
    let busnum = 2;
    let devname = format!("/dev/i2c-{}", busnum);

    let mut linuxi2cdevice = LinuxI2CDevice::new(devname, i2c_addr).unwrap();
    let mut debugDevice = DebugDeviceDecorator {device: &mut linuxi2cdevice};
    let mut bme = Bme280::new(&mut debugDevice).unwrap();

    let t = bme.read_temperature().unwrap();
    println!("The temperature is: {:.2}", t);
    assert!(t > -50.0); // I'm starting out thinking fahrenheit, but we'll get there.
    assert!(t < 130.0);
}

#[test]
#[ignore]
fn pressure_reading_should_be_reasonable() {
    let i2c_addr = 0x77;
    let busnum = 2;
    let devname = format!("/dev/i2c-{}", busnum);

    let mut linuxi2cdevice = LinuxI2CDevice::new(devname, i2c_addr).unwrap();
    let mut debugDevice = DebugDeviceDecorator {device: &mut linuxi2cdevice};
    let mut bme = Bme280::new(&mut debugDevice).unwrap();

    let p = bme.read_pressure().unwrap();
    println!("The pressure is: {:.2} in hg.", p);
    assert!(p > 25.0); 
    assert!(p < 35.0);
}

struct DebugDeviceDecorator<'a, T: I2CDevice<Error=LinuxI2CError> + Sized + 'a> {
    device: &'a mut T
}

impl<'a, T> I2CDevice for DebugDeviceDecorator<'a, T>
    where T: I2CDevice<Error = LinuxI2CError> + Sized + 'a {
    type Error = LinuxI2CError;

    fn read(&mut self, data: &mut [u8]) -> Result<(), Self::Error> {
        println!("read: data: {:?}", data);
        self.device.read(data)
    }

    fn write(&mut self, data: &[u8]) -> Result<(), Self::Error> {
        println!("write: data: {:?}", data);
        self.device.write(data)
    }

    fn smbus_write_quick(&mut self, bit: bool) -> Result<(), Self::Error> {
        println!("smbus_write_quick: bit: {}", bit);
        self.device.smbus_write_quick(bit)
    }

    fn smbus_read_block_data(&mut self, register: u8) -> Result<Vec<u8>, Self::Error> {
        println!("smbus_read_block_data: register: {}", register);
        self.device.smbus_read_block_data(register)
    }

    fn smbus_read_i2c_block_data(&mut self, register: u8, len: u8) -> Result<Vec<u8>, Self::Error> {
        println!("smbus_read_i2c_block_data: register: {}, len: {}", register, len);
        self.device.smbus_read_i2c_block_data(register, len)
    }

    fn smbus_write_block_data(&mut self, register: u8, values: &[u8]) -> Result<(), Self::Error> {
        println!("smbus_write_block_data: register: {}, values: {:?}", register, values);
        self.device.smbus_write_block_data(register, values)
    }

    fn smbus_process_block(&mut self, register: u8, values: &[u8]) -> Result<(), Self::Error> {
        println!("smbus_process_block: register: {}, values: {:?}", register, values);
        self.device.smbus_process_block(register, values)
    }

    fn smbus_read_word_data(&mut self, register: u8) -> Result<u16, LinuxI2CError> {
        println!("smbus_read_word_data: register: {}", register);
        let result = try!(self.device.smbus_read_word_data(register));
        println!("result: {}", result);
        Ok(result)
    }

    fn smbus_read_byte_data(&mut self, register: u8) -> Result<u8, Self::Error> {
            println!("smbus_read_byte_data: register: {}", register);
            let result = try!(self.device.smbus_read_byte_data(register));
            println!("result: {}", result);
            Ok(result)
    }
}

