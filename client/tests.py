import socket
import paho.mqtt.client as mqtt
import pytest
import threading
import time

def mock_function(num_msg = 0):
    s = socket.socket()
    
    host = socket.gethostname()
    print(host)
    s.bind((host, 9999))
    print(f"socket binded to {s}")

    s.listen(1)
    news, info = s.accept()
    news.setblocking(True)
    print(f"Got connection from {info}")
    for i in range(num_msg):
        data = news.recv(1024)
        print(f"Received {data}")
  #  print(news)
  #  print(info)
    print("close socket")
    s.close()


def test_foo():
    t = threading.Thread(target=mock_function).start()
    print("thread started")
    time.sleep(1)
    print("clinet")
    client = mqtt.Client(mqtt.CallbackAPIVersion.VERSION2)
    assert client.connect("172.19.0.2", 9999, 60) == mqtt.MQTT_ERR_SUCCESS
    print(client.publish("test/topic", "test message")) 
    #assert client.publish("test/topic", "test message") == mqtt.MQTT_ERR_SUCCESS

   # client.subscribe("test/topic", 0)
""" 
def test_foo1():
    threading.Thread(target=mock_function, args=[10]).start()
    time.sleep(1)
    client = mqtt.Client(mqtt.CallbackAPIVersion.VERSION2)
    assert client.connect("172.19.0.2", 9999, 60) == mqtt.MQTT_ERR_SUCCESS
    a =  client.publish("test/topic", "test message")
    print(a) """
   # client.subscribe("test/topic", 0)




