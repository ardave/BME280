extern crate i2cdev;
extern crate bme280;

use std::error::Error;
use i2cdev::linux::{LinuxI2CDevice};
use bme280::bme280::{Bme280};

#[test]
fn it_can_initialize() {
    let i2c_addr = 0x77;
    let busnum = 2;
    let devname = format!("/dev/i2c-{}", busnum);
    let mut device = LinuxI2CDevice::new(devname, i2c_addr).unwrap();
    let result = Bme280::new(&mut device);
    
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
fn temperature_reading_should_be_reasonable() {
    let i2c_addr = 0x77;
    let busnum = 2;
    let devname = format!("/dev/i2c-{}", busnum);
    let mut device = LinuxI2CDevice::new(devname, i2c_addr).unwrap();
    let mut bme = Bme280::new(&mut device).unwrap();

    let t = bme.read_temperature().unwrap();
    println!("The temperature is: {:.2}", t);
    assert!(t > -50.0); // I'm starting out thinking fahrenheit, but we'll get there.
    assert!(t < 130.0);
}

#[test]
fn pressure_reading_should_be_reasonable() {
    let i2c_addr = 0x77;
    let busnum = 2;
    let devname = format!("/dev/i2c-{}", busnum);
    let mut device = LinuxI2CDevice::new(devname, i2c_addr).unwrap();
    let mut bme = Bme280::new(&mut device).unwrap();

    let p = bme.read_pressure().unwrap();
    println!("The pressure is: {:.2} in hg.", p);
    assert!(p > -25.0); 
    assert!(p < 35.0);
}

// #[test]
// fn print_the_calibration() {
//     let i2c_addr = 0x77;
//     let busnum = 2;
//     let devname = format!("/dev/i2c-{}", busnum);
//     let mut device = LinuxI2CDevice::new(devname, i2c_addr).unwrap();
//     let mut bme = Bme280::new(&mut device).unwrap();

//     println!("The calibration:");
//     bme.print_calibration();
// }