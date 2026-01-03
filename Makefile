.PHONY: help up down restart build logs clean dev test check fmt clippy db-shell db-reset

# デフォルトターゲット
.DEFAULT_GOAL := help

# 色付き出力用
BLUE := \033[36m
RESET := \033[0m

help: ## ヘルプを表示
	@echo "$(BLUE)利用可能なコマンド:$(RESET)"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "  $(BLUE)%-15s$(RESET) %s\n", $$1, $$2}'

# ========================================
# Docker Compose コマンド
# ========================================

up: ## Docker Composeでコンテナを起動
	cd deployments/api && docker-compose up -d
	@echo "$(BLUE)コンテナが起動しました$(RESET)"
	@echo "API: http://localhost:8080"
	@echo "PostgreSQL: localhost:5432"

down: ## コンテナを停止
	cd deployments/api && docker-compose down
	@echo "$(BLUE)コンテナを停止しました$(RESET)"

restart: down up ## コンテナを再起動

build: ## Dockerイメージをビルド
	cd deployments/api && docker-compose build
	@echo "$(BLUE)イメージのビルドが完了しました$(RESET)"

logs: ## コンテナのログを表示
	cd deployments/api && docker-compose logs -f

clean: ## コンテナとボリュームを削除
	cd deployments/api && docker-compose down -v
	@echo "$(BLUE)コンテナとボリュームを削除しました$(RESET)"

# ========================================
# データベースコマンド
# ========================================

db-shell: ## PostgreSQLに接続
	docker exec -it fridgers-backend-postgres psql -U fridgers -d fridgers

db-reset: ## データベースをリセット
	cd deployments/api && docker-compose down -v
	cd deployments/api && docker-compose up -d postgres
	@echo "$(BLUE)データベースをリセットしました$(RESET)"

# ========================================
# ローカル開発コマンド
# ========================================

dev: ## ローカルでRustサーバーを起動
	cargo run --bin rest-server

test: ## テストを実行
	cargo test

check: ## コードをチェック（ビルドなし）
	cargo check

fmt: ## コードをフォーマット
	cargo fmt

clippy: ## Clippyリンターを実行
	cargo clippy

# ========================================
# 便利なエイリアス
# ========================================

start: up ## upのエイリアス

stop: down ## downのエイリアス

psql: db-shell ## db-shellのエイリアス
