#[axum::debug_handler]
pub async fn create_survey(
    State(state): State<ServerState>,
    extract::Json(payload): extract::Json<CreateSurveyRequest>,
) -> impl IntoResponse {
    let survey = parse_markdown_v3(payload.plaintext.clone());
    // let survey = Survey::from(payload.plaintext.clone());
    let response_survey = survey.clone();

    let res = sqlx::query!(
        "insert into surveys (id, plaintext, user_id, created_at, modified_at, version, parse_version) 
        values 
        ($1, $2, $3, $4, $5, $6, $7)
        // returning * 
        ",
    )
    .bind(response_survey.id.clone())
    .bind(payload.plaintext)
    .bind(survey.user_id)
    .bind(survey.created_at)
    .bind(survey.modified_at)
    // .bind(json!({"questions": survey.questions}))
    .bind(survey.version)
    .bind(survey.parse_version).fetch_one(&state.db.pool)
    .await
    .unwrap();

    let response = CreateSurveyResponse {
        survey: Survey::from(response_survey),
        metadata: res,
    };

    (StatusCode::CREATED, Json(response))
}

#[axum::debug_handler]
pub async fn list_survey(State(state): State<ServerState>) -> impl IntoResponse {
    let pool = state.db.pool;

    let count: i64 = sqlx::query_scalar("select count(id) from surveys")
        .fetch_one(&pool)
        .await
        .map_err(internal_error)
        .unwrap();
    println!("Survey count: {count:#?}");

    let res: Vec<SurveyModel> = sqlx::query_as::<_, SurveyModel>("select * from surveys")
        .fetch_all(&pool)
        .await
        .unwrap();

    let surveys = res.iter().map(|x| SurveyModel::to_survey(x)).collect();

    // json!({ "surveys": res });

    println!("Survey: {res:#?}");
    let listresp = ListSurveyResponse { surveys: surveys };

    // (StatusCode::OK, Json(json!({ "surveys": res })))
    (StatusCode::OK, Json(listresp))
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ListSurveyResponse {
    pub surveys: Vec<Survey>,
}

#[axum::debug_handler]
pub async fn get_survey(
    State(state): State<ServerState>,
    Path(survey_id): Path<String>,
) -> impl IntoResponse {
    let pool = state.db.pool;

    let count: i64 = sqlx::query_scalar("select count(id) from surveys")
        .fetch_one(&pool)
        .await
        .map_err(internal_error)
        .unwrap();
    println!("Survey count: {count:#?}");

    let res = sqlx::query_as::<_, SurveyModel>("select * from surveys as s where s.id = $1")
        .bind(survey_id)
        .fetch_one(&pool)
        .await
        .unwrap();

    println!("Survey: {res:#?}");
    let resp_survey = parse_markdown_v3(res.plaintext.clone());
    let response = CreateSurveyResponse {
        survey: resp_survey,
        metadata: res,
    };

    let template = FormTemplate {
        survey_id: response.survey.id,
    };

    return (StatusCode::OK, template);
}

fn setup_routes() {
    let router: Router = Router::new()
        // .route("/surveys/new", get(create_survey_form))
        .route(&format!("/surveys"), post(create_survey).get(list_survey));
    return router;
}
