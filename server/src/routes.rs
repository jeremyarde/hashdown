use axum::{
    error_handling::HandleErrorLayer,
    http::Method,
    response::Response,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use tower_http::cors::CorsLayer;
use tracing::info;

use crate::{
    api::{list_survey, submit_response},
    auth::{self},
    db::{
        database::MdpSurvey,
        surveys::{create_survey, get_survey, submit_survey},
    },
    error::handle_error,
    stripe, survey_responses,
    webhook::{self},
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

#[axum::debug_handler]
async fn serve_folder() -> Response {
    Response::new("<div>Hi from my folder funciton</div>".into())
}

pub fn get_router(state: ServerState) -> anyhow::Result<Router> {
    // let rate_limit = ServiceBuilder::new()
    //     .layer(BufferLayer::new(1024))
    //     .layer(RateLimitLayer::new(5, Duration::from_secs(1)));

    let public_routes = Router::new()
        // .route("/v1/hello", get(hello))
        .route("/v1/auth/login", post(auth::login))
        .route("/v1/auth/signup", post(auth::signup))
        // .route("/v1/auth/remove", post(auth::delete))
        .route("/v1/submit", post(submit_response))
        .route("/v1/auth/confirm", get(auth::confirm))
        // .route("/v1/payment/success", post(payments::echo))
        .route("/v1/webhook", post(webhook::handle_stripe_webhook))
        .route("/v1/health", get(ping))
        .route(
            "/v1/create-checkout-session",
            post(stripe::checkout_session),
        )
        .route("/v1/surveys/:id", get(get_survey).post(submit_survey));

    let auth_routes = Router::new()
        .route("/v1/auth/logout", get(auth::logout))
        .route("/v1/surveys", post(create_survey).get(list_survey))
        .route("/v1/responses", get(survey_responses::list_response));
    // stripe related
    // .route_layer(middleware::from_fn_with_state(
    //     state.clone(),
    //     validate_session_middleware,
    // ));

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
        // .layer(middleware::map_response(main_response_mapper))
        .with_state(state.clone())
        // .layer(HandleErrorLayer::new(handle_timeout_error))
        // .timeout(Duration::from_secs(30))
        // .layer(handle(handle_error))
        .layer(corslayer);
    // .layer(HandleErrorLayer::new(handle_error));
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

// async fn handle_error(
//     // `Method` and `Uri` are extractors so they can be used here
//     method: Method,
//     uri: Uri,
//     // the last argument must be the error itself
//     err: BoxError,
// ) -> (StatusCode, String) {
//     (
//         StatusCode::INTERNAL_SERVER_ERROR,
//         format!("`{method} {uri}` failed with {err}"),
//     )
// }

// async fn propagate_header<B>(req: Request<Body>, next: Next) -> Response {
//     next.run(req).await
// }

#[tracing::instrument]
#[axum::debug_handler]
pub async fn ping() -> anyhow::Result<Json<Value>, ServerError> {
    return Ok(Json(json!({"result": "Ok"})));
}
