# SubTrack API 仕様書

## エンドポイント一覧

## カテゴリ API

### GET /categories

ユーザーのカテゴリー一覧取得

#### リクエストパラメータ

| パラメータ名 | 必須 | 型     | 説明        |
| ------------ | ---- | ------ | ----------- |
| user_id      | ○    | string | ユーザー ID |

#### レスポンス

```json
{
  "categories": [
    {
      "category_id": "string",
      "category_name": "string"
    }
  ]
}
```

### GET /categories/{category_id}

カテゴリー詳細取得

#### リクエストパラメータ

| パラメータ名 | 必須 | 型     | 説明          |
| ------------ | ---- | ------ | ------------- |
| user_id      | ○    | string | ユーザー ID   |
| category_id  | ○    | string | カテゴリー ID |

#### レスポンス

```json
{
  "category_id": "string",
  "category_name": "string"
}
```

### POST /categories

カテゴリー新規登録

#### リクエスト

```json
{
  "user_id": "string",
  "category_name": "string"
}
```

#### レスポンス

```json
{
  "category_id": "string",
  "message": "Created successfully"
}
```

### PUT /categories/{category_id}

カテゴリー更新

#### リクエスト

```json
{
  "user_id": "string",
  "category_name": "string"
}
```

#### レスポンス

```json
{
  "message": "Updated successfully"
}
```

### DELETE /categories/{category_id}

カテゴリー削除

#### リクエストパラメータ

| パラメータ名 | 必須 | 型     | 説明          |
| ------------ | ---- | ------ | ------------- |
| user_id      | ○    | string | ユーザー ID   |
| category_id  | ○    | string | カテゴリー ID |

#### レスポンス

```json
{
  "message": "Deleted successfully"
}
```
