#[derive(Debug)]
pub struct UserRepositoryImpl {
    client: aws_sdk_dynamodb::Client,
    table_name: String,
}

impl UserRepositoryImpl {
    pub fn new(client: aws_sdk_dynamodb::Client, table_name: String) -> Self {
        UserRepositoryImpl { client, table_name }
    }
}

const USER_ID: &str = "user_id";
const EMAIL: &str = "email";
const NAME: &str = "name";
const USER_TYPE: &str = "user_type";
const PROFILE_ICON_PATH: &str = "profile_icon_path";

const USER_ID_ATTR: &str = "#user_id";
const EMAIL_ATTR: &str = "#email";
const NAME_ATTR: &str = "#name";
const USER_TYPE_ATTR: &str = "#user_type";
const PROFILE_ICON_PATH_ATTR: &str = "#profile_icon_path";

const USER_ID_VALUE: &str = ":user_id";
const EMAIL_VALUE: &str = ":email";
const NAME_VALUE: &str = ":name";
const USER_TYPE_VALUE: &str = ":user_type";
const PROFILE_ICON_PATH_VALUE: &str = ":profile_icon_path";

impl domain::repository::user_repository::UserRepository for UserRepositoryImpl {
    fn find_by_id(
        &self, id: i32,
    ) -> std::pin::Pin<
        Box<
            dyn std::future::Future<Output = Result<domain::user::User, domain::user::user_error::UserError>>
                + Send
                + '_,
        >,
    > {
        use crate::mapper::Mapper;
        use aws_sdk_dynamodb::error::ProvideErrorMetadata;

        let result = Box::pin(async move {
            let result = self
                .client
                .get_item()
                .table_name(&self.table_name)
                .key(USER_ID, aws_sdk_dynamodb::types::AttributeValue::S(id.to_string()))
                .send()
                .await
                .map_err(|e| {
                    let msg = match e.message() {
                        Some(s) => s.to_string(),
                        None => "".to_string(),
                    };
                    domain::user::user_error::UserError::FindByIdError(msg)
                })?;

            match result.item {
                Some(item) => {
                    tracing::info!("{:?}", item);
                    UserRepositoryImpl::map_to_domain_model(item)
                }
                None => {
                    let error = Err(domain::user::user_error::UserError::FindByIdError(id.to_string()));
                    tracing::error!("{:?}", error);
                    error
                }
            }
        });
        result
    }

    fn create(
        &self, user: domain::user::User,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), domain::user::user_error::UserError>> + Send + '_>>
    {
        use aws_sdk_dynamodb::error::ProvideErrorMetadata;
        use aws_sdk_dynamodb::types::AttributeValue;

        let result = Box::pin(async move {
            let result = self
                .client
                .put_item()
                .table_name(&self.table_name)
                .item(USER_ID, AttributeValue::S(user.user_id().to_string()))
                .item(EMAIL, AttributeValue::S(user.email().to_string()))
                .item(NAME, AttributeValue::S(user.name().to_string()))
                .item(USER_TYPE, AttributeValue::S(user.user_type().to_string()))
                .item(
                    PROFILE_ICON_PATH,
                    AttributeValue::S({
                        if let Some(v) = user.profile_icon_path() {
                            v.to_string()
                        } else {
                            "0".to_string()
                        }
                    }),
                );

            match result.send().await {
                Ok(p) => {
                    tracing::info!("{:?}", p);
                    Ok(())
                }
                Err(e) => {
                    let msg = match e.message() {
                        Some(s) => s.to_string(),
                        None => "".to_string(),
                    };
                    let error = Err(domain::user::user_error::UserError::CreateUserError(msg));
                    tracing::error!("{:?}", error);
                    error
                }
            }
        });
        result
    }

