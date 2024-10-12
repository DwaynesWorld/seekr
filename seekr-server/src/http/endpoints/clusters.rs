use std::collections::HashMap;

use axum::extract::{Path, Query, State};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::http::{AppContext, Result};
use crate::protos::api::clusters::{
    Cluster as ClusterPb, ClusterKind, CreateClusterRequest, CreateClusterResponse,
    ListClustersRequest, ListClustersResponse,
};

pub(crate) fn router() -> Router<AppContext> {
    Router::new()
        .route("/api/v1/clusters", post(create_cluster).get(list_clusters))
        .route("/api/v1/clusters/:id", get(get_cluster))
}

async fn create_cluster(
    context: State<AppContext>,
    Json(r): Json<CreateClusterRequest>,
) -> Result<Json<CreateClusterResponse>> {
    info!("create cluster");
    info!("{:?}", r);

    if let Err(err) = r.validate() {
        return Err(err);
    }

    let cluster: Cluster = r.into();
    let value = bincode::serialize(&cluster).unwrap();

    context.db.insert(cluster.get_key(), value).unwrap();

    Ok(Json(CreateClusterResponse {
        cluster: Some(cluster.into()),
    }))
}

async fn list_clusters(
    context: State<AppContext>,
    Query(p): Query<ListClustersRequest>,
) -> Result<Json<ListClustersResponse>> {
    info!("reading from database");

    let clusters: Vec<ClusterPb> = context
        .db
        .iter()
        .map(|c| {
            bincode::deserialize::<Cluster>(&c.unwrap().1)
                .unwrap()
                .into()
        })
        .collect();

    let count = clusters.len() as i64;

    Ok(Json(ListClustersResponse { clusters, count }))
}

#[derive(Debug, Deserialize, Serialize)]
struct GetParams {
    id: String,
}

async fn get_cluster(
    context: State<AppContext>,
    Path(cluster_id): Path<String>,
) -> Result<Json<ClusterPb>> {
    info!("reading from database {}", cluster_id);

    let encoded = context.db.get(&get_key(cluster_id)).unwrap().unwrap();
    let cluster = bincode::deserialize::<Cluster>(&encoded).unwrap();
    Ok(Json(cluster.into()))
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Cluster {
    pub id: Uuid,
    pub kind: ClusterKind,
    pub name: String,
    pub config: HashMap<String, String>,
    pub created_at: i64,
    pub modified_at: i64,
}

impl Cluster {
    pub fn get_key(&self) -> Vec<u8> {
        get_key(self.id.to_string())
    }
}

impl Into<ClusterPb> for Cluster {
    fn into(self) -> ClusterPb {
        ClusterPb {
            id: self.id.to_string(),
            name: self.name,
            kind: self.kind.into(),
            config: self.config.clone(),
            created_at: self.created_at,
            modified_at: self.modified_at,
        }
    }
}

impl CreateClusterRequest {
    #[rustfmt::skip]
    fn validate(&self) -> Result<()> {
        Ok(())
    }
}

impl Into<Cluster> for CreateClusterRequest {
    fn into(self) -> Cluster {
        let now = chrono::offset::Utc::now().timestamp_millis();

        Cluster {
            id: Uuid::now_v7(),
            kind: self.kind(),
            name: self.name.clone(),
            config: self.config.clone(),
            created_at: now,
            modified_at: now,
        }
    }
}

fn get_key(id: String) -> Vec<u8> {
    bincode::serialize(&format!("clusters/{}", id)).unwrap()
}
