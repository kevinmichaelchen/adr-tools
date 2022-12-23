use std::borrow::Cow;
use serde::{Deserialize, Serialize};
use chrono::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct ADR {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    pub title: Cow<'static, str>,
    pub time: DateTime<Utc>,
    pub name: Author,
    pub draft: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Author {
    pub first: Cow<'static, str>,
    pub last: Cow<'static, str>,
}