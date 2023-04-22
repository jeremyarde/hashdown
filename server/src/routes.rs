pub mod routes {
    use axum::{
        extract::{DefaultBodyLimit, Multipart, Path, Query},
        http::{self, HeaderMap, HeaderName, HeaderValue, Method, Uri},
        middleware,
        response::IntoResponse,
        response::Response,
        routing::{get, get_service, post, MethodRouter},
        Extension, Router,
    };
    use db::{
        database::Database,
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
        mware::{
            self,
            ctext::{mw_ctx_resolver, Ctext},
        },
        server::CreateSurveyResponse,
        CustomError, ServerState,
    };

    // pub fn get_routes(state: ServerState) -> Router {
    //     let t = Router::new()
    //         // .layer(Extension(state))
    //         .route(&format!("/surveys"), post(create_survey).get(list_survey))
    //         .route(&format!("/surveys/test"), post(test_survey))
    //         .route(&format!("/surveys/:id"), get(get_survey))
    //         .route(&format!("/submit"), post(submit_survey))
    //         .route(&format!("login"), post(api_login))
    //         .with_state(state);
    //     return t;
    // }
    // use mware::middleware_require_auth;

    pub fn get_routes(state: ServerState) -> Router {
        let survey_routes: Router = Router::new()
            .route(&format!("/surveys/:id"), get(get_survey))
            .route(&format!("/submit"), post(submit_survey))
            .with_state(state.clone())
            .route_layer(middleware::from_fn_with_state(
                state.clone(),
                mw_ctx_resolver,
            ));

        let t = Router::new()
            // .layer(Extension(state))
            .route(&format!("/surveys"), post(create_survey).get(list_survey))
            .route(&format!("/surveys/test"), post(test_survey))
            .route(&format!("/login"), post(api_login))
            .layer(middleware::map_response(main_response_mapper))
            .with_state(state.clone());
        // .layer(Extension(state));
        // .with_state(state);

        return t;
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
        let service_error = res.extensions().get::<CustomError>();
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

    async fn log_request(
        uuid: Uuid,
        req_method: Method,
        uri: Uri,
        ctx: Option<Ctext>,
        service_error: Option<&CustomError>,
        client_error: Option<crate::error::ClientError>,
    ) {
        println!("logging request...");
    }

    #[tracing::instrument]
    #[axum::debug_handler]
    pub async fn create_survey(
        headers: HeaderMap,
        State(state): State<ServerState>,
        extract::Json(payload): extract::Json<CreateSurveyRequest>,
    ) -> impl IntoResponse {
        info!("Called create survey");

        // Check user + type, can they make surveys?
        let testuser = HeaderValue::from_str("").unwrap();
        let user_id = headers
            .get("x-user-id")
            .unwrap_or(&testuser)
            .to_str()
            .unwrap();

        println!("Creating new survey for user={user_id:?}");
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
        (StatusCode::CREATED, Json(response))
    }

    #[tracing::instrument]
    #[axum::debug_handler]
    pub async fn submit_survey(
        State(state): State<ServerState>,
        // mut multipart: Multipart,
        extract::Json(payload): extract::Json<CreateAnswersRequest>,
    ) -> Result<Json<CreateAnswersResponse>, CustomError> {
        // let content_type_header = req.headers().get(CONTENT_TYPE);
        // let content_type => content_type_header.and_then(|value| value.to_str().ok());

        info!("Called submit survey");

        info!("Found survey_id in request");

        let survey = match state.db.get_survey(&payload.survey_id).await.unwrap() {
            Some(x) => x,
            None => return Err(CustomError::BadRequest("another issue".to_string())),
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
        state: Extension<ServerState>,
        ctx: Result<Ctext, CustomError>,
        // State(state): State<ServerState>,
        headers: HeaderMap,
    ) -> impl IntoResponse {
        println!("Recieved headers={headers:#?}");
        let testuser = HeaderValue::from_str("").unwrap();
        let user_id = headers
            .get("x-user-id")
            .unwrap_or(&testuser)
            .to_str()
            .unwrap();

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

        (StatusCode::OK, Json(resp))
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct ListSurveyResponse {
        pub surveys: Vec<SurveyModel>,
    }

    #[derive(Deserialize, Debug, Serialize)]
    pub struct LoginPayload {
        pub username: String,
        pub password: String,
    }
    use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
    #[derive(Debug, Serialize, Deserialize)]
    struct Claims {
        sub: String,
        company: String,
        exp: usize,
    }

    pub async fn api_login(
        cookies: Cookies,
        // ctx: Result<Ctext, CustomError>,
        state: ServerState,
        payload: Json<LoginPayload>,
    ) -> Result<Json<Value>, CustomError> {
        info!("api_login");

        // println!("ctx in login: {ctx:?}");
        // TODO: real db auth
        let claim = Claims {
            sub: payload.username.clone(),
            company: "audience".to_string(),
            exp: 1,
        };

        let jwt = match encode(&Header::default(), &claim, &EncodingKey::from_secret(key)) {
            Ok(t) => t,
            Err(_) => {
                return Err(CustomError::BadRequest(
                    "yo this request is messed".to_string(),
                ))
            }
        };

        // TODO: set cookies
        cookies.add(Cookie::new("x-auth-token", jwt));

        // TODO: create success body
        let username = payload.username.clone();
        let logged_in = true;
        Ok(Json(json!({"result": logged_in, "username": username})))
    }
}
