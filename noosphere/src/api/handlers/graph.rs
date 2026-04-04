//! Graph endpoints - document relationship visualization

use axum::{
    extract::{Query, State},
    Json,
};
use dpn_core::{DbPool, GraphOptions};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::ApiError;

#[derive(Deserialize, Default)]
pub struct GraphQueryParams {
    /// Only include documents matching this path prefix
    pub path_prefix: Option<String>,
    /// Maximum number of nodes to include
    pub max_nodes: Option<usize>,
    /// Minimum weight (degree) to include a node
    pub min_weight: Option<usize>,
    /// Include orphan nodes (no connections)
    #[serde(default)]
    pub include_orphans: bool,
}

/// GET /api/graph?path_prefix=Areas&max_nodes=100&min_weight=1&include_orphans=false
/// Returns full graph data (nodes + edges) for visualization
pub async fn get_graph(
    State(pool): State<DbPool>,
    Query(params): Query<GraphQueryParams>,
) -> Result<Json<Value>, ApiError> {
    let options = GraphOptions {
        path_prefix: params.path_prefix,
        max_nodes: params.max_nodes,
        min_weight: params.min_weight,
        include_orphans: params.include_orphans,
        categories: vec![],
        tags: vec![],
    };

    let graph_data = dpn_core::build_graph(&pool, options)
        .await
        .map_err(|e| ApiError::Database(e.to_string()))?;

    Ok(Json(serde_json::json!(graph_data)))
}
