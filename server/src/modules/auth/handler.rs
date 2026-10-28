use axum::Json;
use serde_json::{json, Value};


pub async fn signup()-> Json<Value>{
Json(
  json!({
    "message": "Register endpoint"
  })
)
}
