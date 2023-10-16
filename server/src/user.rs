// struct UserLogin {
//     id: String,
//     plaintext: String,
//     user_id: String,
//     created_at: String,
//     modified_at: String,
//     version: String,
// }

// #[axum::debug_handler]
// pub async fn login(State(state): State<ServerState>) -> impl IntoResponse {
//     prinln!("Logging in");

//     let pool = state.db.pool;

//     let res: Vec<SurveyModel> =
//         sqlx::query_as::<_, UserLogin>("select * from users where users.username = $1")
//             .bind()
//             .fetch_one(&pool)
//             .await
//             .unwrap();

//     // let count: i64 = sqlx::query_scalar("select count(id) from surveys")
//     //     .fetch_one(&pool)
//     //     .await
//     //     .map_err(internal_error)
//     //     .unwrap();
//     // println!("Survey count: {count:#?}");

//     let surveys = res.iter().map(|x| SurveyModel::to_survey(x)).collect();

//     // json!({ "surveys": res });

//     println!("Survey: {res:#?}");
//     let listresp = ListSurveyResponse { surveys: surveys };

//     // (StatusCode::OK, Json(json!({ "surveys": res })))
//     (StatusCode::OK, Json(listresp))
// }

// // Session is optional
// async fn index(user: Option<User>) -> impl IntoResponse {
//     match user {
//         Some(u) => format!(
//             "Hey {}! You're logged in!\nYou may now access `/protected`.\nLog out with `/logout`.",
//             u.username
//         ),
//         None => "You're not logged in.\nVisit `/auth/discord` to do so.".to_string(),
//     }
// }

// async fn discord_auth(State(client): State<BasicClient>) -> impl IntoResponse {
//     let (auth_url, _csrf_token) = client
//         .authorize_url(CsrfToken::new_random)
//         .add_scope(Scope::new("identify".to_string()))
//         .url();

//     // Redirect to Discord's oauth service
//     Redirect::to(auth_url.as_ref())
// }

// // Valid user session required. If there is none, redirect to the auth page
// async fn protected(user: User) -> impl IntoResponse {
//     format!(
//         "Welcome to the protected area :)\nHere's your info:\n{:?}",
//         user
//     )
// }

// async fn logout(
//     State(store): State<MemoryStore>,
//     TypedHeader(cookies): TypedHeader<headers::Cookie>,
// ) -> impl IntoResponse {
//     let cookie = cookies.get(COOKIE_NAME).unwrap();
//     let session = match store.load_session(cookie.to_string()).await.unwrap() {
//         Some(s) => s,
//         // No session active, just redirect
//         None => return Redirect::to("/"),
//     };

//     store.destroy_session(session).await.unwrap();

//     Redirect::to("/")
// }

// #[derive(Debug, Deserialize)]
// #[allow(dead_code)]
// struct AuthRequest {
//     code: String,
//     state: String,
// }

// async fn login_authorized(
//     Query(query): Query<AuthRequest>,
//     State(store): State<MemoryStore>,
//     State(oauth_client): State<BasicClient>,
// ) -> impl IntoResponse {
//     // Get an auth token
//     let token = oauth_client
//         .exchange_code(AuthorizationCode::new(query.code.clone()))
//         .request_async(async_http_client)
//         .await
//         .unwrap();

//     // Fetch user data from discord
//     let client = reqwest::Client::new();
//     let user_data: User = client
//         // https://discord.com/developers/docs/resources/user#get-current-user
//         .get("https://discordapp.com/api/users/@me")
//         .bearer_auth(token.access_token().secret())
//         .send()
//         .await
//         .unwrap()
//         .json::<User>()
//         .await
//         .unwrap();

//     // Create a new session filled with user data
//     let mut session = Session::new();
//     session.insert("user", &user_data).unwrap();

//     // Store session and get corresponding cookie
//     let cookie = store.store_session(session).await.unwrap().unwrap();

//     // Build the cookie
//     let cookie = format!("{}={}; SameSite=Lax; Path=/", COOKIE_NAME, cookie);

//     // Set cookie
//     let mut headers = HeaderMap::new();
//     headers.insert(SET_COOKIE, cookie.parse().unwrap());

//     (headers, Redirect::to("/"))
// }

// struct AuthRedirect;

// impl IntoResponse for AuthRedirect {
//     fn into_response(self) -> Response {
//         Redirect::temporary("/auth/discord").into_response()
//     }
// }

// #[async_trait]
// impl<S> FromRequestParts<S> for User
// where
//     MemoryStore: FromRef<S>,
//     S: Send + Sync,
// {
//     // If anything goes wrong or no session is found, redirect to the auth page
//     type Rejection = AuthRedirect;

//     async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
//         let store = MemoryStore::from_ref(state);

//         let cookies = parts
//             .extract::<TypedHeader<headers::Cookie>>()
//             .await
//             .map_err(|e| match *e.name() {
//                 header::COOKIE => match e.reason() {
//                     TypedHeaderRejectionReason::Missing => AuthRedirect,
//                     _ => panic!("unexpected error getting Cookie header(s): {}", e),
//                 },
//                 _ => panic!("unexpected error getting cookies: {}", e),
//             })?;
//         let session_cookie = cookies.get(COOKIE_NAME).ok_or(AuthRedirect)?;

//         let session = store
//             .load_session(session_cookie.to_string())
//             .await
//             .unwrap()
//             .ok_or(AuthRedirect)?;

//         let user = session.get::<User>("user").ok_or(AuthRedirect)?;

//         Ok(user)
//     }
// }
