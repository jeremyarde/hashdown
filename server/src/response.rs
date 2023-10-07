struct Response {
    survey_id: String,
    responses: HashMap<String, Vec<String>>,
}

#[tracing::instrument]
#[axum::debug_handler]
pub async fn submit_response(
    State(state): State<ServerState>,
    Path(survey_id): Path<String>,
    ctx: Option<Ctext>,
    Json(payload): extract::Json<Value>, // for urlencoded
) -> Result<Json<Value>, ServerError> {
    info!("->> submit_survey");
    debug!("    ->> survey: {:#?}", payload);

    // json version
    let survey = match state
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

    let answer_result = state
        .db
        .create_answer(create_answer_model)
        .await
        .expect("Should create answer in database");

    info!("completed survey submit");

    return Ok(Json(json!({ "survey_id": survey_id })));
}

#[tracing::instrument]
#[axum::debug_handler]
pub async fn list_response(
    State(state): State<ServerState>,
    Path(survey_id): Path<String>,
    ctx: Option<Ctext>,
    Json(payload): extract::Json<Value>, // for urlencoded
) -> Result<Json<Value>, ServerError> {
    info!("->> submit_survey");
    debug!("    ->> survey: {:#?}", payload);

    // json version
    let survey = match state
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

    let answer_result = state
        .db
        .create_answer(create_answer_model)
        .await
        .expect("Should create answer in database");

    info!("completed survey submit");

    return Ok(Json(json!({ "survey_id": survey_id })));
}
