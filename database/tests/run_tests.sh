#! /bin/bash


docker-compose up -d database mosquitto

sleep 20

echo Start SUT
export RUST_BACKTRACE=1
export RUST_LOG=DEBUG
./../mysql_client/target/debug/mysql_client /app/database/tests &
SUT_PID=$!
echo mysql_client PID: $SUT_PID

python3 tests.py
echo Test result: $?

kill -9 $SUT_PID

docker-compose down

echo End
