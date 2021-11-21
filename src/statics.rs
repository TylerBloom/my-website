use rocket::response::status::NotFound;
use rocket::fs::NamedFile;

use std::path::PathBuf;

#[get("/<path..>")]
pub async fn find(path: PathBuf) -> Result<NamedFile, NotFound<String>> {
    println!("Inside statics::find");
    println!("{}", path.as_path().to_str().unwrap() );
    let path = PathBuf::from("site/static").join(path);
    NamedFile::open(path).await.map_err(|e| NotFound(e.to_string()))
}

