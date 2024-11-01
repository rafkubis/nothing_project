FROM ubuntu

RUN apt-get update
RUN apt-get install -y mosquitto-clients net-tools iputils-ping 
RUN apt-get install -y traceroute python3 python3-pip curl git 
RUN apt-get install -y iproute2 telnet strace

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

RUN apt-get install -y mysql-client-core-8.0 vim build-essential pkg-config libssl-dev
RUN apt-get install -y cmake
COPY .bashrc /root/.bashrc

RUN python3 -m pip install requests paho-mqtt pytest pytest-asyncio --break-system-packages

ENV PATH="/root/.cargo/bin:${PATH}"



#RUN cargo add paho-mqtt

ENTRYPOINT ["/bin/bash"]
