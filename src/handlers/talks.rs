use super::{Error::*, Result};
use crate::{app::State, post::Post, templates};
use axum::{
    extract::{Extension, Path},
    response::Html,
};
use lazy_static::lazy_static;
use prometheus::{opts, register_int_counter_vec, IntCounterVec};
use std::sync::Arc;
use tracing::instrument;

lazy_static! {
    static ref HIT_COUNTER: IntCounterVec = register_int_counter_vec!(
        opts!("talks_hits", "Number of hits to talks images"),
        &["name"]
    )
    .unwrap();
}

#[instrument(skip(state))]
pub async fn index(Extension(state): Extension<Arc<State>>) -> Result {
    let state = state.clone();
    let mut result: Vec<u8> = vec![];
    templates::talkindex_html(&mut result, state.talks.clone())?;
    Ok(Html(result))
}

#[instrument(skip(state))]
pub async fn post_view(
    Path(name): Path<String>,
    Extension(state): Extension<Arc<State>>,
) -> Result {
    let mut want: Option<Post> = None;

    for post in &state.talks {
        if post.link == format!("talks/{}", name) {
            want = Some(post.clone());
        }
    }

    match want {
        None => Err(PostNotFound(name).into()),
        Some(post) => {
            HIT_COUNTER
                .with_label_values(&[name.clone().as_str()])
                .inc();
            let body = templates::Html(post.body_html.clone());
            let mut result: Vec<u8> = vec![];
            templates::talkpost_html(&mut result, post, body)?;
            Ok(Html(result))
        }
    }
}
