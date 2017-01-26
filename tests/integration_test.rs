extern crate i2cdev;
extern crate BME280;

use std::error::Error;

#[test]
fn it_can_initialize() {
    let result = BME280::create(0x77, 0);
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