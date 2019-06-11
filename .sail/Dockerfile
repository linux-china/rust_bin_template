FROM codercom/ubuntu-dev-llvm8

RUN sudo apt-get update && sudo apt-get install bash-completion

# complete -W '$(just --summary)' just
RUN curl -LSfs https://japaric.github.io/trust/install.sh | sudo sh -s -- --git casey/just --target x86_64-unknown-linux-musl --to /usr/local/bin
RUN echo "complete -W '$(just --summary)' just" >> ~/.bashrc

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain stable -y

ENV PATH=/home/user/.cargo/bin:$PATH

RUN installext rust-lang.rust
