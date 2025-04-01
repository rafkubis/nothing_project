import requests
import json
import datetime
from uniplot import plot



weather = requests.get("https://api.openweathermap.org/data/3.0/onecall?lat=51.44&lon=-17.04&appid=393a3378086a16e084baa6e49a3b7527")


decoded = json.loads(weather.content)

#print(decoded["hourly"])

clouds = []
dt = []
cloudsTime = []
timeStr = []

for hour in decoded["hourly"]:
    cloud = hour["clouds"] 
    clouds.append(hour["clouds"])
    timeStr.append((datetime.datetime.fromtimestamp(hour["dt"]).strftime('%Y-%m-%d %H:%M:%S'), cloud))
    date = hour["dt"]
    a = datetime.datetime.fromtimestamp(date).hour
    dt.append(a)
    cloudsTime.append((date, cloud))

print(timeStr)

plot(clouds[0:23], dt[0:23], title="Sine wave")