use axum::Json;
use jwt_simple::prelude::Deserialize;
use crate::error::AppError;
use crate::repository::Repository;

#[derive(Deserialize)]
pub struct TurnUserIntoAdminRequest {
    user_id: i64,
}
pub async fn turn_user_into_admin(
    repository: Repository,
    Json(request): Json<TurnUserIntoAdminRequest>
) -> Result<(), AppError> {
    repository.turn_user_into_admin(request.user_id).await?;
    Ok(())
}