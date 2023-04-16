pub mod routes {
    use axum::{
        extract::{self, State},
        response::IntoResponse,
    };
    use db::models::{SurveyModelBuilder, CreateSurveyRequest};
    use markdownparser::parse_markdown_v3;
    use tracing::info;

    use crate::ServerState;

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
        let response = CreateSurveyResponse::from(new_survey);
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
        State(state): State<ServerState>,
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
        let pool = state.db.pool;

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
                .fetch_all(&pool)
                .await
                .unwrap();

        let resp = ListSurveyResponse { surveys: res };

        (StatusCode::OK, Json(resp))
    }

    #[derive(Deserialize, Serialize, Debug)]
    pub struct ListSurveyResponse {
        pub surveys: Vec<SurveyModel>,
    }

    fn routes_static() -> Router {
        let router = Router::new().nest_service("/", get_service(ServeDir::new("./")));
        return router;
    }
}
