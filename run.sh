#docker run -itd --rm --net mqtt_net --ip 172.18.10.0 -p 1883:1883 -p 9001:9001 -v $(pwd)/broker/mosquitto.conf:/mosquitto/config/mosquitto.conf --name mqtt_broker eclipse-mosquitto

#docker run -itd --rm -p 1883:1883 -p 9001:9001 -v $(pwd)/broker/mosquitto.conf:/mosquitto/config/mosquitto.conf --name mqtt_broker eclipse-mosquitto
#docker run -itd --rm --name mysql_database -e MYSQL_ROOT_PASSWORD=strong_password mysql
#docker run -itd --rm --mount src="$(pwd)",target=/home/project,type=bind --name mqtt_dev_run mqtt_dev


#docker ps | tail -n +2 | awk '{print $1}' | xargs docker inspect --format '{{ .NetworkSettings.IPAddress }}' 
#| 
#    printf "MQTT Broker: %s\nMQTT Dev: %s\nMySQL: %s\n" $(sed 's/ /:/g' | tr '\n' ' ')
docker ps | tail -n +2 | awk '{print $1}' | xargs docker inspect | grep IP
