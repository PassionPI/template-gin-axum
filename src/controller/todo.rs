use std::sync::Arc;

use axum::{extract::Query, response::IntoResponse, Extension, Json};

use super::{middleware::jwt::Claims, AppError};
use crate::{core::dep::Dep, model::common::OptionPagination};

pub async fn add(
    Extension(dep): Extension<Arc<Dep>>,
    Extension(claims): Extension<Claims>,
    Json(create): Json<crate::model::todo::TodoCreateItem>,
) -> Result<impl IntoResponse, AppError> {
    dep.pg.todo_insert_item(&claims.username, &create).await?;
    Ok("Ok")
}

pub async fn put(
    Extension(dep): Extension<Arc<Dep>>,
    Json(update): Json<crate::model::todo::TodoUpdateItem>,
) -> Result<impl IntoResponse, AppError> {
    dep.pg.todo_update_item(&update).await?;
    Ok("Ok")
}

pub async fn list(
    Extension(dep): Extension<Arc<Dep>>,
    Extension(claims): Extension<Claims>,
    Query(pagination): Query<OptionPagination>,
) -> Result<impl IntoResponse, AppError> {
    let todo = dep
        .pg
        .todo_find_by_username(&claims.username, &pagination.default())
        .await?;
    Ok(Json(todo))
}

pub async fn del(
    Extension(dep): Extension<Arc<Dep>>,
    Json(delete): Json<crate::model::todo::TodoDelItem>,
) -> Result<impl IntoResponse, AppError> {
    dep.pg.todo_delete_item(&delete).await?;
    Ok("Ok")
}
