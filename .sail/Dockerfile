FROM codercom/ubuntu-dev

RUN sudo apt-get update && sudo apt-get install bash-completion

# complete -W '$(just --summary)' just
RUN curl -LSfs https://japaric.github.io/trust/install.sh | sudo sh -s -- --git casey/just --target x86_64-unknown-linux-musl --to /usr/local/bin

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

RUN installext rust-lang.rust

