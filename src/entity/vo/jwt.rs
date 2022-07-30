use actix_http::header::HeaderValue;
use actix_web::HttpRequest;
use crate::error::Error;
use jsonwebtoken::errors::ErrorKind;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use crate::service::CONTEXT;

/// JWT 鉴权 Token结构
#[derive(Debug, Serialize, Deserialize, Clone,Eq, PartialEq)]
pub struct JWTToken {
    /// 账号
    pub account: String,
    /// 姓名
    pub name: String,
    /// 组织
    pub organize: u64,
    /// 登录ip
    pub ip: String,
    /// 登录城市
    pub city: String,
    /// 过期时间
    pub exp: usize,
}

impl JWTToken {
    /// extract token detail
    /// secret: your secret string
    pub fn extract_token(token:&String) -> Result<JWTToken, Error> {
        let token = JWTToken::verify(&CONTEXT.config.jwt_secret, token);
        if token.is_err() {
            return Err(Error::from(format!("access_token is invalid!")));
        }
        let user_data = token.unwrap();
        return Ok(user_data);
    }

    /// extract token detail
    /// secret: your secret string
    pub fn extract_token_by_header(token:Option<&HeaderValue>) -> Result<JWTToken, Error> {
        return match token {
            Some(token) => {
                let token = token.to_str().unwrap_or("");
                JWTToken::extract_token(&token.to_string())
            }
            _ => {
                Err(Error::from(format!("access_token is empty!")))
            }
        }
    }

    /// extract token detail
    /// secret: your secret string
    pub fn extract_user_by_header(token:Option<&HeaderValue>) -> Option<JWTToken>{
        let extract_result = &JWTToken::extract_token_by_header(token);
        if extract_result.is_err() {
            log::error!("在获取用户信息时，发生异常:{}",extract_result.clone().unwrap_err().to_string());
            return None;
        }
        let user_session = extract_result.clone().unwrap();
        return Some(user_session);
    }

    /// extract token detail
    /// secret: your secret string
    pub fn extract_user_by_request(req: &HttpRequest) -> Option<JWTToken>{
        let token = req.headers().get("access_token");
        JWTToken::extract_user_by_header(token)
    }


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
            leeway: 1800,// 过期时间(30分钟)，单位秒
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
    use crate::entity::vo::jwt::JWTToken;
    use crate::entity::vo::JWTToken;

    #[test]
    fn test_jwt() {
        let j = JWTToken {
            account: "189".to_string(),
            name: "1".to_string(),
            organize: 1,
            ip:String::from("127.0.0.1"),
            city:String::from("局域网"),
            exp: DateTimeNative::now().timestamp() as usize,
        };
        sleep(Duration::from_secs(5));
        let token = j.create_token("ssss").unwrap();
        assert_eq!(JWTToken::verify("ssss", &token).unwrap(), j);
    }
}
