# fridgers-backend

スマート冷蔵庫とその内容物を管理するためのRust製Webサービスです。

## 技術スタック

- **言語**: Rust 1.93.0
- **Webフレームワーク**: Actix-web 4
- **データベース**: PostgreSQL 16 + sqlx 0.8
- **アーキテクチャ**: クリーンアーキテクチャ

## セットアップ

```bash
# .envファイルを準備
cp .env.template .env

# PostgreSQLを起動
make up

# マイグレーション実行
make db-migrate

# サーバーを起動
make dev
```

## 主なコマンド

| コマンド | 説明 |
|---------|------|
| `make up` | PostgreSQLコンテナを起動 |
| `make down` | PostgreSQLコンテナを停止 |
| `make dev` | ローカルでサーバーを起動 |
| `make db-migrate` | マイグレーション実行 |
| `make db-rollback` | マイグレーションをロールバック |
| `make db-reset` | データベースをリセット |
| `cargo test` | テストを実行 |
| `cargo clippy` | リンターを実行 |

## プロジェクト構造

```
src/
├── apps/rest-server/       # アプリケーションバイナリ
├── domain/                 # ドメインエンティティ
├── use-case/               # ユースケース・ビジネスロジック
├── interface/
│   ├── rest-controller/    # REST APIハンドラ
│   └── rdb-gateway/        # PostgreSQLリポジトリ実装
└── infrastructure/config/  # 環境変数ベースの設定

deployments/
├── api/                    # APIコンテナ（Dockerfile）
└── db/                     # PostgreSQL + マイグレーション
```

## APIドキュメント

OpenAPI仕様: `docs/fridgers.openapi.yaml`
