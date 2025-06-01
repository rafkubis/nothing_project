import testcontainers
import testcontainers.compose
import subprocess
import os
import time
import paho.mqtt.client as mqtt1
import mysql.connector
import docker
import requests
import json


def save_logs(logs, service_name, pwd):
    (out, err) = logs
    with open(f"{pwd}{service_name}_out.log", "w") as file:
        file.write(out)
    with open(f"{pwd}{service_name}_err.log", "w") as file:
        file.write(err)


class TestCase:
    def setup_method(self, method):
        self.test_name = method.__name__
        print(f"Start {self.test_name}")
        self.dir_path = (
            f"{os.environ["CARGO_TARGET_DIR"]}/integration_test_py/{self.test_name}/"
        )
        os.makedirs(self.dir_path, exist_ok=True)
        self.mqtt = "mqtt"
        self.database = "database"
        self.compose = testcontainers.compose.DockerCompose(
            os.environ["WORKDIR"], "compose.yaml"
        )
        self.compose.services = [self.mqtt, self.database]
        self.compose.wait = True
        try:
            self.compose.start()

        except Exception as e:
            print("Error starting compose: ", e)
            assert False, "Error starting compose"

        self.process = subprocess.Popen(
            [f"{os.environ["CARGO_TARGET_DIR"]}/debug/mysql_client", self.dir_path]
        )
        time.sleep(2)

    def save_container_logs(self):
        mqtt_logs = self.compose.get_logs(self.mqtt)
        save_logs(mqtt_logs, self.mqtt, self.dir_path)
        database_logs = self.compose.get_logs(self.database)
        save_logs(database_logs, self.database, self.dir_path)

    def cleanup(self):
        self.process.terminate()
        self.process.wait()
        self.compose.stop()

    def teardown_method(self):
        try:
            self.save_container_logs()
        except Exception as e:
            self.cleanup()
            assert False, "Error getting logs"

        self.cleanup()
        assert True, "Test passed"

    def create_mqtt_client(self):
        client = mqtt1.Client(mqtt1.CallbackAPIVersion.VERSION2)
        client.username = "app"
        client.password = "test_app"
        client.connect(self.mqtt, 1883, 60)
        client.loop_start()
        return client

    def parse_wheather(self, weather):
        decoded = json.loads(weather.content)
        cloudsTime = []

        for hour in decoded["hourly"]:
            cloud = hour["clouds"]
            date = hour["dt"]
            cloudsTime.append((date, cloud))

        dtCloudJson = []
        for dt, cloud in cloudsTime:
            dtCloudJson.append({"dt": dt, "cloud": cloud})

        res = json.dumps({"wheather": dtCloudJson})
        return res

    def get_forecast(self):
        file = os.environ["PROJECT_USER_DATA"] + "/open_wheather/url.txt"
        with open(file, "r") as f:
            url = f.read()
        print(url)

        weather = requests.get(url)
        return self.parse_wheather(weather)

    def recive_mqtt_message(self, client, userdata, message):
        print(f"Received message: {message.topic} {message.payload.decode()}")
        self.recived_massages.append(message)

    def test_mqtt_and_mysql(self):
        payload = '{"multiSensor": {"sensors": [{"type": "temperature", "id": 0, "value": 2137, "trend": 2, "state": 2, "elapsedTimeS": -1}]}}'
        self.recived_massages = []
        client = self.create_mqtt_client()
        client.on_message = lambda client, userdata, message: self.recive_mqtt_message(
            client, userdata, message
        )
        client.subscribe("driver", qos=2)
        client.publish("temperature", payload, qos=0)
        client.publish("wheather", self.get_forecast(), qos=0)

        time.sleep(2)
        db = mysql.connector.connect(
            host="database",
            user="root",
            password="strong_password",
            database="test",
            port=3306,
        )
        cursor = db.cursor()
        cursor.execute("SELECT * FROM users")
        result = cursor.fetchall()
        print(result)
        assert result[0][0] == 21.37
        time.sleep(5)
        assert self.recived_massages[0].topic == "driver"

    def test_driver(self):
        docker_client = docker.from_env()
        container = docker_client.containers.get("workspace-mqtt-1")
        container.restart()
        time.sleep(5)
