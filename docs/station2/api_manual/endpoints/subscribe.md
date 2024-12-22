# SubTrack API 仕様書

## エンドポイント一覧

## Subscribe API

### GET /subscribes

ユーザーのサブスクリプション一覧取得

#### リクエストパラメータ

| パラメータ名 | 必須 | 型     | 説明        |
| ------------ | ---- | ------ | ----------- |
| user_id      | ○    | string | ユーザー ID |

#### レスポンス

```json
{
  "subscribes": [
    {
      "subscribe_id": "string",
      "subscribe_name": "string",
      "payment_method_id": "string",
      "category_id": "string",
      "amount": 0,
      "payment_cycle": "string",
      "icon_path": "string",
      "notification": true,
      "first_payment_date": "datetime",
      "next_payment_date": "datetime",
      "auto_renewal": true,
      "status": "string",
      "memo": "string"
    }
  ]
}
```

### GET /subscribes/category

カテゴリー別サブスクリプション一覧取得

#### リクエストパラメータ

| パラメータ名 | 必須 | 型     | 説明          |
| ------------ | ---- | ------ | ------------- |
| user_id      | ○    | string | ユーザー ID   |
| category_id  | ○    | string | カテゴリー ID |

#### レスポンス

```json
{
  "subscribes": [
    {
      "subscribe_id": "string",
      "subscribe_name": "string",
      "payment_method_id": "string",
      "category_id": "string",
      "amount": 0,
      "payment_cycle": "string",
      "icon_path": "string",
      "notification": true,
      "first_payment_date": "datetime",
      "next_payment_date": "datetime",
      "auto_renewal": true,
      "status": "string",
      "memo": "string"
    }
  ]
}
```

### GET /subscribes/{subscribe_id}

サブスクリプション詳細取得

#### リクエストパラメータ

| パラメータ名 | 必須 | 型     | 説明                  |
| ------------ | ---- | ------ | --------------------- |
| user_id      | ○    | string | ユーザー ID           |
| subscribe_id | ○    | string | サブスクリプション ID |

#### レスポンス

```json
{
  "subscribe_id": "string",
  "subscribe_name": "string",
  "payment_method_id": "string",
  "category_id": "string",
  "amount": 0,
  "payment_cycle": "string",
  "icon_path": "string",
  "notification": true,
  "first_payment_date": "datetime",
  "next_payment_date": "datetime",
  "auto_renewal": true,
  "status": "string",
  "memo": "string"
}
```

### POST /subscribes

サブスクリプション新規登録

#### リクエスト

```json
{
  "user_id": "string",
  "subscribe_name": "string",
  "payment_method_id": "string",
  "category_id": "string",
  "amount": 0,
  "payment_cycle": "string",
  "icon_path": "string",
  "notification": true,
  "first_payment_date": "datetime",
  "next_payment_date": "datetime",
  "auto_renewal": true,
  "status": "string",
  "memo": "string"
}
```

#### レスポンス

```json
{
  "subscribe_id": "string",
  "message": "Created successfully"
}
```

### PUT /subscribes/{subscribe_id}

サブスクリプション更新

#### リクエスト

```json
{
  "user_id": "string",
  "subscribe_name": "string",
  "payment_method_id": "string",
  "category_id": "string",
  "amount": 0,
  "payment_cycle": "string",
  "icon_path": "string",
  "notification": true,
  "first_payment_date": "datetime",
  "next_payment_date": "datetime",
  "auto_renewal": true,
  "status": "string",
  "memo": "string"
}
```

#### レスポンス

```json
{
  "message": "Updated successfully"
}
```

### DELETE /subscribes/{subscribe_id}

サブスクリプション削除

#### リクエストパラメータ

| パラメータ名 | 必須 | 型     | 説明                  |
| ------------ | ---- | ------ | --------------------- |
| user_id      | ○    | string | ユーザー ID           |
| subscribe_id | ○    | string | サブスクリプション ID |

#### レスポンス

```json
{
  "message": "Deleted successfully"
}
```
