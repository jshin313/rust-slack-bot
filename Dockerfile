FROM ubuntu:18.04

RUN apt-get update && \
    apt-get install -y redis-server && \
    apt-get clean

RUN curl https://sh.rustup.rs -sSf | sh

RUN git clone https://github.com/jshin313/rust-slack-bot && \
    cd rust-slack-bot

CMD cargo run <api-key>
