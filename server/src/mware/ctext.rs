// use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::db::database::Session;

pub const AUTH_TOKEN: &str = "x-auth-token";
// struct Keys {
//     encoding: EncodingKey,
//     decoding: DecodingKey,
// }

// impl Keys {
//     fn new(secret: &[u8]) -> Self {
//         Self {
//             encoding: EncodingKey::from_secret(secret),
//             decoding: DecodingKey::from_secret(secret),
//         }
//     }
// }

// static KEYS: Lazy<Keys> = Lazy::new(|| {
//     let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
//     Keys::new(secret.as_bytes())
// });

// #[async_trait]
// impl<S: Send + Sync> FromRequestParts<S> for Ctext {
//     type Rejection = ServerError;

//     // #[tracing::instrument]
//     async fn from_request_parts(
//         parts: &mut axum::http::request::Parts,
//         _state: &S,
//     ) -> Result<Self, ServerError> {
//         info!("->> from_request_parts");

//         let headers = parts
//             .extract::<HeaderMap>()
//             .await
//             .expect("Could not extract headers");

//         let auth_token = match headers.get("x-auth-token") {
//             Some(x) => {
//                 println!("Parsing header auth token: {:?}", &x);
//                 x.to_str().expect("Auth token was not a string")
//             }
//             None => "",
//         };
//         println!("Parsed auth_token: {:?}", &auth_token);

//         let session_id = match headers.get("session_id") {
//             Some(x) => {
//                 println!("Parsing header auth token: {:?}", &x);
//                 x.to_str().expect("Auth token was not a string")
//             }
//             None => "",
//         };
//         info!("Session token: ${session_id}");

//         if auth_token.is_empty() {
//             info!(" ->> Auth header was not present");
//             return Err(ServerError::AuthFailNoTokenCookie);
//         }

//         let jwt_claim = validate_jwt_claim(auth_token)?;

//         Ok(Ctext::new(jwt_claim.uid, ))
//     }
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String, // subject
    exp: usize,  // expire
    iat: usize,  // issued at
    // iss: String, // issuer
    // aud: String, // audience
    uid: String,  // customer - user_id
    role: String, // custom - role of the user
    nbf: usize,   // not before
    tenant: String,
}

#[derive(Serialize)]
pub struct JwtResult {
    pub token: String,
    pub expires: usize,
}
// #[derive(Debug, Serialize, Deserialize)]
// struct Claims {
//     sub: String, // subject
//     exp: usize,  // expire
//     iat: usize,  // issued at
//     // iss: String, // issuer
//     // aud: String, // audience
//     uid: String,  // customer - user_id
//     role: String, // custom - role of the user
//     nbf: usize,   // not before
//     tenant: String,
// }

// #[tracing::instrument]
// fn validate_jwt_claim(
//     // payload: &Json<LoginPayload>,
//     // key: &[u8; 10],
//     jwt_token: &str,
// ) -> anyhow::Result<Claims, ServerError> {
//     info!("->> validate_jwt_claim");

//     // let key = b"privatekey";
//     let decode_key = &KEYS.decoding;
//     let decode_result =
//         match decode::<Claims>(jwt_token, decode_key, &Validation::new(Algorithm::HS256)) {
//             Ok(x) => {
//                 info!("jwt was decoded properly");
//                 x.claims
//             }
//             Err(err) => {
//                 info!("Invalid token: {}", err.to_string());
//                 match err.into_kind() {
//                     jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
//                         info!("Invalid token - Expired");
//                         return Err(ServerError::AuthFailTokenExpired);
//                     }
//                     _ => {
//                         return Err(ServerError::AuthFailTokenDecodeIssue);
//                     }
//                 }
//             }
//         };

//     return Ok(decode_result);
// }

// #[tracing::instrument]
// pub fn create_jwt_claim(
//     // payload: &Json<LoginPayload>,
//     // key: &[u8; 10],
//     user_id: String,
//     role: &str, // jwt_token: String,
// ) -> anyhow::Result<JwtResult, ServerError> {
//     info!("->> create_jwt_claim");

//     let nowutc = chrono::Utc::now();
//     let now: usize = match nowutc
//         .timestamp()
//         .try_into()
//         .with_context(|| "Could not turn time into timestamp")
//     {
//         Ok(x) => x,
//         Err(_e) => return Err(ServerError::WrongCredentials),
//     };
//     let expire: usize = match (nowutc + chrono::Duration::minutes(5))
//         .timestamp()
//         .try_into()
//     {
//         Ok(x) => x,
//         Err(_e) => return Err(ServerError::WrongCredentials),
//     };

//     let claim = Claims {
//         sub: "myemailsub@email.com".to_string(),
//         exp: expire,
//         iat: now,
//         uid: user_id.to_string(),
//         nbf: now,
//         tenant: "tenant".to_string(),
//         role: role.to_string(),
//     };

//     // return Ok(claim);
//     let jwt = match encode(&Header::default(), &claim, &KEYS.encoding) {
//         Ok(t) => t,
//         Err(_) => {
//             return Err(ServerError::BadRequest(
//                 "yo this request is messed".to_string(),
//             ))
//         }
//     };

//     return Ok(JwtResult {
//         token: jwt,
//         expires: expire,
//     });
// }

#[derive(Clone, Debug)]
pub struct Ctext {
    pub user_id: String,
    pub session: Session,
    // parse cookies in here?
}

impl Ctext {
    // pub fn user_id(&self) -> &String {
    //     &self.user_id
    // }

    pub fn new(user_id: String, session: Session) -> Self {
        Ctext { user_id, session }
    }
}

// pub fn create_jwt_token(user: &Ctext) -> Result<String, ServerError> {
//     let nowutc = chrono::Utc::now();
//     let now: usize = match nowutc
//         .timestamp()
//         .try_into()
//         .with_context(|| "Could not turn time into timestamp")
//     {
//         Ok(x) => x,
//         Err(_e) => return Err(ServerError::WrongCredentials),
//     };
//     let expire: usize = match (nowutc + chrono::Duration::minutes(5))
//         .timestamp()
//         .try_into()
//     {
//         Ok(x) => x,
//         Err(_e) => return Err(ServerError::WrongCredentials),
//     };
//     let claims = Claims {
//         sub: "myemailsub@email.com".to_string(),
//         exp: expire,
//         iat: now,
//         uid: user.user_id.to_string(),
//         nbf: now,
//         tenant: "".to_string(),
//         role: "".to_owned(),
//     };
//     let jwt = encode(&Header::default(), &claims, &KEYS.encoding)
//         .map_err(|_| ServerError::AuthTokenCreationFail)?;
//     Ok(jwt)
// }

// #[derive(FromRow)]
// pub struct UserModel {
//     id: i32,
// }

#[derive(FromRow, Debug)]
pub struct Person {
    pub id: i32,
    pub name: String,
    pub age: i32,
}
