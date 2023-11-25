# Ubuntu 22.04をベースにする
FROM ubuntu:22.04

# 必要なパッケージをインストール
RUN apt-get update && apt-get install -y curl git build-essential musl-tools

# Rustをインストール
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Rustのパスを設定
ENV PATH="/root/.cargo/bin:${PATH}"

# rustfmtとcargo-makeをインストール
RUN rustup component add rustfmt
RUN cargo install --force cargo-make
RUN rustup target add x86_64-unknown-linux-musl

# コンテナを起動するときに実行されるコマンド
CMD ["/bin/bash"]
