#![deny(warnings)]

use std::{fs::File, io::Write};
use serde_derive::{Deserialize, Serialize};

use warp::Filter;

#[derive(Deserialize, Serialize)]
struct FileCreater {
   file_name : String,
    content: String,
    extension: String,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();


    let promote = warp::post()
        .and(warp::path("filecreator"))      
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .map(|fc: FileCreater| {
            let mut file = File::create(format!("{}.{}",fc.file_name,fc.extension)).unwrap();
            file.write_all(fc.content.as_bytes()).unwrap();
            warp::reply::json(&fc)
        });
        warp::serve(promote).run(([127, 0, 0, 1], 3030)).await

        
}
