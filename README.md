# ec-extension

各ECプラットフォームとAPI連携することで、データ操作や機能拡張を実現する

## 使用技術一覧

<p style="display: inline">
 <img src="https://img.shields.io/badge/-Rust-000000.svg?logo=rust&style=for-the-badge">
  <img src="https://img.shields.io/badge/-Actix Web-000000.svg?logo=actix&style=for-the-badge">
 <img src="https://img.shields.io/badge/-Docker-1488C6.svg?logo=docker&style=for-the-badge">
 <img src="https://img.shields.io/badge/-Shopify-7AB55C.svg?logo=shopify&style=for-the-badge">
</p>

## アーキテクチャ

クリーンアーキテクチャを採用したプロジェクト構成とする

## フレームワーク/ライブラリ

[Cargo.toml](backend/Cargo.toml)を参考にする

## 実行

本アプリケーションの実行方法を記載する

### 手順

* `backend`配下に`.env`を作成する、[環境変数一覧](#環境変数一覧)を参考にすること
* プロジェクトルートで`docker compose up`

### 環境変数一覧

| 変数名 | 説明 | デフォルト値 |
| - | - | - |
| STORE_URL | ECプラットフォームのAPIエンドポイント | |
| ACCESS_TOKEN | ECプラットフォームのAPIアクセストークン | |
| LOG_LEVEL | アプリケーションのログレベル(error, warn, info, debug, trace, offから設定) | debug |
| APP_ADDRESS | アプリケーションのアドレス | 0.0.0.0 |
| APP_PORT | アプリケーションの使用ポート | 8011 |
