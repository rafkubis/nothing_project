import testcontainers
import pytest
import testcontainers.compose
import testcontainers.core
import testcontainers.core.container
import testcontainers.core.image
import restReadPublisMqtt
import time

def test():
    compsoe = testcontainers.compose.DockerCompose("/app", "compose.yaml")
    compsoe.services = ["mqtt", "json-server"]
    try:
        compsoe.start()
    except:
        print("Exception")
    print("AA")
    print(compsoe)

    logs = compsoe.get_logs("mqtt")
    print(logs)

    logs = compsoe.get_logs("json-server")
    print(logs)
   # time.sleep(30)
    
    compsoe.stop()

    assert(False)