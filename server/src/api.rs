#[tracing::instrument]
#[axum::debug_handler]
pub async fn list_survey(
    state: State<ServerState>,
    Extension(ctx): Extension<SessionContext>,
    // headers: HeaderMap,
) -> anyhow::Result<Json<Value>, ServerError> {
    info!("->> list_survey");

    println!("Getting surveys for user={}", ctx.user_id);
    let pool = &state.db.pool;

    let res = sqlx::query_as::<_, SurveyModel>(
        "select * from mdp.surveys where mdp.surveys.user_id = $1 and mdp.surveys.workspace_id = $2",
    )
    .bind(ctx.user_id.clone())
    .bind(ctx.session.workspace_id.clone())
    .fetch_all(pool)
    .await
    .map_err(|err| ServerError::Database(err.to_string()))
    .unwrap();

    let resp = ListSurveyResponse { surveys: res };

    Ok(Json(json!(resp)))
}
