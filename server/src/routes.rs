pub mod routes {
    use std::collections::HashMap;

    use anyhow::Context;
    use argon2::{
        password_hash::{rand_core::OsRng, SaltString},
        Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    };
    use chrono::{DateTime, Utc};
    use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

    use axum::{
        body::Body,
        extract::{DefaultBodyLimit, Multipart, Path, Query},
        http::{self, HeaderMap, HeaderName, HeaderValue, Method, Request, Uri},
        middleware::{self, Next},
        response::IntoResponse,
        response::Response,
        routing::{get, get_service, post, MethodRouter},
        Extension, Form, Router,
    };

    use db::database::CreateUserRequest;
    use lettre::message::header::Date;
    use markdownparser::{nanoid_gen, parse_markdown_v3};
    // use oauth2::basic::BasicClient;

    // use ormlite::Model;
    // use ormlite::model::Model;
    // use reqwest::header;
    use serde::{Deserialize, Serialize};
    use serde_json::{json, Value};
    // use tower_cookies::{Cookie, Cookies};
    use tracing::{debug, log::info};
    // use uuid::Uuid;
    // use sqlx::{Sqlite, SqlitePool};
    // use tower_http::http::cors::CorsLayer;
    use axum::{
        extract::{self, State},
        http::StatusCode,
        Json,
    };
    use tower_http::{
        auth::require_authorization::Bearer,
        cors::{Any, CorsLayer},
        limit::RequestBodyLimitLayer,
        services::ServeDir,
        trace::TraceLayer,
    };
    use uuid::Uuid;

    // use markdownparser::{nanoid_gen, parse_markdown_v3, MetadataBuilder, SurveyBuilder};

    use crate::{
        auth::{validate_credentials, AuthError},
        db::{self, database::CreateAnswersModel},
        log::log_request,
        mware::{
            self,
            ctext::{
                create_jwt_claim,
                create_jwt_token,
                // mw_ctx_resolver,
                Claims,
                Ctext,
            },
        },
        server::{CreateSurveyResponse, SurveyModel},
        ServerError, ServerState,
    };

    pub fn get_routes(state: ServerState) -> anyhow::Result<Router> {
        let routes = Router::new()
            .route(&format!("/surveys"), post(create_survey).get(list_survey))
            .route("/surveys/:id", get(get_survey).post(submit_survey))
            // .route(&format!("/surveys/test"), post(test_survey))
            .route(&format!("/auth/login"), post(authorize))
            .route("/auth/signup", post(signup))
            .layer(middleware::map_response(main_response_mapper))
            // .layer(middleware::from_fn(propagate_header))
            .with_state(state);

        return Ok(routes);
    }

    async fn propagate_header<B>(req: Request<B>, next: Next<B>) -> Response {
        let mut res = next.run(req).await;
        res
    }

    async fn main_response_mapper(
        ctx: Option<Ctext>,
        uri: Uri,
        req_method: Method,
        res: Response,
    ) -> Response {
        println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
        let uuid = Uuid::new_v4();

        // -- Get the eventual response error.
        let service_error = res.extensions().get::<ServerError>();
        let client_status_error = service_error.map(|se| se.client_status_and_error());

        // -- If client error, build the new reponse.
        let error_response =
            client_status_error
                .as_ref()
                .map(|(status_code, client_error, message)| {
                    let client_error_body = json!({
                        "error": {
                            "type": client_error.as_ref(),
                            "req_uuid": uuid.to_string(),
                            "message": message,
                        }
                    });

                    info!("    ->> client_error_body: {client_error_body}");

                    // Build the new response from the client_error_body
                    (*status_code, Json(client_error_body)).into_response()
                });

        // Build and log the server log line.
        // let client_error = client_status_error.unzip().1;
        let client_error = match client_status_error {
            Some(x) => Some(x.1),
            None => None,
        };
        log_request(uuid, req_method, uri, ctx, service_error, client_error)
            .await
            .expect("Did not log request properly");

        info!("Mapped response, returning...");
        error_response.unwrap_or(res)
    }

    // async fn log_request(
    //     uuid: Uuid,
    //     req_method: Method,
    //     uri: Uri,
    //     ctx: Option<Ctext>,
    //     service_error: Option<&ServerError>,
    //     client_error: Option<crate::error::ClientError>,
    // ) {
    //     println!("logging request...");
    // }

    // #[tracing::instrument]
    #[axum::debug_handler]
    pub async fn create_survey(
        headers: HeaderMap,
        State(state): State<ServerState>,
        ctx: Option<Ctext>,
        extract::Json(payload): extract::Json<CreateSurveyRequest>,
    ) -> anyhow::Result<Json<Value>, ServerError> {
        info!("->> create_survey");

        let user_id = match &ctx {
            Some(x) => x.user_id(),
            None => return Err(ServerError::AuthFailNoTokenCookie),
        };
        info!("Creating new survey for user={user_id:?}");
        // Check that the survey is Ok
        let parsed_survey =
            parse_markdown_v3(payload.plaintext).expect("Could not parse the survey");

        let metadata = Metadata::new();

        // let survey_model = SurveyModelBuilder::default()
        //     .plaintext(parsed_survey.plaintext.clone())
        //     .parse_version(parsed_survey.parse_version.clone())
        //     .user_id(user_id.to_string())
        //     .created_at(metadata.created_at.clone())
        //     .modified_at(metadata.modified_at.clone())
        //     .version(metadata.version.clone())
        //     .id(0)
        //     .build()
        //     .expect("Could not create survey model");

        let survey = SurveyModel {
            id: 0,
            survey_id: nanoid_gen(12),
            plaintext: parsed_survey.plaintext.clone(),
            user_id: user_id.to_owned(),
            created_at: metadata.created_at,
            modified_at: metadata.modified_at,
            version: "fixme".to_string(),
            parse_version: parsed_survey.parse_version.clone(),
        };

        // let new_survey = SurveyBuilder::default()
        //     .metadata(metadata)
        //     .survey(parsed_survey)
        //     .user_id(Some(user_id.to_owned()))
        //     .build()
        //     .expect("Could not create new survey model");

        let insert_result = state
            .db
            .create_survey(survey)
            .await
            .expect("Should create survey in database");

        info!("     ->> Inserted survey");
        // let response = CreateSurveyResponse { survey: new_survey };

        // return Ok(json!(response));

        return Ok(Json(json!({ "survey": parsed_survey })));
    }

    #[derive(Deserialize, Serialize, Debug)]
    #[serde(tag = "type", rename_all = "snake_case")]
    pub enum Answer {
        MultipleChoice { id: String, value: Vec<String> },
        Radio { id: String, value: String },
    }

    // #[derive(Deserialize, Serialize, Debug)]
    // pub enum Answers {
    //     id: String,
    //     value: Answer,
    // }

    // #[tracing::instrument]
    #[axum::debug_handler]
    pub async fn submit_survey(
        State(state): State<ServerState>,
        Path(survey_id): Path<String>,
        // mut multipart: Multipart,
        // ctx: Option<Ctext>,
        // extract::Json(payload): extract::Json<CreateAnswersRequest>,
        Json(payload): extract::Json<Value>, // for urlencoded
    ) -> Result<Json<Value>, ServerError> {
        info!("->> submit_survey");
        debug!("    ->> survey: {:#?}", payload);

        // json version
        let survey = match state
            .db
            .get_survey(&survey_id)
            .await
            .expect("Could not get survey from db")
        {
            Some(x) => x,
            None => {
                return Err(ServerError::BadRequest(
                    "Resource does not exist".to_string(),
                ))
            }
        };
        // info!("Found survey_id in database");
        // let answer_id = nanoid_gen(12);
        // let response = CreateAnswersResponse {
        //     answer_id: answer_id.clone(),
        // };
        let create_answer_model = CreateAnswersModel {
            id: None,
            answer_id: nanoid_gen(12),
            survey_id: survey_id.clone(),
            answers: json!(payload),
            submitted_at: chrono::Utc::now().to_string(),
            // external_id: "".to_string(),
            // survey_version: "".to_string(),
            // start_time: chrono::Local::now().to_string(),
            // end_time: "".to_string(),
            // created_at: "".to_string(),
        };

        let answer_result = state
            .db
            .create_answer(create_answer_model)
            .await
            .expect("Should create answer in database");

        info!("completed survey submit");

        return Ok(Json(json!({ "survey_id": survey_id })));
    }

    fn try_thing() -> Result<(), anyhow::Error> {
        anyhow::bail!("it failed!")
    }

    #[derive(Deserialize, Debug)]
    pub struct GetSurveyQuery {
        pub format: SurveyFormat,
    }

    // #[derive(Clone, Debug, Serialize, Deserialize, Model)]
    // pub struct SurveyModel {
    //     pub id: i32,
    //     pub plaintext: String,
    //     pub user_id: String,
    //     // pub created_at: String,
    //     // pub modified_at: String,
    //     // pub questions: Option<Vec<Question>>,
    //     // pub version: String,
    //     // pub parse_version: String,
    //     // #[serde(flatten)]
    //     // pub metadata: Metadata,
    // }

    #[derive(Clone, Debug, Serialize, Deserialize)]

    pub struct Metadata {
        pub metadata_id: String,
        pub created_at: DateTime<Utc>,
        pub modified_at: DateTime<Utc>,
        pub version: String,
    }

    impl Metadata {
        fn new() -> Self {
            Self {
                metadata_id: nanoid_gen(24),
                created_at: Utc::now(),
                modified_at: Utc::now(),
                version: "0".to_string(),
            }
        }
    }

    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "lowercase")]
    pub enum SurveyFormat {
        Html,
        Json,
    }

    #[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
    pub struct CreateSurveyRequest {
        pub plaintext: String,
    }

    // #[tracing::instrument]
    #[axum::debug_handler]
    pub async fn get_survey(
        State(_state): State<ServerState>,
        ctx: Option<Ctext>,
        // authorization: TypedHeader<Authorization<Bearer>>,
        Path(survey_id): Path<String>,
        Query(query): Query<GetSurveyQuery>,
    ) -> impl IntoResponse {
        let db_response = _state
            .db
            .get_survey(&survey_id)
            .await
            .expect("Did not find survey in db");
        // let response = CreateSurveyResponse::from(insert_result);
        info!("query: {query:#?}");
        println!("query: {query:#?}");

        // let results = transform_response(db_response, query);
        (StatusCode::OK, Json(db_response))
    }

    #[tracing::instrument]
    #[axum::debug_handler]
    pub async fn list_survey(
        state: State<ServerState>,
        ctx: Option<Ctext>,
        // State(state): State<ServerState>,
        headers: HeaderMap,
    ) -> anyhow::Result<Json<Value>, ServerError> {
        println!("context: {:?}", ctx);

        if ctx.is_none() {
            return Err(ServerError::AuthFailNoTokenCookie);
        }

        let user_id = &ctx.expect("Context should be available").user_id().clone();

        println!("Getting surveys for user={user_id}");
        let pool = &state.db.pool;

        let res: Vec<SurveyModel> =
            sqlx::query_as::<_, SurveyModel>("select * from surveys where surveys.user_id = $1")
                .bind(user_id)
                .fetch_all(pool)
                .await
                .unwrap();

        let resp = ListSurveyResponse { surveys: res };

        Ok(Json(json!(resp)))
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct ListSurveyResponse {
        pub surveys: Vec<SurveyModel>,
    }

    #[derive(Deserialize, Debug, Serialize)]
    pub struct LoginPayload {
        pub email: String,
        pub password: String,
    }

    pub async fn signup(
        // cookies: Cookies,
        // ctx: Option<Ctext>,
        state: State<ServerState>,
        payload: Json<LoginPayload>,
    ) -> anyhow::Result<Json<Value>, ServerError> {
        info!("->> signup");

        match state
            .db
            .get_user_by_email(payload.email.clone())
            .await
            .with_context(|| "Checking if user already exists")
        {
            Ok(_) => return Err(ServerError::UserAlreadyExists),
            Err(_) => {}
        };

        let argon2 = argon2::Argon2::default();
        let salt = SaltString::generate(OsRng);
        let hash = argon2
            .hash_password(payload.password.as_bytes(), &salt)
            .unwrap();
        let password_hash_string = hash.to_string();

        let user = match state
            .db
            .create_user(CreateUserRequest {
                email: payload.email.clone(),
                password_hash: hash.to_string(),
            })
            .await
        {
            Ok(user) => user,
            Err(e) => {
                println!("Could not create user, error in database: {e}");
                return Err(ServerError::WrongCredentials);
            }
        };

        let jwt_claim = create_jwt_claim(user.email.clone(), "somerole-pleasechange")?;

        // let auth_cookie = Cookie::build("x-auth-token", jwt_claim.token.clone())
        //     // .domain("localhost")
        //     .same_site(tower_cookies::cookie::SameSite::Strict)
        //     .expires(
        //         tower_cookies::cookie::time::OffsetDateTime::from_unix_timestamp(
        //             jwt_claim.expires as i64,
        //         )
        //         .unwrap(),
        //     )
        //     .secure(true)
        //     .http_only(true)
        //     .finish();
        // cookies.add(auth_cookie);

        return Ok(Json(
            json!({"email": user.email, "auth_token": jwt_claim.token}),
        ));
    }

    pub async fn authorize(
        // cookies: Cookies,
        // ctx: Result<Ctext, CustomError>,
        state: State<ServerState>,
        payload: Json<LoginPayload>,
    ) -> anyhow::Result<Json<Value>, ServerError> {
        info!("->> api_login");
        info!("Payload: {payload:#?}");

        if payload.email.is_empty() || payload.password.is_empty() {
            return Err(ServerError::MissingCredentials);
        }

        // look for email in database
        let user = match state
            .db
            .get_user_by_email(payload.email.clone())
            .await
            .with_context(|| "Could not get find user by email")
        {
            Ok(x) => x,
            Err(_) => {
                info!("Did not find user in database");
                return Err(ServerError::WrongCredentials);
            }
        };

        // check if password matches
        let argon2 = argon2::Argon2::default();

        let hash = PasswordHash::new(&user.password_hash).unwrap();
        let is_correct = match argon2.verify_password(&payload.password.as_bytes(), &hash) {
            Ok(_) => true,
            Err(_) => return Err(ServerError::AuthPasswordsDoNotMatch),
        };
        println!("      ->> password matches={is_correct}");
        let jwt = create_jwt_token(user)?;

        // TODO: create success body
        let username = payload.email.clone();
        let logged_in = true;

        info!("     ->> Success logging in");
        Ok(Json(
            json!({"result": logged_in, "username": username, "auth_token": jwt}),
        ))
    }

    // TODO: this is neat: https://github.com/jbertovic/svelte-axum-project/blob/main/back_end/src/services.rs
}
