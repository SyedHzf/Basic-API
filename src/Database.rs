use warp::Filter;
use std::sync::Arc;
use serde_derive::{Deserialize, Serialize};
use tokio::sync::Mutex;

#[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct FileCreator {
       pub file_name : String,
        pub content: String, 
       pub extension: String,
 }
    pub type Db = Arc<Mutex<FileCreator>>;
    
  pub fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
  }
    pub fn blank_db() -> Db {
        Arc::new(Mutex::new(FileCreator{
            
            file_name: String::from(""),
            content:  String::from(""),
            extension:  String::from(""),
        }))
    }
    