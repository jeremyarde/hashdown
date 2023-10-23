pub mod routes {
    use chrono::{DateTime, Utc};

    use axum::{
        extract::Path,
        http::{self, HeaderMap, HeaderName, Request},
        middleware::{self, Next},
        response::IntoResponse,
        response::Response,
        routing::{get, post},
        Extension, Router,
    };

    use axum::{
        extract::{self, State},
        http::StatusCode,
        Json,
    };
    use hyper::{header::CONTENT_ENCODING, Method};
    use serde::{Deserialize, Serialize};
    use serde_json::{json, Value};

    use tower::ServiceBuilder;
    use tower_http::{cors::CorsLayer, trace::TraceLayer};
    use tracing::{debug, log::info};

    use crate::{
        auth::{self, validate_session_middleware},
        db::database::{CreateAnswersModel, Session},
        error::main_response_mapper,
        mware::ctext::Ctext,
        server::SurveyModel,
        survey_responses::survey_responses::{self, submit_response},
        ServerError, ServerState,
    };

    #[derive(Serialize)]
    struct CreateSurvey {
        plaintext: String,
    }

    use markdownparser::{nanoid_gen, parse_markdown_v3};

    #[derive(Deserialize, Serialize, Debug)]
    pub struct ListSurveyResponse {
        pub surveys: Vec<SurveyModel>,
    }

    #[derive(Deserialize, Debug, Serialize)]
    pub struct LoginPayload {
        pub email: String,
        pub password: String,
    }

    // pub fn get_router() -> anyhow::Result<Router> {
    //     get_routes(state)
    //     let auth_session_service = ServiceBuilder::new().layer(middleware::from_fn_with_state(
    //         state.clone(),
    //         validate_session_middleware,
    //     ));

    //     Router::new()
    //         .merge(get_routes(state).unwrap())
    //         .layer(corslayer)
    //         .layer(TraceLayer::new_for_http())
    //         .layer(auth_session_service)
    // }

    pub fn get_router(state: ServerState) -> anyhow::Result<Router> {
        let public_routes = Router::new()
            .route("/auth/login", post(auth::login))
            .route("/auth/signup", post(auth::signup))
            .route("/ping", get(ping));

        let auth_routes = Router::new()
            .route("/surveys", post(create_survey).get(list_survey))
            .route("/surveys/:id", get(get_survey).post(submit_survey))
            .route("/responses", post(submit_response))
            .route("/responses/:id", get(survey_responses::list_response));

        let all = public_routes
            .merge(auth_routes)
            .layer(middleware::map_response(main_response_mapper))
            .with_state(state.clone());

        // let auth_session_service = ServiceBuilder::new().layer(middleware::from_fn_with_state(
        //     state.clone(),
        //     validate_session_middleware,
        // ));

        let mut origins = vec![];
        info!("Starting app in stage={:?}", &state.config.stage);
        if state.config.is_dev() {
            origins.append(&mut vec![
                "http://localhost:3000".parse().unwrap(),
                "http://localhost:3001".parse().unwrap(),
                "http://localhost:8080".parse().unwrap(),
                "http://localhost:5173".parse().unwrap(),
                // "http://api.example.com".parse().unwrap(),
            ]);
        }

        let corslayer = CorsLayer::new()
            .allow_methods([Method::POST, Method::GET])
            .allow_headers([
                http::header::CONTENT_TYPE,
                http::header::ACCEPT,
                http::header::AUTHORIZATION,
                http::header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
                http::header::ACCESS_CONTROL_REQUEST_METHOD,
                HeaderName::from_static("x-auth-token"),
                HeaderName::from_static("x-sid"),
                HeaderName::from_static("session_id"),
                HeaderName::from_static("credentials"),
            ])
            // .allow_headers(Any)
            .allow_credentials(true)
            .allow_origin(origins)
            .expose_headers([CONTENT_ENCODING, HeaderName::from_static("session_id")]);

        let router = all.layer(corslayer).layer(TraceLayer::new_for_http());
        // .layer(auth_session_service);

        Ok(router)
    }

    async fn propagate_header<B>(req: Request<B>, next: Next<B>) -> Response {
        next.run(req).await
    }

    #[tracing::instrument]
    #[axum::debug_handler]
    pub async fn ping() -> anyhow::Result<Json<Value>, ServerError> {
        return Ok(Json(json!({"result": "Ok"})));
    }

    #[tracing::instrument]
    #[axum::debug_handler]
    pub async fn create_survey(
        headers: HeaderMap,
        State(state): State<ServerState>,
        Extension(ctx): Extension<Option<Ctext>>,
        extract::Json(payload): extract::Json<CreateSurveyRequest>,
    ) -> anyhow::Result<Json<Value>, ServerError> {
        info!("->> create_survey");

        let ctx = match &ctx {
            Some(x) => x,
            None => return Err(ServerError::AuthFailNoTokenCookie),
        };

        info!("Creating new survey for user={:?}", ctx.user_id);
        // Check that the survey is Ok
        let parsed_survey =
            parse_markdown_v3(payload.plaintext).expect("Could not parse the survey");

        let metadata = Metadata::new();

        let survey = SurveyModel {
            id: 0,
            survey_id: nanoid_gen(12),
            plaintext: parsed_survey.plaintext.clone(),
            user_id: ctx.user_id.to_owned(),
            created_at: metadata.created_at,
            modified_at: metadata.modified_at,
            version: "fixme".to_string(),
            parse_version: parsed_survey.parse_version.clone(),
        };

        let _insert_result = state
            .db
            .create_survey(survey)
            .await
            .expect("Should create survey in database");

        info!("     ->> Inserted survey");

        return Ok(Json(json!({ "survey": parsed_survey })));
    }

    #[derive(Deserialize, Serialize, Debug)]
    #[serde(tag = "type", rename_all = "snake_case")]
    pub enum Answer {
        MultipleChoice { id: String, value: Vec<String> },
        Radio { id: String, value: String },
    }

    #[tracing::instrument]
    #[axum::debug_handler]
    pub async fn submit_survey(
        State(state): State<ServerState>,
        Path(survey_id): Path<String>,
        Extension(ctx): Extension<Option<Ctext>>,
        Json(payload): extract::Json<Value>, // for urlencoded
    ) -> Result<Json<Value>, ServerError> {
        info!("->> submit_survey");
        debug!("    ->> survey: {:#?}", payload);

        // json version
        let _survey = match state
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

        state
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
        Extension(ctx): Extension<Option<Ctext>>,
        // authorization: TypedHeader<Authorization<Bearer>>,
        Path(survey_id): Path<String>,
    ) -> impl IntoResponse {
        let db_response = _state
            .db
            .get_survey(&survey_id)
            .await
            .expect("Did not find survey in db");
        // let response = CreateSurveyResponse::from(insert_result);

        // let results = transform_response(db_response, query);
        (StatusCode::OK, Json(db_response))
    }

    #[tracing::instrument]
    #[axum::debug_handler]
    pub async fn list_survey(
        state: State<ServerState>,
        Extension(ctx): Extension<Option<Ctext>>,
        // State(state): State<ServerState>,
        session: Extension<Session>,
        headers: HeaderMap,
    ) -> anyhow::Result<Json<Value>, ServerError> {
        println!("context: {:?}", ctx);

        let ctx = if ctx.is_none() {
            return Err(ServerError::AuthFailNoTokenCookie);
        } else {
            ctx.unwrap()
        };

        let user_id = &ctx.user_id().clone();

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
}
