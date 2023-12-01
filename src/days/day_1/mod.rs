use serde::{de, Deserialize, Deserializer};
use std::str::FromStr;

use axum::{extract::Path, http::StatusCode, routing::get, Router};

pub fn routes() -> Router {
    Router::new().route("/1/*vals", get(task))
}

struct Vals(Vec<usize>);

impl FromStr for Vals {
    type Err = StatusCode;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Vals(
            s.split("/")
                .map(|v| v.parse::<usize>().map_err(|_| StatusCode::BAD_REQUEST))
                .collect::<Result<Vec<usize>, _>>()?,
        ))
    }
}

impl<'de> Deserialize<'de> for Vals {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        FromStr::from_str(&s).map_err(de::Error::custom)
    }
}

async fn task(Path(vals): Path<Vals>) -> Result<String, StatusCode> {
    let mut iter = vals.0.into_iter();
    let mut output = iter.next().ok_or(StatusCode::BAD_REQUEST)?;

    while let Some(v) = iter.next() {
        output = output ^ v;
    }

    Ok(output.pow(3).to_string())
}
