use std::path::{Path, PathBuf};
use rocket::response::NamedFile;

#[get("/static/<file..>")]
fn all(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}