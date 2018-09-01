FROM ubuntu

WORKDIR /usr/src/myapp
COPY . .

RUN apt-get update && \
    apt-get install -y -q curl pkg-config build-essential openssl libssl-dev cron
RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
RUN $HOME/.cargo/bin/rustup default nightly 
# TODO 1.29.0-nightly

# Run cron job to update medium posts every hour
RUN echo "0 * * * * root curl -sSfq -X POST http://localhost:8000/mediumPosts" \
    >/etc/cron.d/updatedMediumPosts

RUN $HOME/.cargo/bin/cargo build
CMD ["sh", "-c", "${HOME}/.cargo/bin/cargo run & cron -f"]