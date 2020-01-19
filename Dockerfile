FROM ubuntu:18.04

RUN apt-get update && \
    apt-get install -y git && \
    apt-get clean

RUN curl https://sh.rustup.rs -sSf | sh

RUN git clone https://github.com/jshin313/rust-slack-bot && \
    cd rust-slack-bot

ARG api_key
ENV SLACK_API_TOKEN=$api_key

RUN cargo run
