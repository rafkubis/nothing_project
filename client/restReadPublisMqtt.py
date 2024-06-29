import requests
import time
import json
import paho.mqtt.client as mqtt

def on_connect(client, userdata, flags, reason_code, properties):
    print(f"Connected with result code {reason_code}")

   
def on_message(client, userdata, msg):
    print(msg.topic+" "+str(msg.payload))

rest_url = ""
for i in range(255):
    try:
        rest_url = "http://192.168.5." + str(i) + "/state"
        print(rest_url)
        resp = requests.get(rest_url, timeout=1)
        value = json.dumps(resp.json())
        print(value)
        break
    except:
        print("An exception occurred")

state_rest_url = rest_url
broker_url = "mosquitto"

client = mqtt.Client(mqtt.CallbackAPIVersion.VERSION2)
client.on_connect = on_connect
client.on_message = on_message
client.connect(broker_url, 1883, 60)

while 1:
    resp = requests.get(state_rest_url)
    value = json.dumps(resp.json())
    print(value)
    client.publish("test/topic", value)
    time.sleep(10)