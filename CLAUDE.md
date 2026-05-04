# CLAUDE.md

このファイルは、Claude Code (claude.ai/code) がこのリポジトリで作業する際のガイダンスを提供します。

## プロジェクト概要

Fridgers Backendは、スマート冷蔵庫とその内容物を管理するためのRust製Webサービスです。REST APIにはActix-webを使用し、明確なレイヤー分離によるクリーンアーキテクチャの原則に従っています。メール+パスワード認証とJWTによるトークン認証を実装済みです。

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
   - エンティティ:
     - User (UserId, UserName, Email, PasswordHash)
     - Fridge (FridgeId, FridgeName, owner_user_id)
     - Compartment (CompartmentId, CompartmentName, fridge_id)
     - Item (ItemId, ItemName, quantity, unit, expires_at, timestamps)
   - バリデーション: `define_string!`マクロで文字数制限・文字種チェック
   - RawPassword: 生パスワード（10-30文字）

2. **Use-Case Layer** (`src/use-case/`)
   - アプリケーションビジネスルールとオーケストレーション
   - HTTPステータスコード (400, 401, 403, 404, 409, 412, 500) にマッピングされる`Error` enumを定義
   - `Repository`トレイト: DB操作の抽象化（ネイティブ async fn in trait）
   - `Interactor<R: Repository>`: ジェネリクスによるDI、`JwtConfig`を保持
   - `auth`モジュール: JWT生成・検証（`encode_token`, `decode_token`）
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
     - プレフィックス付き環境変数 (`LOG_`, `SERVER_`, `AUTH_`) に`envy`を使用
     - `DATABASE_URL`環境変数から直接DB接続URLを取得
     - アプリケーション設定のための`Config::from_env()`を提供
     - 設定内容: LogConfig, ServerConfig, DbConfig, AuthConfig

5. **Apps Layer** (`src/apps/`)
   - `rest-server/`: メインアプリケーションバイナリ
     - Actix-web HTTPサーバー
     - ミドルウェア: リクエストトレーシングスパン、アクセスログ
     - DI設定: PgPool → PostgresRepository → Interactor (+ JwtConfig)

### APIエンドポイント

| メソッド | パス | 説明 |
|---------|------|------|
| GET | `/liveness` | ヘルスチェック |
| POST | `/v1/users` | ユーザー登録（email, password, name） |
| POST | `/v1/auth/login` | ログイン（email, password → JWT） |
| GET | `/v1/fridges` | 冷蔵庫一覧取得（ログイン中ユーザーが所有するもの。Bearer 必須） |
| POST | `/v1/fridges` | 冷蔵庫作成 |
| GET | `/v1/fridges/{fridge_id}` | 冷蔵庫取得（コンパートメント・アイテム含む） |
| DELETE | `/v1/fridges/{fridge_id}` | 冷蔵庫削除 |
| POST | `/v1/fridges/{fridge_id}/compartments` | コンパートメント作成 |
| PUT | `/v1/fridges/{fridge_id}/compartments/{compartment_id}` | コンパートメント更新 |
| DELETE | `/v1/fridges/{fridge_id}/compartments/{compartment_id}` | コンパートメント削除 |
| POST | `/v1/fridges/{fridge_id}/compartments/{compartment_id}/items` | アイテム作成 |
| PUT | `/v1/fridges/{fridge_id}/compartments/{compartment_id}/items/{item_id}` | アイテム更新 |
| DELETE | `/v1/fridges/{fridge_id}/compartments/{compartment_id}/items/{item_id}` | アイテム削除 |

### 認証・認可

- **パスワード**: Argon2でハッシュ化（`argon2`クレート）、ランダムソルト生成
- **JWT**: `jsonwebtoken`クレートでBearerトークンを発行
  - Claims: `sub`（UserID）、`exp`（有効期限）、`iat`（発行日時）
  - 設定: `JwtConfig { secret, expiry_hours }`（環境変数`AUTH_*`から読み込み）
- **フロー**: ユーザー登録（POST /v1/users） → ログイン（POST /v1/auth/login） → JWTトークン取得
- **認可**: `POST /v1/users` と `POST /v1/auth/login` 以外のすべてのエンドポイントで Bearer 必須
  - Actix の `FromRequest` を実装した `AuthenticatedUser` 抽出器（`src/interface/rest-controller/src/extractor/authenticated_user.rs`）でハンドラの引数として透過的に取得
  - 各 Interactor メソッドは `requesting_user_id: &str` を受け取り、`verify_fridge_ownership` / `verify_compartment_ownership` でリソースのオーナーが一致することを確認
  - 他人のリソースへのアクセスはリソース存在の漏洩を防ぐため `404 Not Found` を返す（`403 Forbidden` ではなく）

### Docker構成

- `deployments/db/docker-compose.yml`: PostgreSQLのみ（ローカル開発用）
- `deployments/api/docker-compose.yml`: APIコンテナ（ビルド・デプロイ用）
- `deployments/api/Dockerfile`: マルチステージビルド（Rust 1.93.0）

### データベース

