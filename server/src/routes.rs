pub mod routes {
    use anyhow::Context;
    use argon2::{
        password_hash::{rand_core::OsRng, SaltString},
        Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    };
    use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

    use axum::{
        body::Body,
        extract::{DefaultBodyLimit, Multipart, Path, Query},
        http::{self, HeaderMap, HeaderName, HeaderValue, Method, Uri},
        middleware,
        response::IntoResponse,
        response::Response,
        routing::{get, get_service, post, MethodRouter},
        Extension, Router,
    };
    use db::{
        database::{CreateUserRequest, Database},
        models::{
            CreateAnswersModel, CreateAnswersRequest, CreateAnswersResponse, CreateSurveyRequest,
            SurveyModel, SurveyModelBuilder,
        },
    };
    use markdownparser::{nanoid_gen, parse_markdown_v3, MetadataBuilder, Survey, SurveyBuilder};
    // use oauth2::basic::BasicClient;

    use serde::{Deserialize, Serialize};
    use serde_json::{json, Value};
    use tower_cookies::{Cookie, Cookies};
    use tracing::log::info;
    // use uuid::Uuid;
    // use sqlx::{Sqlite, SqlitePool};
    // use tower_http::http::cors::CorsLayer;
    use axum::{
        extract::{self, State},
        http::StatusCode,
        Json,
    };
    use tower_http::{
        cors::{Any, CorsLayer},
        limit::RequestBodyLimitLayer,
        services::ServeDir,
        trace::TraceLayer,
    };
    use uuid::Uuid;

    // use markdownparser::{nanoid_gen, parse_markdown_v3, MetadataBuilder, SurveyBuilder};

    use crate::{
        auth::{validate_credentials, AuthError},
        log::log_request,
        mware::{
            self,
            ctext::{create_jwt_claim, mw_ctx_resolver, Ctext},
        },
        server::CreateSurveyResponse,
        ServerError, ServerState,
    };

    pub fn get_routes(state: ServerState) -> anyhow::Result<Router> {
        // let survey_routes: Router = Router::new()
        //     .route(&format!("/surveys/:id"), get(get_survey))
        //     .route(&format!("/submit"), post(submit_survey))
        //     .with_state(state.clone())
        //     .route_layer(middleware::from_fn_with_state(
        //         state.clone(),
        //         mw_ctx_resolver,
        //     ));

        let routes = Router::new()
            // .merge(survey_routes)
            // .layer(Extension(state))
            .route(&format!("/surveys"), post(create_survey).get(list_survey))
            .route(&format!("/surveys/test"), post(test_survey))
            .route(&format!("/login"), post(api_login))
            .route("/signup", post(signup))
            // .with_state(state.clone())
            // .route_layer(middleware::from_fn_with_state(
            //     state.clone(),
            //     mw_ctx_resolver,
            // ))
            .layer(middleware::from_fn_with_state(
                state.clone(),
                mw_ctx_resolver,
            ))
            .layer(middleware::map_response(main_response_mapper))
            .with_state(state);
        // .layer(Extension(state));
        // .with_state(state);

        return Ok(routes);
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
        let error_response = client_status_error
            .as_ref()
            .map(|(status_code, client_error)| {
                let client_error_body = json!({
                    "error": {
                        "type": client_error.as_ref(),
                        "req_uuid": uuid.to_string(),
                    }
                });

                println!("    ->> client_error_body: {client_error_body}");

                // Build the new response from the client_error_body
                (*status_code, Json(client_error_body)).into_response()
            });

        // Build and log the server log line.
        let client_error = client_status_error.unzip().1;
        log_request(uuid, req_method, uri, ctx, service_error, client_error).await;

        println!();
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

    #[tracing::instrument]
    #[axum::debug_handler]
    pub async fn create_survey(
        headers: HeaderMap,
        State(state): State<ServerState>,
        ctx: Option<Ctext>,
        extract::Json(payload): extract::Json<CreateSurveyRequest>,
    ) -> anyhow::Result<Json<Value>, ServerError> {
        info!("Called create survey");

        let user_id = match &ctx {
            Some(x) => x.user_id(),
            None => return Err(ServerError::AuthFailNoTokenCookie),
        };
        info!("Creating new survey for user={user_id:?}");
        // Check that the survey is Ok
        let parsed_survey = parse_markdown_v3(payload.plaintext);

        let metadata = MetadataBuilder::default().build().unwrap();

        let survey_model = SurveyModelBuilder::default()
            .id(metadata.id.clone())
            .plaintext(parsed_survey.plaintext.clone())
            .parse_version(parsed_survey.parse_version.clone())
            .user_id(user_id.to_string())
            .created_at(metadata.created_at.clone())
            .modified_at(metadata.modified_at.clone())
            .version(metadata.version.clone())
            .build()
            .unwrap();

        let new_survey = SurveyBuilder::default()
            .metadata(metadata)
            .survey(parsed_survey)
            .user_id(user_id.to_string())
            .build()
            .unwrap();

        let insert_result = state.db.create_survey(survey_model).await.unwrap();
        let response = CreateSurveyResponse { survey: new_survey };

        return Ok(Json(json!(response)));
    }

    #[tracing::instrument]
    #[axum::debug_handler]
    pub async fn submit_survey(
        State(state): State<ServerState>,
        // mut multipart: Multipart,
        ctx: Option<Ctext>,
        extract::Json(payload): extract::Json<CreateAnswersRequest>,
    ) -> Result<Json<CreateAnswersResponse>, ServerError> {
        // let content_type_header = req.headers().get(CONTENT_TYPE);
        // let content_type => content_type_header.and_then(|value| value.to_str().ok());

        info!("Called submit survey");
        // check

        info!("Found survey_id in request");

        let survey = match state.db.get_survey(&payload.survey_id).await.unwrap() {
            Some(x) => x,
            None => return Err(ServerError::BadRequest("another issue".to_string())),
        };
        info!("Found survey_id in database");
        let answer_id = nanoid_gen();
        let response = CreateAnswersResponse {
            answer_id: answer_id.clone(),
        };
        let create_answer_model = CreateAnswersModel {
            id: None,
            answer_id: answer_id,
            external_id: "".to_string(),
            survey_id: payload.survey_id,
            survey_version: "".to_string(),
            start_time: chrono::Local::now().to_string(),
            end_time: "".to_string(),
            answers: payload.answers,
            created_at: "".to_string(),
        };

        let answer_result = state.db.create_answer(create_answer_model).await.unwrap();

        info!("completed survey submit");

        return Ok(Json(response));
    }

    fn try_thing() -> Result<(), anyhow::Error> {
        anyhow::bail!("it failed!")
    }

    #[derive(Deserialize, Debug)]
    pub struct GetSurveyQuery {
        pub format: SurveyFormat,
    }

    #[derive(Deserialize, Debug)]
    #[serde(rename_all = "lowercase")]
    pub enum SurveyFormat {
        Html,
        Json,
    }

    // #[tracing::instrument]
    #[axum::debug_handler]
    pub async fn get_survey(
        State(_state): State<ServerState>,
        ctx: Option<Ctext>,
        Path(survey_id): Path<String>,
        Query(query): Query<GetSurveyQuery>,
    ) -> impl IntoResponse {
        let db_response = _state.db.get_survey(&survey_id).await.unwrap();
        // let response = CreateSurveyResponse::from(insert_result);
        info!("query: {query:#?}");
        println!("query: {query:#?}");

        // let results = transform_response(db_response, query);
        (StatusCode::OK, Json(db_response))
    }

    #[tracing::instrument]
    #[axum::debug_handler]
    pub async fn test_survey(
        // State(_state): State<ServerState>,
        extract::Json(payload): extract::Json<CreateSurveyRequest>,
    ) -> impl IntoResponse {
        let survey = parse_markdown_v3(payload.plaintext.clone());
        (StatusCode::OK, Json(survey))
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

        // println!("Recieved headers={headers:#?}");
        // let testuser = HeaderValue::from_str("").unwrap();
        // let user_id = headers
        //     .get("x-user-id")
        //     .unwrap_or(&testuser)
        //     .to_str()
        //     .unwrap();
        let user_id = &ctx.expect("Context should be available").user_id().clone();

        println!("Getting surveys for user={user_id}");
        let pool = &state.db.pool;

        // let count: i64 = sqlx::query_scalar("select count(*) from surveys where surveys.id = $1")
        //     .bind(user_id)
        //     .fetch_one(&pool)
        //     .await
        //     .map_err(internal_error)
        //     .unwrap();
        // println!("Survey count: {count:#?}");

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
        cookies: Cookies,
        ctx: Option<Ctext>,
        state: State<ServerState>,
        payload: Json<LoginPayload>,
    ) -> anyhow::Result<Json<Value>, ServerError> {
        info!("signup");

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
                return Err(ServerError::LoginFail);
            }
        };

        let jwt_claim = create_jwt_claim(user.email.clone(), "somerole-pleasechange")?;
        let auth_cookie = Cookie::build("x-auth-token", jwt_claim.token)
            // .domain("localhost")
            .same_site(tower_cookies::cookie::SameSite::Strict)
            .expires(
                tower_cookies::cookie::time::OffsetDateTime::from_unix_timestamp(
                    jwt_claim.expires as i64,
                )
                .unwrap(),
            )
            .secure(true)
            .http_only(true)
            .finish();
        cookies.add(auth_cookie);

        return Ok(Json(json!(user.email)));
    }

    pub async fn api_login(
        cookies: Cookies,
        // ctx: Result<Ctext, CustomError>,
        state: State<ServerState>,
        payload: Json<LoginPayload>,
    ) -> anyhow::Result<Json<Value>, ServerError> {
        info!("api_login");

        // look for email in database
        let user = match state
            .db
            .get_user_by_email(payload.email.clone())
            .await
            .with_context(|| "Could not get find user by email")
        {
            Ok(x) => x,
            Err(_) => {
                println!("Did not find user in database");
                return Err(ServerError::LoginFail);
            }
        };

        // let password = "mypassword";
        let argon2 = argon2::Argon2::default();
        // let salt = SaltString::generate(OsRng);
        // let hash: PasswordHash = argon2.hash_password(password.as_bytes(), &salt).unwrap();
        // let password_hash_string = hash.to_string();
        // PasswordHash::generate(phf, password, salt)
        // let hash = PasswordHash::new(&password_hash_string).unwrap();

        let hash = PasswordHash::new(&user.password_hash).unwrap();
        let is_correct = match argon2.verify_password(&payload.password.as_bytes(), &hash) {
            Ok(_) => true,
            Err(_) => return Err(ServerError::AuthPasswordsDoNotMatch),
        };
        println!("password matches={is_correct}");
        // login and check database
        // match validate_credentials("passwordhash", payload.password) {};

        // start building token
        let jwt = create_jwt_claim(user.email, "randomrole- please update")?;

        // TODO: set cookies
        cookies.add(Cookie::new("x-auth-token", jwt.token));

        // TODO: create success body
        let username = payload.email.clone();
        let logged_in = true;
        Ok(Json(json!({"result": logged_in, "username": username})))
    }
}
