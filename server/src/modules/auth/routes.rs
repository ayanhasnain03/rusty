use axum::{
  Router,
  routing::post
};

use crate::modules::auth::handler::{
  signup
};
pub fn routes ()-> Router{
  Router::new()
  .route("/signup", post(signup))
}
