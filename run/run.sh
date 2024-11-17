#! /bin/bash
echo "Run stop script"
sh /app/run/stop.sh
echo ""

python3 /app/client/restReadPublisMqtt.py log_to_file=True &

log_path=$PWD
echo "log_path: $log_path"
cd /app/database/mysql_client
RUST_LOG=INFO cargo run -- $log_path &

cd ../