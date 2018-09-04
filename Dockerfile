FROM buildpack-deps

WORKDIR /usr/src/myapp
COPY . .

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly

RUN USER=root $HOME/.cargo/bin/cargo new --bin crawler
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# Run cargo build before copying src directory in order to create a cached layer
# see https://github.com/rust-lang/cargo/issues/2644#issuecomment-335258680
# RUN $HOME/.cargo/bin/cargo build --release
RUN rm src/*.rs
COPY ./src ./src
COPY .env.production .env
# RUN $HOME/.cargo/bin/cargo build --release
CMD ["sh", "-c", "${HOME}/.cargo/bin/cargo run --release"]