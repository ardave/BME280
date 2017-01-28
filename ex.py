from Adafruit_BME280 import *

sensor = BME280(mode=BME280_OSAMPLE_8, busnum=2)

pascals = sensor.read_pressure()
degrees = sensor.read_temperature()

hectopascals = pascals / 100
humidity = sensor.read_humidity()

print 'Timestamp = {0:0.3f}'.format(sensor.t_fine)
print 'Temp      = {0:0.3f} deg C'.format(degrees)
print 'Pressure  = {0:0.2f} hPa'.format(hectopascals)
print 'Humidity  = {0:0.2f} %'.format(humidity)
