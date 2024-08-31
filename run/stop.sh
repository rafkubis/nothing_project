#! /bin/bash
REST_PROCESSES=$(ps -ax | grep restReadPublisMqtt | grep -v grep) 
MYSQL_CLIENT_PROCESS=$(ps -ax | grep mysql_client | grep -v grep) 

if [ -z "$REST_PROCESSES" ]; then
    echo "restReadPublisMqtt is not running"
else
    echo "restReadPublisMqtt is running, killing process"
    echo "$REST_PROCESSES"
    echo "$REST_PROCESSES" | awk '{print $1}' | xargs kill -9
fi

if [ -z "$MYSQL_CLIENT_PROCESS" ]; then
    echo "mysql_client is not running"
else
    echo "mysql_client is running, killing process"
    echo "$MYSQL_CLIENT_PROCESS"
    echo "$MYSQL_CLIENT_PROCESS" | awk '{print $1}' | xargs kill -9
fi