use rocket::response::status::NotFound;
use rocket::fs::NamedFile;

use std::path::PathBuf;

#[get("/<path..>")]
pub async fn find(path: PathBuf) -> Result<NamedFile, NotFound<String>> {
    let path = PathBuf::from("site").join(path);
    NamedFile::open(path).await.map_err(|e| NotFound(e.to_string()))
}

