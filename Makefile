.PHONY: help up down restart logs clean dev test check fmt clippy db-shell db-reset db-migrate db-rollback db-migrate-info api-build api-up api-down

# デフォルトターゲット
.DEFAULT_GOAL := help

# 色付き出力用
BLUE := \033[36m
RESET := \033[0m

help: ## ヘルプを表示
	@echo "$(BLUE)利用可能なコマンド:$(RESET)"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "  $(BLUE)%-20s$(RESET) %s\n", $$1, $$2}'

# ========================================
# ローカル開発（DB + cargo run）
# ========================================

up: ## PostgreSQLコンテナを起動
	cd deployments/db && docker compose up -d
	@echo "$(BLUE)PostgreSQLが起動しました$(RESET)"
	@echo "PostgreSQL: localhost:5432"

down: ## PostgreSQLコンテナを停止
	cd deployments/db && docker compose down
	@echo "$(BLUE)PostgreSQLを停止しました$(RESET)"

restart: down up ## PostgreSQLコンテナを再起動

logs: ## PostgreSQLのログを表示
	cd deployments/db && docker compose logs -f

clean: ## PostgreSQLコンテナとボリュームを削除
	cd deployments/db && docker compose down -v
	@echo "$(BLUE)コンテナとボリュームを削除しました$(RESET)"

dev: ## ローカルでRustサーバーを起動
	cargo run --bin rest-server

# ========================================
# APIコンテナ（ビルド・デプロイ用）
# ========================================

api-build: ## APIのDockerイメージをビルド
	cd deployments/api && docker compose build
	@echo "$(BLUE)APIイメージのビルドが完了しました$(RESET)"

api-up: ## APIコンテナを起動
	cd deployments/api && docker compose up -d
	@echo "$(BLUE)APIコンテナが起動しました$(RESET)"

api-down: ## APIコンテナを停止
	cd deployments/api && docker compose down
	@echo "$(BLUE)APIコンテナを停止しました$(RESET)"

# ========================================
# データベースコマンド
# ========================================

db-shell: ## PostgreSQLに接続
	docker exec -it fridgers-backend-postgres psql -U fridgers -d fridgers

db-reset: clean up db-migrate ## データベースをリセットしマイグレーション実行

db-migrate: ## マイグレーションを実行
	sqlx migrate run --source deployments/db/migrations
	@echo "$(BLUE)マイグレーションを実行しました$(RESET)"

db-rollback: ## マイグレーションをロールバック
	sqlx migrate revert --source deployments/db/migrations
	@echo "$(BLUE)マイグレーションをロールバックしました$(RESET)"

db-migrate-info: ## マイグレーションの適用状況を確認
	sqlx migrate info --source deployments/db/migrations

# ========================================
# コード品質
# ========================================

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
