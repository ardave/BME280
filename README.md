# BME280
A Linux User-Mode facade for interacting with the Bosch BME280 sensor using I2C.

Provides temperature, pressure, and humidity readings from an attached sensor.

Has been tested on Beaglebone Black Rev C.

Usage example:
```
fn read_some_sensor_values() {
    let i2c_addr = 0x77;
    let bus_num = 2;
    let bme = Bme280::<LinuxI2CDevice>::new(i2c_addr, bus_num).unwrap();

    println!("Temperature is {} degrees Fahrenheit.", bme.read_temperature().unwrap());
    println!("Barometric pressure is {} inhg.", bme.read_pressure().unwrap());
    println!("Relative Humidity is {}%.", bme.read_humidity().unwrap());
}
```

