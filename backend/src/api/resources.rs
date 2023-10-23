use actix_files as fs;
use actix_web::{
    get,
    http::header::{ContentDisposition, DispositionType},
    HttpRequest,
};

use crate::error::BackendError;

#[get("/resources/{filename:.*}")]
pub async fn index(req: HttpRequest) -> Result<fs::NamedFile, BackendError> {
    let path: std::path::PathBuf = req
        .match_info()
        .query("filename")
        .parse()
        .map_err(|_| BackendError::ContentNotFound("Content not found!".to_owned()))?;
    let file =
        fs::NamedFile::open(path).map_err(|e| BackendError::ContentNotFound(e.to_string()))?;
    Ok(file
        .use_last_modified(true)
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Attachment,
            parameters: vec![],
        }))
}
