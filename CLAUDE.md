# CLAUDE.md

このファイルは、Claude Code (claude.ai/code) がこのリポジトリで作業する際のガイダンスを提供します。

## プロジェクト概要

Fridgers Backendは、スマート冷蔵庫とその内容物を管理するためのRust製Webサービスです。REST APIにはActix-webを使用し、明確なレイヤー分離によるクリーンアーキテクチャの原則に従っています。

## ビルドと開発コマンド

### ローカル開発

```bash
# .envファイルを準備
cp .env.template .env

# PostgreSQLコンテナを起動
make up

# マイグレーション実行
make db-migrate

# ローカルでRustサーバーを起動 (cargo run)
make dev
```

ローカル開発ではPostgreSQLのみDockerで起動し、Rustアプリは`cargo run`で直接実行する構成です。

### ビルド
```bash
# すべてのワークスペースメンバーをビルド
cargo build

# リリースモードでビルド
cargo build --release

# 特定のバイナリをビルド
cargo build --bin rest-server
```

### テスト
```bash
# すべてのテストを実行
cargo test

# インテグレーションテストのみ実行（DB起動が必要）
cargo test --test integration_test -- --test-threads=1

# 特定のパッケージのテストを実行
cargo test -p fridgers-backend-domain
cargo test -p fridgers-backend-use-case
cargo test -p fridgers-backend-config

# 特定のテストを実行
cargo test test_name
```

### コード品質
```bash
# ビルドせずにコードをチェック
cargo check

# clippyリンターを実行
cargo clippy

# コードをフォーマット
cargo fmt
```

### データベース
```bash
# PostgreSQLコンテナを起動
make up

# PostgreSQLコンテナを停止
make down

# マイグレーション実行
make db-migrate

# マイグレーションをロールバック
make db-rollback

# マイグレーションの適用状況を確認
make db-migrate-info

# PostgreSQLに接続
make db-shell

# データベースをリセット（ボリューム削除＋マイグレーション再実行）
make db-reset
```

### APIコンテナ（ビルド・デプロイ用）
```bash
# APIのDockerイメージをビルド
make api-build

# APIコンテナを起動
make api-up

# APIコンテナを停止
make api-down
```

## アーキテクチャ

このコードベースは**クリーンアーキテクチャ**に従い、関心事を明確なレイヤーに分離するワークスペース構造を採用しています：

### レイヤー構造 (依存方向: Apps → Interface → Use-Case → Domain)

1. **Domain Layer** (`src/domain/`)
   - コアビジネスエンティティとドメインロジック
   - 外部依存なし
   - エンティティ: User (UserId, UserName)

2. **Use-Case Layer** (`src/use-case/`)
   - アプリケーションビジネスルールとオーケストレーション
   - HTTPステータスコード (400, 401, 403, 404, 409, 412, 500) にマッピングされる`Error` enumを定義
   - `Repository`トレイト: DB操作の抽象化（ネイティブ async fn in trait）
   - `Interactor<R: Repository>`: ジェネリクスによるDI
   - フレームワークや外部システムから独立

3. **Interface Layer** (`src/interface/`)
   - 外部通信のためのアダプター
   - `rest-controller/`: REST APIコントローラー（ハンドラ、ルーター、スキーマ）
   - `rdb-gateway/`: PostgreSQLリポジトリ実装
     - `dto/`: DBの行をドメインモデルに変換するDTO（TryFromトレイトで変換）
     - `repositories/`: テーブルごとのDB操作メソッド

4. **Infrastructure Layer** (`src/infrastructure/`)
   - `config/`: 環境変数ベースの設定管理
     - .envファイル読み込みに`dotenvy`を使用
     - プレフィックス付き環境変数 (`LOG_`, `SERVER_`) に`envy`を使用
     - `DATABASE_URL`環境変数から直接DB接続URLを取得
     - アプリケーション設定のための`Config::from_env()`を提供
     - 設定内容: LogConfig, ServerConfig, DbConfig

5. **Apps Layer** (`src/apps/`)
   - `rest-server/`: メインアプリケーションバイナリ
     - Actix-web HTTPサーバー
     - ミドルウェア: リクエストトレーシングスパン、アクセスログ
     - DI設定: PgPool → PostgresRepository → Interactor

### Docker構成

- `deployments/db/docker-compose.yml`: PostgreSQLのみ（ローカル開発用）
- `deployments/api/docker-compose.yml`: APIコンテナ（ビルド・デプロイ用）
- `deployments/api/Dockerfile`: マルチステージビルド（Rust 1.93.0）

### データベース

- PostgreSQL 16（Docker Composeで提供）
- sqlx 0.8を使用（非同期、`query_as` + `FromRow`による型安全なマッピング）
- マイグレーションファイル: `deployments/db/migrations/`（リバーシブル形式: .up.sql / .down.sql）
- `query_as!`マクロ（コンパイル時DB接続が必要）は使用せず、`query_as`関数を使用

### リポジトリパターン

- DBは1つなのでリポジトリトレイトも統一（`Repository`トレイト）
- ネイティブ async fn in trait を使用（async-traitクレート不使用）
- `Interactor<R: Repository>` でジェネリクスによるDI
- 標準トレイト（`TryFrom`, `From`等）を積極的に利用し、独自メソッドより優先する

### エラーハンドリングパターン

プロジェクトでは、レイヤー化されたエラーハンドリングアプローチを採用：
- 各レイヤーが独自の`Error` enumと`Result<T>`型エイリアスを定義
- インフラストラクチャエラーは`From`トレイト実装を通じてユースケースエラーに変換
- rdb-gateway内でsqlxエラーをuse-case Errorに変換
- ユースケース`Error`バリアントはHTTPステータスコードにマッピング (`src/use-case/src/error.rs`参照)

### テスト構成

インテグレーションテストは`src/apps/rest-server/tests/`に配置：
```
tests/
├── lib.rs            # エントリポイント（Cargo.tomlの[[test]]で指定）
├── helper/
│   └── mod.rs        # テストユーティリティ（App構築、DBクリーンアップ）
└── user/
    └── mod.rs        # ユーザー関連のテストケース
```

- actix-webのテストユーティリティでインプロセスにAPIを検証
- 実際のPostgreSQLに接続（`make up`でDB起動が必要）
- `--test-threads=1`で直列実行（DB共有のため）

### 設定

設定は環境変数から読み込まれます：
- `LOG_LEVEL`: ログレベル (例: "debug", "info")
- `SERVER_URL`, `SERVER_PORT`: サーバー設定
- `DATABASE_URL`: PostgreSQL接続URL

利用可能なすべての設定オプションは`.env.template`を参照してください。

### APIドキュメント

OpenAPI仕様は`docs/fridgers.openapi.yaml`で管理されており、以下をカバーしています：
- Livenessプローブエンドポイント
- ユーザー操作
- グループ操作
- 冷蔵庫操作
- アイテム操作

## Rustツールチェーン

このプロジェクトは`rust-toolchain.toml`で指定されているRust 1.93.0を使用します。rustupを使用している場合、ツールチェーンは自動的にインストールされます。
