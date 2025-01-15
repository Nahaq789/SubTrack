/// DynamoDBのデータをドメインモデルに変換するためのトレイトです
///
/// Tはドメインモデルの型、Eはエラー型を表します
///
/// # Example
/// ```
/// impl Mapper<PaymentMethod, PaymentError> for PaymentRepositoryImpl {
///     fn to_domain_model(v: HashMap<String, AttributeValue>) -> Result<PaymentMethod, PaymentError> {
///         // 実装
///     }
/// }
/// ```
pub trait Mapper<T, E> {
    fn map_to_domain_model(
        v: std::collections::HashMap<String, aws_sdk_dynamodb::types::AttributeValue>,
    ) -> Result<T, E>;
}

/// DynamoDBのAttributeValueから文字列を取得します
///
/// # Arguments
/// * `val` - DynamoDBのAttributeValue
/// * `default` - 値が取得できない場合のデフォルト値
///
/// # Returns
/// AttributeValueから取得した文字列、または指定されたデフォルト値
pub fn as_string(val: Option<&aws_sdk_dynamodb::types::AttributeValue>, default: &str) -> String {
    val.and_then(|v| v.as_s().ok()).map(ToString::to_string).unwrap_or_else(|| default.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_as_string_success() {
        let binding1 = &aws_sdk_dynamodb::types::AttributeValue::S("test".to_string());
        let binding2 = &aws_sdk_dynamodb::types::AttributeValue::S("test2".to_string());
        let test_cases = vec![
            (Some(binding1), "test".to_string()),
            (Some(binding2), "test2".to_string()),
        ];
        for (found, expected) in test_cases {
            let result = as_string(found, "");
            assert_eq!(result, expected);
        }
    }
}
