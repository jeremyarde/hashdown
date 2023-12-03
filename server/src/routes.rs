pub mod routes {

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

    use tower_http::{cors::CorsLayer, trace::TraceLayer};
    use tracing::{debug, log::info};

    use crate::{
        auth::{self, validate_session_middleware},
        constants::SESSION_ID_KEY,
        db::database::CreateAnswersModel,
        error::main_response_mapper,
        mware::ctext::Ctext,
        server::SurveyModel,
        survey_responses::{self, submit_response},
        ServerError, ServerState,
    };

    #[derive(Serialize)]
    struct CreateSurvey {
        plaintext: String,
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

    pub fn get_router(state: ServerState) -> anyhow::Result<Router> {
        let public_routes = Router::new()
            .route("/api/auth/login", post(auth::login))
            .route("/api/auth/signup", post(auth::signup))
            .route("/api/submit", post(submit_response))
            .route("/api/surveys/:id", get(get_survey).post(submit_survey))
            .route("/api/ping", get(ping));

        let auth_routes = Router::new()
            .route("/api/surveys", post(create_survey).get(list_survey))
            .route("/api/responses", get(survey_responses::list_response))
            .route_layer(middleware::from_fn_with_state(
                state.clone(),
                validate_session_middleware,
            ));

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
                // "http://localhost:3000".parse().unwrap(),
                // "http://localhost:3001".parse().unwrap(),
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
        // Extension(ctx): Extension<Option<Ctext>>,
        extract::Json(payload): extract::Json<CreateSurveyRequest>,
    ) -> anyhow::Result<Json<Value>, ServerError> {
        info!("->> create_survey");

        // let ctx = Some(Ctext::new(String::from(""), Session::new()));
        let session_id = headers
            .get(SESSION_ID_KEY)
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let session = state.db.get_session(session_id).await.unwrap();

        // let ctx = match &ctx {
        //     Some(x) => x,
        //     None => return Err(ServerError::AuthFailNoTokenCookie),
        // };

        info!("Creating new survey for user={:?}", session.user_id);
        // Check that the survey is Ok

        let survey = SurveyModel::new(payload, &session);

        let insert_result: SurveyModel = state
            .db
            .create_survey(survey)
            .await
            .map_err(|x| {
                ServerError::Database(format!("Could not create new survey: {x}").to_string())
            })
            .unwrap();

        info!("     ->> Inserted survey");

        return Ok(Json(json!({ "survey": insert_result })));
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
        // let _survey = match state
        //     .db
        //     .get_survey(&survey_id)
        //     .await
        //     .expect("Could not get survey from db")
        // {
        //     Some(x) => x,
        //     None => {
        //         return Err(ServerError::BadRequest(
        //             "Resource does not exist".to_string(),
        //         ))
        //     }
        // };
        let create_answer_model = CreateAnswersModel {
            survey_id: survey_id.clone(),
            responses: payload.get("responses").unwrap().to_owned(),
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
        // Extension(ctx): Extension<Option<Ctext>>,
        // authorization: TypedHeader<Authorization<Bearer>>,
        Path(survey_id): Path<String>,
    ) -> anyhow::Result<Json<Value>, ServerError> {
        let db_response = match _state.db.get_survey(&survey_id).await {
            Ok(x) => x,
            Err(err) => return Err(ServerError::Database("Could not get survey".to_string())),
        };

        Ok(Json(json!(db_response)))
    }

    #[tracing::instrument]
    #[axum::debug_handler]
    pub async fn list_survey(
        state: State<ServerState>,
        Extension(session): Extension<Ctext>,
        // headers: HeaderMap,
    ) -> anyhow::Result<Json<Value>, ServerError> {
        info!("->> list_survey");
        // println!("context: {:?}", ctx);

        // let ctx = if ctx.is_none() {
        //     return Err(ServerError::AuthFailNoTokenCookie);
        // } else {
        //     ctx.unwrap()
        // };

        // let user_id = &ctx.user_id().clone();

        println!("Getting surveys for user={}", session.user_id);
        let pool = &state.db.pool;

        let res = sqlx::query_as::<_, SurveyModel>(
            "select * from mdp.surveys where mdp.surveys.user_id = $1",
        )
        .bind(session.user_id.clone())
        .fetch_all(pool)
        .await
        .map_err(|err| ServerError::Database(err.to_string()))
        .unwrap();

        let resp = ListSurveyResponse { surveys: res };

        Ok(Json(json!(resp)))
    }
}
