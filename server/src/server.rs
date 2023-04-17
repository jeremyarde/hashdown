use axum::{
    extract::{DefaultBodyLimit, Multipart, Query},
    http::{self, HeaderMap, HeaderName, HeaderValue, Method, Response},
    middleware,
    response::IntoResponse,
    Extension, Router,
};
use db::database::Database;
use markdownparser::{nanoid_gen, parse_markdown_v3, MetadataBuilder, Survey, SurveyBuilder};

use tokio::{fs, task::JoinHandle};
use tower_cookies::CookieManagerLayer;
use tracing::log::info;
// use uuid::Uuid;
// use sqlx::{Sqlite, SqlitePool};
// use tower_http::http::cors::CorsLayer;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
// use yewui::runapp;

// use crate::answer::post_answer;
use crate::{
    routes::routes::{
        api_login, create_survey, get_routes, get_survey, list_survey, submit_survey, test_survey,
    },
    ServerState,
};

// use tower_http::trace::TraceLayer;
// use tower::http
pub struct ServerApplication {
    pub base_url: SocketAddr,
    pub server: JoinHandle<()>,
    // pub oauth_client: Option<BasicClient>,
}

// async fn uiapp(State(_state): State<ServerState>) -> impl IntoResponse {
//     println!("Rendering ui app, curr state: {_state:?}");

//     Html(mainapp::dioxusapp().await)
//     // Html("This is great")
// }
// use include_dir::{include_dir, Dir};

// static STATIC_DIR: Dir<'_> = include_dir!("./ui/public");

// async fn runyew() -> impl IntoResponse {
//     Html(runapp().await)
// }
// use self::routes;
// mod routes;

impl ServerApplication {
    pub async fn get_router() -> Router {
        let db = Database::new(true)
            .await
            .expect("Database was not created.");
        let state = ServerState { db: db };

        let corslayer = CorsLayer::new()
            .allow_methods([Method::POST, Method::GET])
            // .allow_headers([
            //     http::header::CONTENT_TYPE,
            //     http::header::ACCEPT,
            //     HeaderName::from_str("x-user-id").unwrap(),
            // ])
            .allow_headers(Any)
            .allow_origin(Any);

        // let corslayer = CorsLayer::new().allow_headers(Any);

        // let static_dir = "./dist";

        // build our application with a route
        let app = Router::new()
            // .route(&format!("/surveys"), post(create_survey).get(list_survey))
            // .with_state(state.clone())
            // .merge(get_routes(state.clone()))
            .merge(get_routes(state))
            // .layer(Extension(state))
            .layer(corslayer)
            // .layer(middleware::map_response(main_response_mapper))
            .layer(CookieManagerLayer::new())
            // .with_state(state)
            // .route(&format!("/surveys/test"), post(test_survey))
            // .route(&format!("/surveys/:id"), get(get_survey))
            // .route(&format!("/submit"), post(submit_survey))
            // .route("login", post(api_login))
            // .layer(DefaultBodyLimit::disable())
            // .layer(RequestBodyLimitLayer::new(1 * 1024 * 1024 /* 250mb */))
            .layer(TraceLayer::new_for_http());
        // let app = app.merge(routes_static());

        return app;
    }

    pub async fn new() -> ServerApplication {
        // const V1: &str = "v1";

        // dotenvy::from_filename("dev.env").ok();
        // initialize tracing
        // tracing_subscriber::fmt::init();
        tracing_subscriber::fmt::try_init();

        // tracing_subscriber::registry()
        //     .with(
        //         tracing_subscriber::EnvFilter::try_from_default_env()
        //             .unwrap_or_else(|_| "tower_http=debug".into()),
        //     )
        //     .with(tracing_subscriber::fmt::layer());

        let app = ServerApplication::get_router().await;

        // let app = configure_app().await;
        let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
        tracing::debug!("listening on {}", addr);

        let server = tokio::spawn(async move {
            info!("Server address: http://{addr}");
            axum::Server::bind(&addr)
                .serve(app.into_make_service())
                .await
                .unwrap();
            info!("after axum.");
        });
        // let oauth_client = oauth_client();

        return ServerApplication {
            base_url: addr,
            server: server,
            // oauth_client: None,
        };
    }
}

// pub fn routes_static() -> Router {
//     let router = Router::new().nest_service("/", get_service(ServeDir::new("./")));
//     return router;
// }

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateSurveyResponse {
    pub survey: Survey,
}

impl CreateSurveyResponse {
    fn from(survey: Survey) -> Self {
        CreateSurveyResponse { survey: survey }
    }
}

