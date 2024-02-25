use axum::{
    http::Method,
    middleware,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use tower_http::{
    cors::CorsLayer,
    services::{ServeDir, ServeFile},
};
use tracing::info;

use crate::{
    api::{list_survey, submit_response},
    auth::{self, validate_session_middleware},
    db::{
        database::MdpSurvey,
        surveys::{create_survey, get_survey, submit_survey},
    },
    error::main_response_mapper,
    survey_responses,
    webhook::{self, echo},
    ServerError, ServerState,
};

#[derive(Serialize)]
struct CreateSurvey {
    plaintext: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ListSurveyResponse {
    pub surveys: Vec<MdpSurvey>,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct LoginPayload {
    // pub name: String,
    pub email: String,
    pub password: String,
}

pub async fn hello() -> Response {
    Response::new("Hi!".into())
}

trait ApiRoutes {
    fn hello();
}

#[axum::debug_handler]
async fn serve_folder() -> Response {
    return Response::new("<div>Hi from my folder funciton</div>".into());
}

pub fn get_router(state: ServerState) -> anyhow::Result<Router> {
    // let rate_limit = ServiceBuilder::new()
    //     .layer(BufferLayer::new(1024))
    //     .layer(RateLimitLayer::new(5, Duration::from_secs(1)));

    let public_routes = Router::new()
        .route("/v1/hello", get(hello))
        .route("/v1/auth/login", post(auth::login))
        .route("/v1/auth/signup", post(auth::signup))
        // .route("/v1/auth/remove", post(auth::delete))
        .route("/v1/submit", post(submit_response))
        .route("/v1/auth/confirm", get(auth::confirm))
        // .route("/v1/payment/success", post(payments::echo))
        .route("/v1/webhook", post(webhook::echo))
        .route("/v1/surveys/:id", get(get_survey).post(submit_survey));

    let auth_routes = Router::new()
        .route("/v1/auth/logout", get(auth::logout))
        .route("/v1/surveys", post(create_survey).get(list_survey))
        .route("/v1/responses", get(survey_responses::list_response))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            validate_session_middleware,
        ));

    // This does work
    // let static_routes = Router::new()
    //     .nest_service(
    //         "/",
    //         ServeDir::new("/Users/jarde/Documents/code/markdownparser/ui-vite/dist"),
    //     )
    //     .nest_service(
    //         "/assets",
    //         ServeDir::new("/Users/jarde/Documents/code/markdownparser/ui-vite/dist/assets"),
    //     );

    let mut origins = vec![];
    info!("Starting app in stage={:?}", &state.config.stage);
    if state.config.is_dev() {
        origins.append(&mut vec![
            //     "http://localhost:8080".parse().unwrap(),
            "http://localhost:5173".parse().unwrap(), //     "http:127.0.0.1:5173".parse().unwrap(),
                                                      // "https://mdp-api.onrender.com".parse().unwrap(),
                                                      // "https://gethashdown.com".parse().unwrap(),
        ]);
    }
    if state.config.is_prod() {
        origins.append(&mut vec![
            //     "http://localhost:8080".parse().unwrap(),
            // "http://localhost:5173".parse().unwrap(), //     "http:127.0.0.1:5173".parse().unwrap(),
            // "https://mdp-api.onrender.com".parse().unwrap(),
            "https://gethashdown.com".parse().unwrap(),
        ]);
    }

    let corslayer = CorsLayer::new()
        .allow_methods([Method::POST, Method::GET])
        .allow_headers([
            axum::http::header::CONTENT_TYPE,
            axum::http::header::ACCEPT,
            // axum::http::header::AUTHORIZATION,
            // axum::http::header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
            // axum::http::header::ACCESS_CONTROL_REQUEST_METHOD,
            // axum::http::HeaderName::from_static("x-auth-token"),
            // axum::http::HeaderName::from_static("x-sid"),
            axum::http::HeaderName::from_static("session_id"),
            // axum::http::HeaderName::from_static("credentials"),
        ])
        // .allow_headers(Any)
        // .allow_credentials(true)
        .allow_origin(origins)
        // .allow_origin(Any)
        .expose_headers([
            axum::http::header::CONTENT_ENCODING,
            axum::http::HeaderName::from_static("session_id"),
        ]);

    // let corslayer = CorsLayer::new();
    // .allow_origin(origins)
    // .expose_headers([CONTENT_ENCODING, HeaderName::from_static("session_id")]);

    let all = public_routes
        .merge(auth_routes)
        // .merge(static_routes)
        .layer(middleware::map_response(main_response_mapper))
        .with_state(state.clone())
        .layer(corslayer);
    // .layer(BufferLayer::new(1024))
    // .layer(RateLimitLayer::new(5, Duration::from_secs(1)))

    // .layer(
    //     ServiceBuilder::new()
    //         .layer(BufferLayer::new(1024))
    //         .layer(RateLimitLayer::new(5, Duration::from_secs(1))),
    // );

    // let router = all.layer(corslayer);
    // .layer(auth_session_service);

    Ok(all)
}

// async fn propagate_header<B>(req: Request<Body>, next: Next) -> Response {
//     next.run(req).await
// }

#[tracing::instrument]
#[axum::debug_handler]
pub async fn ping() -> anyhow::Result<Json<Value>, ServerError> {
    return Ok(Json(json!({"result": "Ok"})));
}

// #[tracing::instrument]
// #[axum::debug_handler]
// pub async fn create_survey(
//     headers: HeaderMap,
//     State(state): State<ServerState>,
//     ctx: Extension<Option<Ctext>>,
//     extract::Json(payload): extract::Json<CreateSurveyRequest>,
// ) -> anyhow::Result<Json<Value>, ServerError> {
//     info!("->> create_survey");
//     info!("Creating new survey for user={:?}", ctx.session.user_id);

//     let survey = SurveyModel::new(payload, &ctx.session);

//     let insert_result: SurveyModel = state
//         .db
//         .create_survey(survey, &ctx.session.workspace_id)
//         .await
//         .map_err(|x| ServerError::Database(format!("Could not create new survey: {x}").to_string()))
//         .unwrap();

//     info!("     ->> Inserted survey");

//     return Ok(Json(json!({ "survey": insert_result })));
// }

// #[tracing::instrument]
// #[axum::debug_handler]
// pub async fn submit_survey(
//     State(state): State<ServerState>,
//     Path(survey_id): Path<String>,
//     // Extension(ctx): Extension<Option<Ctext>>,
//     Json(payload): extract::Json<SubmitResponseRequest>, // for urlencoded
// ) -> Result<Json<Value>, ServerError> {
//     info!("->> submit_survey");
//     debug!("    ->> survey: {:#?}", payload);

//     state
//         .db
//         .create_answer(payload)
//         .await
//         .expect("Should create answer in database");

//     info!("completed survey submit");

//     return Ok(Json(json!({ "survey_id": survey_id })));
// }

// #[derive(Deserialize, Debug)]
// pub struct GetSurveyQuery {
//     pub format: SurveyFormat,
// }

// #[derive(Deserialize, Debug)]
// #[serde(rename_all = "lowercase")]
// pub enum SurveyFormat {
//     Html,
//     Json,
// }

// #[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
// pub struct CreateSurveyRequest {
//     pub plaintext: String,
//     pub organization: Option<String>,
// }

// // #[tracing::instrument]
// #[axum::debug_handler]
// pub async fn get_survey(
//     State(_state): State<ServerState>,
//     // Extension(ctx): Extension<Option<Ctext>>,
//     // authorization: TypedHeader<Authorization<Bearer>>,
//     Path(survey_id): Path<String>,
// ) -> anyhow::Result<Json<Value>, ServerError> {
//     let db_response = match _state.db.get_survey(&survey_id).await {
//         Ok(x) => x,
//         Err(_err) => return Err(ServerError::Database("Could not get survey".to_string())),
//     };

//     Ok(Json(json!(db_response)))
// }

// #[tracing::instrument]
// #[axum::debug_handler]
// pub async fn list_survey(
//     state: State<ServerState>,
//     Extension(session): Extension<Option<Ctext>>,
//     // headers: HeaderMap,
// ) -> anyhow::Result<Json<Value>, ServerError> {
//     info!("->> list_survey");
//     // println!("context: {:?}", ctx);

//     // let ctx = if ctx.is_none() {
//     //     return Err(ServerError::AuthFailNoTokenCookie);
//     // } else {
//     //     ctx.unwrap()
//     // };

//     // let user_id = &ctx.user_id().clone();

//     println!("Getting surveys for user={}", session.user_id);
//     let pool = &state.db.pool;

//     let res = sqlx::query_as::<_, SurveyModel>(
//         "select * from mdp.surveys where mdp.surveys.user_id = $1",
//     )
//     .bind(session.user_id.clone())
//     .fetch_all(pool)
//     .await
//     .map_err(|err| ServerError::Database(err.to_string()))
//     .unwrap();

//     let resp = ListSurveyResponse { surveys: res };

//     Ok(Json(json!(resp)))
// }
