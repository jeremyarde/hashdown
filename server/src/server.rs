use axum::{
    extract::{DefaultBodyLimit, Multipart, Query},
    http::{self, HeaderMap, HeaderName, HeaderValue, Method},
    middleware,
    response::{IntoResponse, Response},
    Extension, Router,
};
use chrono::{DateTime, Utc};
use db::database::Database;
use markdownparser::{nanoid_gen, parse_markdown_v3, MetadataBuilder, Survey};

use sqlx::FromRow;
// use ormlite::Model;
use tokio::{fs, task::JoinHandle};

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
    config::EnvConfig,
    db::{self, database::ConnectionDetails},
    mail::mail::Mailer,
    routes::routes::{
        create_survey,
        get_routes,
        get_survey,
        list_survey,
        submit_survey,
        // test_survey,
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
        let in_memory = if dotenvy::var("DATABASE_IN_MEMORY")
            .expect("Could not find `DATABASE_IN_MEMORY` env variable")
            .trim()
            == "true"
        {
            true
        } else {
            false
        };

        let uri = dotenvy::var("DATABASE_URL").expect("Could not get connection string from env");
        let db_url = ConnectionDetails(uri);
        let db = Database::new(in_memory, db_url)
            .await
            .expect("Error connecting to database");
        let state = ServerState {
            db: db,
            mail: Mailer::new(),
            config: EnvConfig::new(),
        };

        let mut origins = vec![];
        info!("Starting app in stage={:?}", state.config.stage);
        if state.config.is_dev() {
            origins.append(&mut vec![
                "http://localhost:3000".parse().unwrap(),
                "http://localhost:3001".parse().unwrap(),
                "http://localhost:8080".parse().unwrap(),
                // "http://api.example.com".parse().unwrap(),
            ]);
        }
        let corslayer = CorsLayer::new()
            .allow_methods([Method::POST, Method::GET])
            .allow_headers([
                http::header::CONTENT_TYPE,
                http::header::ACCEPT,
                // http::header::AUTHORIZATION,
                HeaderName::from_static("x-auth-token"),
            ])
            // .allow_headers(Any)
            .allow_credentials(true)
            .allow_origin(origins);

        // build our application with a route
        let app = Router::new()
            .merge(get_routes(state).unwrap())
            .layer(corslayer)
            .layer(TraceLayer::new_for_http());

        return app;
    }

    pub async fn new() -> ServerApplication {
        info!("Spinning up the server.");

        // const V1: &str = "v1";
        tracing_subscriber::fmt::try_init();

        // tracing_subscriber::registry()
        //     .with(
        //         tracing_subscriber::EnvFilter::try_from_default_env()
        //             .unwrap_or_else(|_| "tower_http=debug".into()),
        //     )
        //     .with(tracing_subscriber::fmt::layer());

        let app = ServerApplication::get_router().await;

        // let app = configure_app().await;
        let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
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

        info!("Server is running...");
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

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct SurveyModel {
    pub id: i32,
    pub survey_id: String,
    pub user_id: String,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
    pub plaintext: String,
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

mod test {
    use std::collections::HashMap;

    use crate::{db, ServerState};
    use axum::extract::{multipart, Multipart};
    use db::database::Database;

    // #[tokio::test]
    // async fn test_create_answer() {
    //     let db = Database::new(true).await.unwrap();
    //     let state = ServerState { db: db };
    //     let mp = Multipart;
    //     let res = submit_survey(state, mp);
    // }
}
