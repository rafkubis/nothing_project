import os
import time
import threading
import sys
import subprocess
import mysql.connector
import pytest

def test():
    print("Start building")
    a = os.system("cd ../mysql_client/ && cargo build")
    print("Buidl finished with ", a)
    pwd = os.system("pwd")


   # a = os.spawnlp(os.P_NOWAITO, "export RUST_LOG=DEBUG && cd .. && ./app/database/mysql_client/target/debug/mysql_client /app/database/tests", [" "])

    #subprocess.run("export RUST_LOG=DEBUG && ./../mysql_client/target/debug/mysql_client /app/database/tests", shell=True, check=True)
    #subprocess.call("export RUST_LOG=DEBUG && ./../mysql_client/target/debug/mysql_client /app/database/tests")

   # print(a)

    #sut_thread = threading.Thread(target=sut_func)
    #sut_thread.start()

    time.sleep(1)
    message = '{"multiSensor": {"sensors": [{"type": "temperature", "id": 0, "value": 2137, "trend": 2, "state": 2, "elapsedTimeS": -1}]}}'
    mqtt_publish = "mosquitto_pub -t 'test/topic' -m '" + message + "' -h mosquitto"
    os.system(mqtt_publish)
    time.sleep(1)
    db = mysql.connector.connect(host = "database", user= "root", password="strong_password")
    cursor = db.cursor()
    cursor.execute("USE test")
    cursor.execute("SELECT * FROM users")
    res = cursor.fetchall()
    print(res)
    #cursor.execute("DROP TABLE users")

    assert res[0][0] == 21.37

test()

