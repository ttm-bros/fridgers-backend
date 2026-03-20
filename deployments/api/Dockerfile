# ========================================
# ビルドステージ
# ========================================
FROM rust:1.93.1-bookworm AS builder

WORKDIR /app

# 依存関係のキャッシュを最適化するため、まずCargo.tomlファイルをコピー
COPY Cargo.toml ./
COPY rust-toolchain.toml ./
COPY src ./src

# リリースビルドを実行
RUN cargo build --release --bin rest-server

# ========================================
# ランタイムステージ
# ========================================
FROM debian:bookworm-slim

# 必要なランタイム依存関係をインストール
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# 非rootユーザーを作成
RUN useradd -m -u 1000 appuser

WORKDIR /app

# ビルドステージからバイナリをコピー
COPY --from=builder /app/target/release/rest-server /app/rest-server

# アプリケーションの所有権を変更
RUN chown -R appuser:appuser /app

# 非rootユーザーに切り替え
USER appuser

# ポートを公開
EXPOSE 8080

# ヘルスチェック
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD ["/bin/sh", "-c", "test -f /app/rest-server"]

# アプリケーションを起動
CMD ["/app/rest-server"]
