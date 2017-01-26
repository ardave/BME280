extern crate i2cdev;
extern crate BME280;

use std::error::Error;

#[test]
fn it_can_initialize() {
    let result = BME280::create(0x77, 2);
    match result {
        Ok(device) => assert!(true),
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
    let mut device = BME280::create(0x77, 2).unwrap();
    let t = device.read_temperature().unwrap();
    println!("The temperature is: {}", t);
    assert!(t > -50.0); // I'm starting out thinking fahrenheit, but we'll get there.
    assert!(t < 130.0);
}