use std::sync::Arc;

use axum::{response::IntoResponse, Extension};

use crate::core::Core;

pub async fn get_public_pem(Extension(core): Extension<Arc<Core>>) -> impl IntoResponse {
    core.rsa.get_public_pem().to_owned()
}
