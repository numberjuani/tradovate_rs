
#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    Json(serde_json::Error),
    Io(std::io::Error),
    Url(url::ParseError),
    Other(String),
}