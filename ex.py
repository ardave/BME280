from Adafruit_BME280 import *

sensor = BME280(mode=BME280_OSAMPLE_8, busnum=2)

pascals = sensor.read_pressure()
degrees = sensor.read_temperature()
degreesf = degrees * 9 / 5 + 32

hectopascals = pascals / 100
inhg = hectopascals / 33.8638866667
humidity = sensor.read_humidity()

print 'Timestamp = {0:0.3f}'.format(sensor.t_fine)
print 'Temp      = {0:0.3f} deg C'.format(degreesf)
print 'Pressure  = {0:0.2f} inhg'.format(inhg)
print 'Humidity  = {0:0.2f} %'.format(humidity)

pascals = sensor.read_pressure()
hectopascals = pascals / 100
inhg = hectopascals / 33.8638866667
print 'Pressure  = {0:0.2f} inhg'.format(inhg)
