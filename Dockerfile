FROM ubuntu

WORKDIR /usr/src/myapp
COPY . .

RUN apt-get update && \
    apt-get install -y -q curl pkg-config build-essential openssl libssl-dev
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
RUN $HOME/.cargo/bin/rustup default 1.29.0-nightly
RUN $HOME/.cargo/bin/cargo build
CMD ["$HOME/.cargo/bin/cargo", "run"]