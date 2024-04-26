import requests
import time
import json
import paho.mqtt.client as mqtt

def on_connect(client, userdata, flags, reason_code, properties):
    print(f"Connected with result code {reason_code}")

   
def on_message(client, userdata, msg):
    print(msg.topic+" "+str(msg.payload))

rest_url = "http://192.168.5.14/state"
broker_url = "172.17.0.2"

client = mqtt.Client(mqtt.CallbackAPIVersion.VERSION2)
client.on_connect = on_connect
client.on_message = on_message
client.connect(broker_url, 1883, 60)

while 1:
    resp = requests.get(rest_url)
    value = json.dumps(resp.json())
    print(value)
    client.publish("test/topic", value)
    time.sleep(10)