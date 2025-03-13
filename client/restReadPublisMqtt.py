import requests
import time
import json
import paho.mqtt.client as mqtt
import logging
import threading
from shared_urls import SharedUrls
import sys

logger = logging.getLogger(__name__)
rest_urls = SharedUrls()

def on_connect(client, userdata, flags, reason_code, properties):
    logger.info(f"Connected with result code {reason_code}")
   
def on_message(client, userdata, msg):
    logger.info(msg.topic+" "+str(msg.payload))

def on_disconnect(client, userdata, rc, arg, arg2):
    logger.info(f"Disconnected with result code {rc}")
    while True:
        time.sleep(1)
        try:
            client.reconnect()
            break
        except Exception as e:
            logger.error(f"An exception occurred: {e}")

def findTempSensorIp(maxIp=255):
    result = []
    for i in range(maxIp):
        try:
            rest_url = "http://192.168.5." + str(i) + "/state"
            logger.debug(rest_url)
            resp = requests.get(rest_url, timeout=0.3)
            if resp.reason == "OK":
                logger.debug(f"Sensor IP found: {rest_url}")
                result.append(rest_url)
            else:
                logger.debug(f"{rest_url} Error: {resp.reason}")
        except Exception as e:
            logger.debug((f"An exception occurred: {e}"))
    return result

def rest():
    while True:
        logger.info("Rest thread")
        url = findTempSensorIp()
        rest_urls.set_urls(url)
        logger.info(f"URL {url}")
        time.sleep(30)

def parse_args(*args):
    parsed = dict()
    for arg in args[0]:
        key = arg.split("=")[0]
        value = arg.split("=")[1]
        parsed[key] = value

    return parsed

def set_logger(args):
    format = "%(asctime)s %(levelname)s %(thread)d %(threadName)s %(filename)s %(funcName)s %(message)s"
    log_level = "INFO"
    if("log_level" in args.keys()):
        log_level = args["log_level"]

    if "log_to_file" in args.keys() and args["log_to_file"] == "True": 
        logging.basicConfig(filemode="w", filename="restReadPublishMqtt.log", level=log_level, datefmt="%Y-%m-%d %H:%M:%S", format=format)
    else:
        logging.basicConfig(level=log_level, datefmt="%Y-%m-%d %H:%M:%S", format=format)

def main(*args):
    app_config = parse_args(args)
    set_logger(app_config)
    
    rest_thread = threading.Thread(target=rest)
    rest_thread.start()

    broker_url = "mqtt"

    client = mqtt.Client(mqtt.CallbackAPIVersion.VERSION2)
    client.on_connect = on_connect
    client.on_message = on_message
    client.on_disconnect = on_disconnect
    client.on_socket_close = lambda client, userdata, rc: logger.info(f"Socket closed with result code {rc}")
    client.connect(broker_url, 1883, 60)
    client.loop_start()

    while 1:
        urls = rest_urls.get_urls()
        for url in urls:
            logger.info(f"accessing: {url}")
            resp = requests.get(url)
            if resp.reason == "OK":
                value = json.dumps(resp.json())
                logger.info(value)
                client.publish("test/topic", value)
            else:
                logger.error(f"Error: {resp.reason}")
        time.sleep(10)

if __name__ == "__main__":
    main(*sys.argv[1:])