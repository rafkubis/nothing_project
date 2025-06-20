FROM ubuntu


RUN apt-get update
RUN apt-get install -y curl wget
RUN apt-get install -y mosquitto-clients net-tools iputils-ping 
RUN apt-get install -y traceroute python3 python3-pip git 
RUN apt-get install -y iproute2 telnet strace docker.io docker-compose
RUN apt-get install -y mysql-client-core-8.0 vim build-essential pkg-config libssl-dev
RUN apt-get install -y cmake nmap ca-certificates neovim cloc unzip
RUN apt-get install -y protobuf-compiler

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

RUN install -m 0755 -d /etc/apt/keyrings
RUN curl -fsSL https://download.docker.com/linux/ubuntu/gpg -o /etc/apt/keyrings/docker.asc
RUN chmod a+r /etc/apt/keyrings/docker.asc

RUN echo \
  "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.asc] https://download.docker.com/linux/ubuntu \
  $(. /etc/os-release && echo "${UBUNTU_CODENAME:-$VERSION_CODENAME}") stable" | \
  tee /etc/apt/sources.list.d/docker.list > /dev/null

RUN apt-get update
RUN apt-get install -y docker-compose-plugin

RUN mkdir -p ~/.config/pip/
RUN echo "[global]" > ~/.config/pip/pip.conf
RUN echo "break-system-packages = true" >> ~/.config/pip/pip.conf
RUN python3 -m pip install requests paho-mqtt pytest pytest-asyncio mysql-connector-python uniplot testcontainers

RUN wget https://developer.download.nvidia.com/compute/cuda/repos/ubuntu2204/x86_64/cuda-ubuntu2204.pin
RUN mv cuda-ubuntu2204.pin /etc/apt/preferences.d/cuda-repository-pin-600
RUN wget https://developer.download.nvidia.com/compute/cuda/12.8.1/local_installers/cuda-repo-ubuntu2204-12-8-local_12.8.1-570.124.06-1_amd64.deb
RUN dpkg -i cuda-repo-ubuntu2204-12-8-local_12.8.1-570.124.06-1_amd64.deb
RUN cp /var/cuda-repo-ubuntu2204-12-8-local/cuda-*-keyring.gpg /usr/share/keyrings/
RUN apt-get update
RUN apt-get -y install cuda-toolkit-12-8

ENV PATH="/usr/local/cuda/bin:${PATH}"
