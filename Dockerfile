# FROM ubuntu
FROM buildpack-deps

WORKDIR /usr/src/myapp

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Run cargo build-deps before copying src directory in order to create a cached layer
# see https://github.com/rust-lang/cargo/issues/2644#issuecomment-335258680
RUN $HOME/.cargo/bin/cargo build --release

COPY ./src ./src

RUN $HOME/.cargo/bin/cargo build --release
CMD ["sh", "-c", "${HOME}/.cargo/bin/cargo run"]