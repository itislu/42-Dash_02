FROM golang:1.22.5

# All dependencies required to build the Go project
RUN apt-get update && apt-get install -y curl build-essential sudo

RUN curl https://sh.rustup.rs/ -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
