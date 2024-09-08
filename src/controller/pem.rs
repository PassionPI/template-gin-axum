use std::sync::Arc;

use axum::{response::IntoResponse, Extension};

use crate::core::dep::Dep;

pub async fn get_public_pem(Extension(dep): Extension<Arc<Dep>>) -> impl IntoResponse {
    dep.rsa.get_public_pem().to_owned()
}
