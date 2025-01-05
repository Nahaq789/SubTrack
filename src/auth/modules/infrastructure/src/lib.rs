pub mod mapper;
pub mod repository_impl;

use base64::Engine;
use hmac::Mac;

pub fn client_secret_hash(email: &domain::user::email::Email, client_id: &str, client_secret: &str) -> String {
    let mut mac =
        hmac::Hmac::<sha2::Sha256>::new_from_slice(client_secret.as_bytes()).expect("HMAC can take key of any size");

    mac.update(email.value().as_bytes());
    mac.update(client_id.as_bytes());

    let result = mac.finalize();

    base64::engine::general_purpose::STANDARD.encode(result.into_bytes())
}
