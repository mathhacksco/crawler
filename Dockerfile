FROM ubuntu
# FROM alpine:3.7

WORKDIR /usr/src/myapp
COPY . .

# RUN apk update && \
#     apk upgrade && \
#     apk add --no-chache sudo && \
#     apk add --no-chache file && \
#     apk add --no-cache curl
# RUN curl -s https://static.rust-lang.org/rustup.sh | sh -s -- --channel=nightly

RUN curl https://sh.rustup.rs -sSf | sh -s -- --channel=nightly -y
RUN rustup default nightly

RUN $HOME/.cargo/bin/cargo build
CMD ["$HOME/.cargo/bin/cargo", "run"]