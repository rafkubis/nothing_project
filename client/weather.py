import requests
import json
import datetime
#from uniplot import plot



weather = requests.get("https://api.openweathermap.org/data/3.0/onecall?lat=51.44&lon=-17.04&appid=393a3378086a16e084baa6e49a3b7527")
#print(weather.content)

decoded = json.loads(weather.content)
#print(decoded["hourly"])

clouds = []
dt = []
cloudsTime = []
timeStr = []

for hour in decoded["hourly"]:
    cloud = hour["clouds"] 
    date = hour["dt"]
    cloudsTime.append((date, cloud))

<<<<<<< HEAD
plot(clouds[0:23], dt[0:23], title="Sine wave")

dtCloudJson = []
for dt, cloud in cloudsTime:
    #print(dt, cloud)
    dtCloudJson.append(json.dumps({"dt": dt, "cloud": cloud}))

#print(dtCloudJson)
=======

dtCloudJson = []
for dt, cloud in cloudsTime:
    dtCloudJson.append(json.dumps({"dt": dt, "cloud": cloud}))

>>>>>>> refs/remotes/origin/master

res = json.dumps({"wheather": dtCloudJson})
print(res)