// async fn main_response_mapper(res: Response) -> Response {
//     println!("main_response_mapper - res={:?}", res);
//     res
// }

// #[axum::debug_handler]
// pub async fn get_survey(
//     State(state): State<ServerState>,
//     Path(survey_id): Path<String>,
// ) -> impl IntoResponse {
//     let pool = state.db.pool;

//     let count: i64 = sqlx::query_scalar("select count(id) from surveys")
//         .fetch_one(&pool)
//         .await
//         .map_err(internal_error)
//         .unwrap();
//     println!("Survey count: {count:#?}");

//     let res = sqlx::query_as::<_, SurveyModel>("select * from surveys as s where s.id = $1")
//         .bind(survey_id)
//         .fetch_one(&pool)
//         .await
//         .unwrap();

//     println!("Survey: {res:#?}");
//     let resp_survey = parse_markdown_v3(res.plaintext.clone());
//     let response = CreateSurveyResponse {
//         survey: resp_survey,
//         // metadata: res,
//     };

//     let template = FormTemplate {
//         survey_id: response.survey.id,
//     };

//     return (StatusCode::OK, template);
// }

// #[derive(Debug, Serialize, Clone, FromRow, Deserialize)]
// pub struct Survey {
//     pub id: String,
//     nanoid: String,
//     pub plaintext: String,
//     // questions: Question,
//     user_id: String,
//     created_at: String,
//     modified_at: String,
//     version: String,
// }

// #[derive(Debug, Deserialize, Serialize)]
// pub struct CreateSurveyRequest {
//     pub plaintext: String,
// }

// #[derive(Debug, Deserialize, Serialize)]
// pub struct CreateSurveyResponse {
//     pub survey: Survey,
// }

use std::{collections::HashMap, net::SocketAddr};

use axum::{
    extract::{self, Path, State},
    http::StatusCode,
    Json,
};
// use markdownparser::{markdown_to_form, parse_markdown_v3, Survey};
use serde::{Deserialize, Serialize};

// use ts_rs::TS;

// use crate::{internal_error, ServerState};

// #[derive(Debug, Serialize, Clone, FromRow, Deserialize)]
// pub struct Survey {
//     pub id: String,
//     // nanoid: String,
//     pub plaintext: String,
//     // user_id: String,
//     // created_at: String,
//     // modified_at: String,
//     // version: String,
// }

// impl Survey {
//     pub fn from(text: String) -> Survey {
//         return Survey {
//             id: nanoid_gen(10),
//             plaintext: text,
//         };
//     }
// }

// impl SurveyModel {
//     fn to_survey(survey: &SurveyModel) -> Survey {
//         let survey = survey.clone();
//         let questions = markdown_to_form(survey.plaintext.clone()).questions;
//         return Survey {
//             survey: todo!(),
//             metadata: todo!(),
//             UserInfo: todo!(),
//             // nanoid: markdownparser::nanoid_gen(),
//             // parse_version: survey.parse_version,
//         };
//     }
// }

#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct SurveyModel {
    pub id: String,
    pub plaintext: String,
    pub user_id: String,
    pub created_at: String,
    pub modified_at: String,
    // pub questions: Option<Vec<Question>>,
    pub version: String,
    pub parse_version: String,
}

struct Form {
    id: String,
    views: i32,
    starts: i32,
    submissions: i32,
    completions: i32,
    created_on: String,
    modified_on: String,
}

struct CreateForm {
    text: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Answers {
    id: String,
    used_id: String,
    survey_id: String,
    submitted_on: String,
    answers: HashMap<String, String>,
}

struct Answer {
    form_id: String,
    value: String,
}

// #[derive(Template)]
// #[template(path = "form.html")]
// struct FormTemplate {
//     survey_id: String,
// }

// pub async fn get_form(
//     State(_state): State<ServerState>,
//     Path(survey_id): Path<String>,
// ) -> FormTemplate {
//     FormTemplate {
//         survey_id: survey_id,
//     }
// }

// #[derive(Template)]
// #[template(path = "create_survey.html")]
// struct CreateSurveyTemplate {
//     // survey_value: String,
// }

// #[axum::debug_handler]
// pub async fn create_survey_form(State(_state): State<ServerState>) -> impl IntoResponse {
//     CreateSurveyTemplate {}
// }

mod test {
    use std::collections::HashMap;

    use axum::extract::{multipart, Multipart};
    use db::database::Database;

    use crate::ServerState;

    // #[tokio::test]
    // async fn test_create_answer() {
    //     let db = Database::new(true).await.unwrap();
    //     let state = ServerState { db: db };
    //     let mp = Multipart;
    //     let res = submit_survey(state, mp);
    // }
}
