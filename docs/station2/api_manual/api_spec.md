# API 仕様書

## API 一覧

[認証 API](./endpoints/auth.md) <br/>
[サブスク API](./endpoints/subscribe.md) <br/>
[カテゴリ API](./endpoints/category.md) <br/>

## 共通仕様

### リクエストヘッダー

| ヘッダー名    | 必須 | 説明                  |
| ------------- | ---- | --------------------- |
| Authorization | ○    | Bearer + JWT トークン |
| Content-Type  | ○    | application/json      |

### エラーレスポンス

```json
{
  "error": {
    "code": "ERROR_CODE",
    "message": "エラーメッセージ"
  }
}
```

### ステータスコード

| コード | 説明                     |
| ------ | ------------------------ |
| 200    | 成功                     |
| 201    | 作成成功                 |
| 400    | リクエストパラメータ不正 |
| 401    | 認証エラー               |
| 403    | 権限エラー               |
| 404    | リソースなし             |
| 409    | リソース重複             |
| 500    | サーバーエラー           |