    fn update(
        &self, user: domain::user::User,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), domain::user::user_error::UserError>> + Send + '_>>
    {
        use aws_sdk_cognitoidentityprovider::error::ProvideErrorMetadata;
        use aws_sdk_dynamodb::types::AttributeValue;

        let result = Box::pin(async move {
            let result = self.client
                .update_item()
                .table_name(&self.table_name)
                .key(USER_ID, aws_sdk_dynamodb::types::AttributeValue::S(user.user_id().to_string()))
                .update_expression("SET #email = :email, #name = :name, #user_type = :user_type, #profile_icon_path = :profile_icon_path")
                .expression_attribute_names(USER_ID_ATTR, USER_ID)
                .expression_attribute_names(EMAIL_ATTR, EMAIL)
                .expression_attribute_names(NAME_ATTR, NAME)
                .expression_attribute_names(USER_TYPE_ATTR, USER_TYPE)
                .expression_attribute_names(PROFILE_ICON_PATH_ATTR, PROFILE_ICON_PATH)
                .expression_attribute_values(USER_ID_VALUE, AttributeValue::S(user.user_id().to_string()))
                .expression_attribute_values(EMAIL_VALUE, AttributeValue::S(user.email().to_string()))
                .expression_attribute_values(NAME_VALUE, AttributeValue::S(user.name().to_string()))
                .expression_attribute_values(USER_TYPE_VALUE, AttributeValue::S(user.user_type().to_string()))
                .expression_attribute_values(
                    PROFILE_ICON_PATH_VALUE,
                    AttributeValue::S({
                        if let Some(v) = user.profile_icon_path() {
                            v.to_string()
                        } else {
                            "0".to_string()
                        }
                    }),
                )
                .send()
                .await
                .map_err(|e| {
                    let msg = match e.message() {
                        Some(s) => s.to_string(),
                        None => e.to_string()
                        };
                    domain::user::user_error::UserError::UpdateUserError(msg)
                });

            match result {
                Ok(v) => {
                    tracing::info!("{:?}", v);
                    Ok(())
                }
                Err(e) => {
                    tracing::error!("{:?}", e);
                    Err(domain::user::user_error::UserError::UpdateUserError(e.to_string()))
                }
            }
        });
        result
    }
}

impl crate::mapper::Mapper<domain::user::User, domain::user::user_error::UserError> for UserRepositoryImpl {
    fn map_to_domain_model(
        v: std::collections::HashMap<String, aws_sdk_dynamodb::types::AttributeValue>,
    ) -> Result<domain::user::User, domain::user::user_error::UserError> {
        use crate::mapper::as_string;
        use domain::user::user_error::UserError;
        use domain::user::User;

        let user_id = as_string(v.get(USER_ID), "");
        let email = as_string(v.get(EMAIL), "");
        let name = as_string(v.get(NAME), "");
        let user_type = as_string(v.get(USER_TYPE), "");
        let profile_icon_path = Some(as_string(v.get(PROFILE_ICON_PATH), ""));

        let user_type = match user_type.as_str() {
            "1" => 1,
            "2" => 2,
            _ => {
                return Err(UserError::UserTypeError(domain::user::user_type::UserTypeError::InvalidValue(
                    user_type.to_string(),
                )))
            }
        };
        Ok(User::build(&user_id, &email, &name, user_type, profile_icon_path)?)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use aws_sdk_dynamodb::types::AttributeValue;
    use domain::EntityId;

    use crate::mapper::{as_string, Mapper};

    use super::*;

    #[test]
    fn tset_map_to_domain_model() {
        let test_cases = vec![
            HashMap::from([
                (USER_ID.to_string(), AttributeValue::S("usr_550e8400-e29b-41d4-a716-446655440000".to_string())),
                (EMAIL.to_string(), AttributeValue::S("hoge123@email.com".to_string())),
                (NAME.to_string(), AttributeValue::S("hoge".to_string())),
                (USER_TYPE.to_string(), AttributeValue::S("1".to_string())),
                (PROFILE_ICON_PATH.to_string(), AttributeValue::S("../../".to_string())),
            ]),
        ];

        let _test = test_cases
            .into_iter()
            .map(|test| match UserRepositoryImpl::map_to_domain_model(test.clone()) {
                Ok(v) => {
                    assert_eq!(v.user_id().value().to_string(), as_string(test.get(USER_ID), ""));
                    assert_eq!(v.email().to_string(), as_string(test.get(EMAIL), ""));
                    assert_eq!(v.name().to_string(), as_string(test.get(NAME), ""));
                    assert_eq!(v.user_type().to_string(), as_string(test.get(USER_TYPE), ""));

                    let icon_path = v.profile_icon_path().clone().unwrap();
                    assert_eq!(icon_path, as_string(test.get(PROFILE_ICON_PATH), ""))
                }
                Err(e) => {
                    println!("{:?}", e);
                    assert!(false)
                }
            })
            .collect::<()>();
    }
}
