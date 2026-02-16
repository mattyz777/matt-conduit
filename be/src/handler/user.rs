use axum::{extract::{Path, State}, Json};
use crate::dto::user::{CreateUserRequest, LoginRequest, UpdateUserRequest, UserResponse};
use crate::dto::common::ApiResponse;
use crate::entity::user;
use crate::service::user::UserService;
use crate::error::AppError;
use crate::state::AppState;

pub type ApiResult<T> = Result<Json<ApiResponse<T>>, AppError>;

/// 将 entity Model 转换为 UserResponse
fn to_response(user: &user::Model) -> UserResponse {
    UserResponse {
        id: user.id,
        username: user.username.clone(),
        age: user.age,
        gender: user.gender.clone(),
        email: user.email.clone(),
        created_at: user.created_at.to_string(),
        updated_at: user.updated_at.to_string(),
    }
}

/// 创建用户
pub async fn create_user(
    State(state): State<AppState>,
    Json(req): Json<CreateUserRequest>,
) -> ApiResult<UserResponse> {
    crate::log_info!("创建用户请求: username={}, age={:?}", req.username, req.age);

    let user = UserService::create(
        &state,
        req.username,
        req.password,
        req.age,
        req.gender,
        req.email,
    )
    .await?;

    crate::log_info!("用户创建成功: id={}, username={}", user.id, user.username);
    Ok(Json(ApiResponse::ok(to_response(&user))))
}

/// 根据 ID 查询用户
pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> ApiResult<Option<UserResponse>> {
    crate::log_info!("查询用户请求: id={}", id);

    let user = UserService::find_by_id(&state, id).await?;
    let response = user.as_ref().map(|u| {
        crate::log_info!("用户查询成功: id={}, username={}", u.id, u.username);
        to_response(u)
    });

    Ok(Json(ApiResponse::ok(response)))
}

/// 更新用户
pub async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(req): Json<UpdateUserRequest>,
) -> ApiResult<UserResponse> {
    crate::log_info!(
        "更新用户请求: id={}, username={:?}, age={:?}, gender={:?}, email={:?}",
        id,
        req.username,
        req.age,
        req.gender,
        req.email
    );

    let user = UserService::update(
        &state,
        id,
        req.username,
        req.password,
        req.age,
        req.gender,
        req.email,
    )
    .await?;

    crate::log_info!("用户更新成功: id={}, username={}", user.id, user.username);
    Ok(Json(ApiResponse::ok(to_response(&user))))
}

/// 删除用户
pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> ApiResult<()> {
    crate::log_info!("删除用户请求: id={}", id);

    UserService::delete(&state, id).await?;

    crate::log_info!("删除用户成功: id={}", id);
    Ok(Json(ApiResponse::<()>::ok_without_data("删除成功")))
}

/// 用户登录
pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> ApiResult<UserResponse> {
    crate::log_info!("用户登录请求: username={}", req.username);

    let user = UserService::verify_login(&state, &req.username, &req.password).await?;

    crate::log_info!("用户登录成功: id={}, username={}", user.id, user.username);
    Ok(Json(ApiResponse::ok(to_response(&user))))
}
