# SubTrack API 仕様書

## エンドポイント一覧

## 認証 API

### POST /auth/signup

新規ユーザー登録を行います。

#### リクエスト

```json
{
  "email": "string",
  "password": "string",
  "name": "string"
}
```

#### レスポンス

```json
{
  "user_id": "string",
  "token": "string",
  "email": "string",
  "name": "string"
}
```

---

### POST /auth/login

ログイン認証を行います。

#### リクエスト

```json
{
  "email": "string",
  "password": "string"
}
```

#### レスポンス

```json
{
  "token": "string",
  "user": {
    "user_id": "string",
    "email": "string",
    "name": "string"
  }
}
```

---

### POST /auth/logout

ログアウト処理を行います。

#### リクエスト

認証ヘッダーのみ必要

#### レスポンス

```json
{
  "message": "Logged out successfully"
}
```

#### ステータスコード

| コード | 説明           |
| ------ | -------------- |
| 200    | ログアウト成功 |
| 401    | 認証エラー     |

---

### POST /auth/refresh-token

アクセストークンを更新します。

#### リクエスト

```json
{
  "refresh_token": "string"
}
```

#### レスポンス

```json
{
  "access_token": "string",
  "refresh_token": "string"
}
```

## 補足事項

- バリデーションルールの詳細は別途定義
- レートリミットの制限は別途定義
- セキュリティ要件は別途定義
- 認証トークンの有効期限は別途定義
