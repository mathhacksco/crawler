FROM ubuntu

WORKDIR /usr/src/myapp
COPY . .

RUN apt-get update && \
    apt-get install -y -q curl pkg-config build-essential openssl libssl-dev
# TODO 1.29.0-nightly
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly

RUN $HOME/.cargo/bin/cargo build
CMD ["sh", "-c", "${HOME}/.cargo/bin/cargo run"]