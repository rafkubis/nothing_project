FROM ubuntu

RUN apt-get update
RUN apt-get install -y mosquitto-clients net-tools iputils-ping 
RUN apt-get install -y traceroute python3 python3-pip curl git 
RUN apt-get install -y iproute2 telnet strace docker.io docker-compose

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

RUN apt-get install -y mysql-client-core-8.0 vim build-essential pkg-config libssl-dev
RUN apt-get install -y cmake nmap ca-certificates

RUN python3 -m pip install requests paho-mqtt pytest pytest-asyncio mysql-connector-python uniplot testcontainers --break-system-packages
ENV PATH="/root/.cargo/bin:${PATH}"

#Add docker gpg key
RUN install -m 0755 -d /etc/apt/keyrings
RUN curl -fsSL https://download.docker.com/linux/ubuntu/gpg -o /etc/apt/keyrings/docker.asc
RUN chmod a+r /etc/apt/keyrings/docker.asc

RUN echo \
  "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.asc] https://download.docker.com/linux/ubuntu \
  $(. /etc/os-release && echo "${UBUNTU_CODENAME:-$VERSION_CODENAME}") stable" | \
  tee /etc/apt/sources.list.d/docker.list > /dev/null

RUN apt-get update
RUN apt-get install -y docker-compose-plugin





#RUN cargo add paho-mqtt

ENTRYPOINT ["/bin/bash"]
