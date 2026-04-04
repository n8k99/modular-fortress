use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct Pipeline {
    pub name: String,
    pub status: String,
    pub active_count: i32,
}

pub async fn list_pipelines(
    State(_state): State<AppState>,
) -> Result<Json<Vec<Pipeline>>, StatusCode> {
    // Mock data for now - will be replaced with real pipeline queries
    let pipelines = vec![
        Pipeline {
            name: "Engineering".to_string(),
            status: "flowing".to_string(),
            active_count: 3,
        },
        Pipeline {
            name: "Complete Success".to_string(),
            status: "pending".to_string(),
            active_count: 0,
        },
        Pipeline {
            name: "Modular Fortress".to_string(),
            status: "idle".to_string(),
            active_count: 0,
        },
    ];

    Ok(Json(pipelines))
}
