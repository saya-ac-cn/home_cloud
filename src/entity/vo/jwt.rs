use crate::error::Error;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation, Algorithm};
use serde::{Deserialize, Serialize};

/// JWT 鉴权 Token结构
#[derive(Debug, Serialize, Deserialize, Clone,Eq, PartialEq)]
pub struct JWTToken {
    //账号
    pub account: String,
    //姓名
    pub name: String,
    //过期时间
    pub exp: usize,
}

impl JWTToken {
    /// create token
    /// secret: your secret string
    pub fn create_token(&self, secret: &str) -> Result<String, Error> {
        return match encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(secret.as_ref()),
        ) {
            Ok(t) => Ok(t),
            Err(_) => Err(Error::from("JWTToken encode fail!")), // in practice you would return the error
        };
    }
    /// verify token invalid
    /// secret: your secret string
    pub fn verify(secret: &str, token: &str) -> Result<JWTToken, Error> {
        let validation = Validation {
            //..Validation::default()
            leeway: 1800,// 过期时间，单位秒
            validate_exp: true,
            validate_nbf: false,
            iss: None,
            sub: None,
            aud: None,
            algorithms: vec![Algorithm::HS256],
        };
        return match decode::<JWTToken>(
            &token,
            &DecodingKey::from_secret(secret.as_ref()),
            &validation,
        ) {
            Ok(c) => Ok(c.claims),
            Err(err) => match *err.kind() {
                ErrorKind::InvalidToken => return Err(Error::from("InvalidToken")), // Example on how to handle a specific error
                ErrorKind::InvalidIssuer => return Err(Error::from("InvalidIssuer")), // Example on how to handle a specific error
                _ => return Err(Error::from("InvalidToken other errors")),
            },
        };
    }
}

#[cfg(test)]
mod test {
    use std::thread::sleep;
    use std::time::Duration;
    use rbatis::DateTimeNative;
    use crate::entity::vo::JWTToken;

    #[test]
    fn test_jwt() {
        let j = JWTToken {
            account: "189".to_string(),
            name: "1".to_string(),
            exp: DateTimeNative::now().timestamp() as usize,
        };
        sleep(Duration::from_secs(5));
        let token = j.create_token("ssss").unwrap();
        assert_eq!(JWTToken::verify("ssss", &token).unwrap(), j);
    }
}
