pub mod routes {

    use chrono::{DateTime, Utc};

    use axum::{
        extract::{Path, Query},
        http::{HeaderMap, Request},
        middleware::{self, Next},
        response::IntoResponse,
        response::{Html, Response},
        routing::{get, post},
        Router,
    };

    // use dioxus::prelude::{Component, Element, VirtualDom};
    use axum::{
        extract::{self, State},
        http::StatusCode,
        Json,
    };
    use dioxus::prelude::*;
    use markdownparser::{nanoid_gen, parse_markdown_v3};
    use serde::{Deserialize, Serialize};
    use serde_json::{json, Value};
    use ui::mainapp::App;

    use tracing::{debug, log::info};

    use crate::{
        auth::{self},
        db::database::CreateAnswersModel,
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

    use markdownparser::{
        parse_markdown_v3, ParsedSurvey, Question, QuestionOption, QuestionType, Questions, Survey,
    };

    #[derive(Deserialize, Serialize, Debug)]
    pub struct ListSurveyResponse {
        pub surveys: Vec<SurveyModel>,
    }

    #[derive(Deserialize, Debug, Serialize)]
    pub struct LoginPayload {
        pub email: String,
        pub password: String,
    }

    pub fn get_routes(state: ServerState) -> anyhow::Result<Router> {
        let routes = Router::new()
            .route("/surveys", post(create_survey).get(list_survey))
            .route("/surveys/:id", get(get_survey).post(submit_survey))
            .route("/responses", post(submit_response))
            .route("/responses/:id", get(survey_responses::list_response))
            .route("/auth/login", post(auth::authorize))
            .route("/auth/signup", post(auth::signup))
            .route("/ping", get(ping))
            .route("/ssr", get(ssr))
            .route("/editor", get(editor_route))
            .layer(middleware::map_response(main_response_mapper))
            // .layer(middleware::from_fn(propagate_header))
            .with_state(state);

        Ok(routes)
    }

    async fn propagate_header<B>(req: Request<B>, next: Next<B>) -> Response {
        next.run(req).await
    }

    pub async fn editor_route() -> Html<String> {
        fn Editor(cx: Scope) -> Element {
            const FORMINPUT_KEY: &str = "forminput";
            let editor_state = use_state(&cx, &EDITOR);
            let survey_state = use_state(&cx, &SURVEY);
            let send_req_timeout = use_state(&cx, &REQ_TIMEOUT);
            let app_state = use_state(&cx, &APP);

            // let question_state = use_atom_state(&cx, APP);
            // let send_request_timeout = use_atom_state(&cx, REQ_TIMEOUT);
            // let some_timeout = use_state(&cx, || TimeoutFuture::new(2000));
            let create_survey = move |content: String, client: Client| {
                cx.spawn({
                    to_owned![editor_state, app_state, send_req_timeout];
                    if app_state.read().user.is_none() {
                        info!("Not logged in yet.");
                        return;
                    }
                    // timeout.get()
                    async move {
                        let something = send_req_timeout.get();
                        let token = app_state.read().user.clone().unwrap().token;
                        // timeout.await;
                        // TimeoutFuture::new(2000).await;
                        // let t = Timeou
                        info!("Attempting to save questions...");
                        // info!("Questions save: {:?}", question_state);
                        match app_state
                            .read()
                            .client
                            .post("http://localhost:3000/surveys")
                            .json(&CreateSurvey {
                                plaintext: editor_state.get().clone(),
                            })
                            // .bearer_auth(token)
                            .send()
                            .await
                        {
                            Ok(x) => {
                                info!("success: {x:?}");
                            }
                            Err(x) => info!("error: {x:?}"),
                        }

                        // timeout = Timeout::new(1000, callback)
                    }
                })
            };

            let post_questions = move |content: Vec<String>| {
                info!("post_questions: {:?}", content);
                cx.spawn({
                    to_owned![app_state];

                    if app_state.read().user.is_none() {
                        info!("user token is not set");
                        return;
                    }

                    let mut token = app_state.read().user.clone().unwrap().token;
                    token = token.trim_matches('"').to_string();
                    async move {
                        info!("Attempting to save questions...");
                        info!("Publishing content, app_state: {:?}", app_state.read());
                        // info!("Questions save: {:?}", question_state);
                        match app_state
                            .read()
                            .client
                            .post("http://localhost:3000/surveys")
                            .json(&CreateSurvey {
                                plaintext: content.get(0).unwrap().to_owned(),
                            })
                            // .bearer_auth(token.clone())
                            .header("x-auth-token", token)
                            .send()
                            .await
                        {
                            Ok(x) => {
                                info!("success: {x:?}");
                                info!("should show toast now");
                            }
                            Err(x) => info!("error: {x:?}"),
                        };
                    }
                })
            };

            let editor_survey = move |content: Vec<String>| {
                info!("editor survey content: {:?}", content);
                match ParsedSurvey::from(content.get(0).unwrap().to_owned()) {
                    Ok(x) => {
                        info!("Parsed: {x:#?}");
                        app_state.write().survey = Survey::from(x.clone());
                        survey_state.modify(|curr| Survey::from(x));
                    }
                    Err(_) => {}
                };
            };

            cx.render(rsx! {
                div { class: "w-full h-full",
                    form {
                        class: "border border-red-600 flex flex-col",
                        prevent_default: "onsubmit",
                        // action: "localhost:3000/survey",
                        onsubmit: move |evt| {
                            info!("Pushed publish :)");
                            let formvalue = evt.values.get(FORMINPUT_KEY).clone().unwrap().clone();
                            post_questions(formvalue);
                            evt.stop_propagation();
                        },
                        oninput: move |e| {
                            let formvalue = e.values.get(FORMINPUT_KEY).clone().unwrap().clone();
                            editor_survey(formvalue.clone());
                            info!("onchange results - editor_state formvalue: {:?}", formvalue);
                            editor_state.modify(|curr| { formvalue.get(0).unwrap().to_owned() });
                        },
                        textarea {
                            class: " bg-transparent resize w-full focus:outline-none border border-emerald-800 focus:border-blue-300",
                            required: "",
                            rows: "8",
                            placeholder: "Write your survey here",
                            name: FORMINPUT_KEY
                        }
                        button { class: "hover:bg-slate-600 transition bg-slate-500",
                            // r#type: "submit",
                            "Publish"
                        }
                    }
                }
            })
        }

        let app: Component = |cx| Editor(cx);

        let mut vdom = VirtualDom::new(app);
        let _ = vdom.rebuild();

        let text = dioxus_ssr::render(&vdom);

        // render the VirtualDom to HTML
        Html(text)
    }

    #[tracing::instrument]
    #[axum::debug_handler]
    pub async fn ssr() -> anyhow::Result<Html<String>, ServerError> {
        let app: Component = |cx| App(cx);

        let mut vdom = VirtualDom::new(app);
        let _ = vdom.rebuild();

        let text = dioxus_ssr::render(&vdom);

        // render the VirtualDom to HTML
        Ok(Html(text))
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
        ctx: Option<Ctext>,
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
        _ctx: Option<Ctext>,
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
}