- PostgreSQL 16（Docker Composeで提供）
- sqlx 0.8を使用（非同期、`query_as` + `FromRow`による型安全なマッピング）
- マイグレーションファイル: `deployments/db/migrations/`（リバーシブル形式: .up.sql / .down.sql）
- `query_as!`マクロ（コンパイル時DB接続が必要）は使用せず、`query_as`関数を使用

テーブル構成:
- `users`: id(UUID PK), name, email(UNIQUE), password_hash, created_at, updated_at
- `fridges`: id(UUID PK), name, owner_user_id(FK→users), created_at, updated_at
- `compartments`: id(UUID PK), fridge_id(FK→fridges ON DELETE CASCADE), name, created_at, updated_at
- `items`: id(UUID PK), compartment_id(FK→compartments ON DELETE CASCADE), name, quantity, unit, expires_at, created_at, updated_at

### リポジトリパターン

- DBは1つなのでリポジトリトレイトも統一（`Repository`トレイト）
- ネイティブ async fn in trait を使用（async-traitクレート不使用）
- `Interactor<R: Repository>` でジェネリクスによるDI
- 標準トレイト（`TryFrom`, `From`等）を積極的に利用し、独自メソッドより優先する

Repositoryトレイトのメソッド:
- User: `save_user`, `find_user_by_id`, `find_user_by_email`, `delete_user`
- Fridge: `save_fridge`, `find_fridge_by_id`, `delete_fridge`
- Compartment: `save_compartment`, `find_compartment_by_id`, `find_compartments_by_fridge_id`, `update_compartment`, `delete_compartment`
- Item: `save_item`, `find_item_by_id`, `find_items_by_compartment_id`, `update_item`, `delete_item`

### エラーハンドリングパターン

プロジェクトでは、レイヤー化されたエラーハンドリングアプローチを採用：
- 各レイヤーが独自の`Error` enumと`Result<T>`型エイリアスを定義
- インフラストラクチャエラーは`From`トレイト実装を通じてユースケースエラーに変換
- rdb-gateway内でsqlxエラーをuse-case Errorに変換
- ユースケース`Error`バリアントはHTTPステータスコードにマッピング (`src/use-case/src/error.rs`参照)

```
InvalidArgument → 400 | Unauthorized → 401 | Forbidden → 403
NotFound → 404 | AlreadyExist → 409 | PreconditionFailed → 412
ExternalServer → 500
```

### テスト構成

インテグレーションテストは`src/apps/rest-server/tests/`に配置：
```
tests/
├── lib.rs            # エントリポイント（Cargo.tomlの[[test]]で指定）
├── helper/
│   └── mod.rs        # テストユーティリティ（App構築、DBクリーンアップ、テスト用JwtConfig）
├── auth/
│   └── mod.rs        # 認証関連のテストケース（ログイン成功/失敗）
└── user/
    └── mod.rs        # ユーザー関連のテストケース（登録、バリデーション、重複メール）
```

- actix-webのテストユーティリティでインプロセスにAPIを検証
- 実際のPostgreSQLに接続（`make up`でDB起動が必要）
- `--test-threads=1`で直列実行（DB共有のため）

### 設定

設定は環境変数から読み込まれます：
- `LOG_LEVEL`: ログレベル (例: "debug", "info")
- `SERVER_URL`, `SERVER_PORT`: サーバー設定
- `DATABASE_URL`: PostgreSQL接続URL
- `AUTH_JWT_SECRET`: JWTトークン署名用シークレットキー
- `AUTH_JWT_EXPIRY_HOURS`: JWTトークンの有効期限（時間）

利用可能なすべての設定オプションは`.env.template`を参照してください。

### APIドキュメント

OpenAPI仕様は`docs/fridgers.openapi.yaml`で管理されています。
※ 現状はユーザー登録エンドポイントのみ記載。冷蔵庫・コンパートメント・アイテム・認証のエンドポイントは未反映。

## 新機能追加時のファイル作成パターン

新しいリソースやユースケースを追加する場合、以下のレイヤーにファイルを作成する：

1. **Domain**: `src/domain/src/{entity}/` - エンティティ、値オブジェクト
2. **Use-Case DTO**: `src/use-case/src/dto/{entity}/{action}/` - Request/Response DTO
3. **Use-Case Interactor**: `src/use-case/src/interactor/{entity}/` - ビジネスロジック
4. **Repository**: `src/use-case/src/repository.rs` にトレイトメソッド追加
5. **RDB Gateway**: `src/interface/rdb-gateway/src/repositories/{entity}.rs` - SQL実装
6. **RDB DTO**: `src/interface/rdb-gateway/src/dto/{entity}.rs` - DBの行マッピング
7. **REST Schema**: `src/interface/rest-controller/src/schema/{entity}/{action}/` - APIスキーマ
8. **REST Handler**: `src/interface/rest-controller/src/handler/{entity}.rs` - ハンドラ
9. **REST Router**: `src/interface/rest-controller/src/router/{entity}.rs` - ルーティング
10. **Migration**: `deployments/db/migrations/` - DBスキーマ変更

各レイヤーの`mod.rs`と`lib.rs`にモジュール宣言を追加することを忘れないこと。

## Rustツールチェーン

このプロジェクトは`rust-toolchain.toml`で指定されているRust 1.93.0を使用します。rustupを使用している場合、ツールチェーンは自動的にインストールされます。
