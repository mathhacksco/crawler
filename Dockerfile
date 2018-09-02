# FROM ubuntu
FROM buildpack-deps

WORKDIR /usr/src/myapp
COPY . .

# RUN apt-get update && \
#     apt-get install -y --no-install-recommends -q \
    # curl pkg-config build-essential openssl libssl-dev
# TODO 1.29.0-nightly
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly

RUN $HOME/.cargo/bin/cargo build --release
CMD ["sh", "-c", "${HOME}/.cargo/bin/cargo run"]