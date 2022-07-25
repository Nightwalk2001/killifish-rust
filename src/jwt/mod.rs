use std::ops::Add;

use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, Validation};
use jsonwebtoken::errors::Error;
use mongodb::options::ReadConcernLevel::Local;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::errors::AppError;
use crate::mongo::Person;

struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = "secret";
    Keys::new(secret.as_bytes())
});

#[derive(Serialize, Deserialize)]
pub struct Claim {
    pub sub: String,
    #[serde(rename = "isManager")]
    pub is_manager: bool,
    pub iat: DateTime<Utc>,
    pub exp: DateTime<Utc>,
}

pub fn issue(person: &Person) -> String {
    let iat = Utc::now();
    let exp = iat + Duration::days(15);

    let claim = Claim {
        sub: person.name.to_string(),
        is_manager: person.is_manager,
        iat,
        exp,
    };
    let token = encode::<Claim>(
        &Header::default(),
        &claim,
        &KEYS.encoding,
    ).unwrap();

    token
}

fn verify(token: String) -> bool {
    let c = decode::<Claim>(
        &token,
        &KEYS.decoding,
        &Validation::default(),
    ).unwrap().claims;

    let now = Utc::now();

    c.exp < now && c.is_manager
}
