extern crate i2cdev;
extern crate bme280;

use std::error::Error;
use i2cdev::core::I2CDevice;
use i2cdev::linux::{LinuxI2CDevice, LinuxI2CError};
use bme280::bme280::Bme280;

fn create_device() -> Bme280<DebugDeviceDecorator<LinuxI2CDevice>> {
    let i2c_addr = 0x77;
    let busnum = 2;
    let devname = format!("/dev/i2c-{}", busnum);

    let linuxi2cdevice = LinuxI2CDevice::new(devname, i2c_addr).unwrap();
    let debug_device = DebugDeviceDecorator { device: linuxi2cdevice };
    let result = Bme280::new(debug_device).unwrap();
    result
}

#[test]
#[ignore]
fn it_can_initialize() {
    let i2c_addr = 0x77;
    let busnum = 2;
    let devname = format!("/dev/i2c-{}", busnum);

    let linuxi2cdevice = LinuxI2CDevice::new(devname, i2c_addr).unwrap();
    let debug_device = DebugDeviceDecorator { device: linuxi2cdevice };
    let result = Bme280::new(debug_device);

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
    let bme = create_device();

    let t = bme.read_temperature().unwrap();
    println!("The temperature is: {:.2}", t);
    assert!(t > -50.0); // I'm starting out thinking fahrenheit, but we'll get there.
    assert!(t < 130.0);
}

#[test]
#[ignore]
fn pressure_reading_should_be_reasonable() {
    let bme = create_device();

    let p = bme.read_pressure().unwrap();
    println!("The pressure is: {:.2} in hg.", p);
    assert!(p > 25.0);
    assert!(p < 35.0);
}

#[test]
#[ignore]
fn humidity_reading_should_be_reasonable() {
    let bme = create_device();

    bme.print_calibration();

    let h = bme.read_humidity().unwrap();
    println!("The humidity is: {:.2}%.", h);
    assert!(h > 0.0);
    assert!(h < 100.0);
}

struct DebugDeviceDecorator<T: I2CDevice<Error = LinuxI2CError> + Sized> {
    device: T,
}

impl<T> I2CDevice for DebugDeviceDecorator<T>
    where T: I2CDevice<Error = LinuxI2CError> + Sized
{
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
        println!("smbus_read_i2c_block_data: register: {}, len: {}",
                 register,
                 len);
        self.device.smbus_read_i2c_block_data(register, len)
    }

    fn smbus_write_block_data(&mut self, register: u8, values: &[u8]) -> Result<(), Self::Error> {
        println!("smbus_write_block_data: register: {}, values: {:?}",
                 register,
                 values);
        self.device.smbus_write_block_data(register, values)
    }

    fn smbus_process_block(&mut self, register: u8, values: &[u8]) -> Result<(), Self::Error> {
        println!("smbus_process_block: register: {}, values: {:?}",
                 register,
                 values);
        self.device.smbus_process_block(register, values)
    }

    fn smbus_read_word_data(&mut self, register: u8) -> Result<u16, LinuxI2CError> {
        print!("smbus_read_word_data: register: {}", register);
        let result = try!(self.device.smbus_read_word_data(register));
        println!(" result: {}", result);
        Ok(result)
    }

    fn smbus_read_byte_data(&mut self, register: u8) -> Result<u8, Self::Error> {
        print!("smbus_read_byte_data: register: {}", register);
        let result = try!(self.device.smbus_read_byte_data(register));
        println!(" result: {}", result);
        Ok(result)
    }
}
