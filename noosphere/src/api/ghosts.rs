use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Ghost {
    pub id: String,
    pub name: String,
    pub role: Option<String>,
    pub tier: Option<String>,
    pub domain: Option<String>,
    pub trust: Option<f64>,
    pub energy: Option<f64>,
    pub status: Option<String>,
    pub location: Option<String>,
}

pub async fn list_ghosts(
    State(state): State<AppState>,
) -> Result<Json<Vec<Ghost>>, StatusCode> {
    let ghosts = sqlx::query_as::<_, Ghost>(
        r#"
        SELECT
            id,
            name,
            role,
            tier,
            area_id as domain,
            trust_level as trust,
            energy_level as energy,
            state as status,
            current_location as location
        FROM agents
        WHERE active = true
        ORDER BY tier DESC, name ASC
        "#,
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        tracing::error!("Database error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(ghosts))
}

pub async fn get_ghost(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> Result<Json<Ghost>, StatusCode> {
    let ghost = sqlx::query_as::<_, Ghost>(
        r#"
        SELECT
            id,
            name,
            role,
            tier,
            area_id as domain,
            trust_level as trust,
            energy_level as energy,
            state as status,
            current_location as location
        FROM agents
        WHERE id = $1
        "#,
    )
    .bind(&id)
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::NOT_FOUND)?;

    Ok(Json(ghost))
}
