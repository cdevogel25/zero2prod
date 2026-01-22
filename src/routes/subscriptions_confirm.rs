use actix_web::{HttpResponse, http::StatusCode, web, ResponseError};
use anyhow::Context;
use sqlx::PgPool;
use uuid::Uuid;

use crate::routes::error_chain_fmt;

// --- SECTION: structs and implementations ---

#[derive(serde::Deserialize)]
pub struct Parameters {
    subscription_token: String,
}

// --- SECTION: errors ---

#[derive(thiserror::Error)]
pub enum ConfirmError {
    // ok honestly i dont even really know what we're testing for here
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
    #[error("There is no subscriber associated with the provided token.")]
    UnknownToken,
}

impl ResponseError for ConfirmError {
    fn status_code(&self) -> StatusCode {
        match self {
            ConfirmError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ConfirmError::UnknownToken => StatusCode::UNAUTHORIZED,
        }
    }
}

impl std::fmt::Debug for ConfirmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

// --- SECTION: web actions ---

#[tracing::instrument(
    name = "Confirm a pending subscriber",
    skip(parameters, pool))
]
pub async fn confirm(parameters: web::Query<Parameters>, pool: web::Data<PgPool>) -> Result<HttpResponse, ConfirmError> {
    let subscriber_id = get_subscriber_id_from_token(&pool, &parameters.subscription_token)
        .await
        .context("Failed to retrieve the subscriber id associated with the provided token")?
        .ok_or(ConfirmError::UnknownToken)?;
    confirm_subscriber(&pool, subscriber_id)
        .await
        .context("Failed to update subscriber status to `confirmed`.")?;
    Ok(HttpResponse::Ok().finish())
}

// --- SECTION: database actions ---

#[tracing::instrument(
    name = "Mark subscriber as confirmed",
    skip(pool, subscriber_id))
]
pub async fn confirm_subscriber(pool: &PgPool, subscriber_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"UPDATE subscriptions SET status = 'confirmed' WHERE id = $1"#,
        subscriber_id,
    )
    .execute(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(())
}

#[tracing::instrument(
    name = "Get subscriber_id from token",
    skip(pool, subscription_token))
]
pub async fn get_subscriber_id_from_token(
    pool: &PgPool,
    subscription_token: &str,
) -> Result<Option<Uuid>, sqlx::Error> {
    let result = sqlx::query!(
        "SELECT subscriber_id FROM subscription_tokens \
        WHERE subscription_token = $1",
        subscription_token,
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;
    Ok(result.map(|r| r.subscriber_id))
}
