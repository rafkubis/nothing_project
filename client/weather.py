import requests
import json
import datetime
import os



file = os.environ["PROJECT_USER_DATA"] + "/open_wheather/url.txt" 
with open(file, "r") as f:
    url = f.read()
print(url)
weather = requests.get(url)
decoded = json.loads(weather.content)

clouds = []
dt = []
cloudsTime = []
timeStr = []

for hour in decoded["hourly"]:
    cloud = hour["clouds"] 
    date = hour["dt"]
    cloudsTime.append((date, cloud))


dtCloudJson = []
for dt, cloud in cloudsTime:
    dtCloudJson.append(json.dumps({"dt": dt, "cloud": cloud}))


res = json.dumps({"wheather": dtCloudJson})
print(res)
