# テーブル定義書

## user テーブル

| 論理名               | 物理名            | データ型 | PK/SK | GSI | NULL 許可 |
| -------------------- | ----------------- | -------- | ----- | --- | --------- |
| ユーザー ID          | user_id           | string   | PK    | -   | NO        |
| メールアドレス       | email             | string   | -     | -   | NO        |
| プロフィール画像パス | profile_icon_path | string   | -     | -   | YES       |
| 名前                 | name              | string   | -     | -   | NO        |
| ユーザータイプ       | user_type         | string   | -     | -   | NO        |

## payment_method テーブル

| 論理名        | 物理名             | データ型      | PK/SK | GSI | NULL 許可 |
| ------------- | ------------------ | ------------- | ----- | --- | --------- |
| ユーザー ID   | user_id            | string        | PK    | -   | NO        |
| 支払い方法 ID | payment_method_key | string        | SK    | -   | NO        |
| 支払い方法名  | method_name        | string        | -     | -   | NO        |
| 支払い種別    | method_kind_name   | string        | -     | -   | NO        |
| 追加情報      | additional_name    | string        | -     | -   | YES       |
| 作成日時      | created_at         | datetime<utc> | -     | -   | NO        |
| 更新日時      | updated_at         | datetime<utc> | -     | -   | YES       |

## subscribe テーブル

| 論理名         | 物理名             | データ型      | PK/SK | GSI              | NULL 許可 |
| -------------- | ------------------ | ------------- | ----- | ---------------- | --------- |
| ユーザー ID    | user_id            | string        | PK    | GSI-Category(PK) | NO        |
| サブスク ID    | subscribe_id       | string        | SK    | -                | NO        |
| サブスク名     | subscribe_name     | string        | -     | -                | NO        |
| 支払い方法 ID  | payment_method_id  | uuid          | -     | -                | NO        |
| カテゴリー ID  | category_id        | int           | -     | GSI-Category(SK) | NO        |
| 金額           | amount             | int           | -     | -                | NO        |
| 支払いサイクル | payment_cycle      | string        | -     | -                | NO        |
| アイコンパス   | icon_path          | string        | -     | -                | YES       |
| 通知設定       | notification       | bool          | -     | -                | NO        |
| 初回支払日     | first_payment_date | datetime<utc> | -     | -                | NO        |
| 次回支払日     | next_payment_date  | datetime<utc> | -     | -                | NO        |
| 自動更新       | auto_renewal       | bool          | -     | -                | NO        |
| ステータス     | status             | string        | -     | -                | NO        |
| メモ           | memo               | string        | -     | -                | YES       |
| 作成日時       | created_at         | datetime<utc> | -     | -                | NO        |
| 更新日時       | updated_at         | datetime<utc> | -     | -                | YES       |

## category テーブル

| 論理名        | 物理名        | データ型 | PK/SK | GSI | NULL 許可 |
| ------------- | ------------- | -------- | ----- | --- | --------- |
| ユーザー ID   | user_id       | string   | PK    | -   | NO        |
| カテゴリー ID | category_id   | string   | SK    | -   | NO        |
| カテゴリー名  | category_name | string   | -     | -   | NO        |

## user_type テーブル

| 論理名            | 物理名       | データ型 | PK/SK | GSI | NULL 許可 |
| ----------------- | ------------ | -------- | ----- | --- | --------- |
| ユーザータイプ ID | user_type_id | string   | PK    | -   | NO        |
| タイプ名          | type_name    | string   | -     | -   | NO        |
