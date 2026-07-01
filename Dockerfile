# ビルドステージ
FROM rust:1-bookworm AS builder

WORKDIR /app

# sqlxのコンパイル時クエリ検証をオフラインで行う
ENV SQLX_OFFLINE=true

# ソースとビルドに必要なファイルをコピー
COPY Cargo.toml Cargo.lock ./
COPY .sqlx ./.sqlx
COPY migrations ./migrations
COPY src ./src

# リリースビルド
RUN cargo build --release

# 実行ステージ
FROM debian:bookworm-slim

# TLS用のルート証明書
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# ビルド済みバイナリだけを持ち込む
COPY --from=builder /app/target/release/do-not-use-AI-agent /usr/local/bin/server

# コンテナが待ち受けるポート
EXPOSE 3000

CMD ["server"]
