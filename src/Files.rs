use super::Database::{FileCreator,Db};
use std::convert::Infallible;
use warp::http::StatusCode;

//Creating Fule
    pub async fn create_file(create: FileCreator, db: Db) -> Result<impl warp::Reply, Infallible> {
        let mut file = db.lock().await;

        // file with name already exists, return `400 BadRequest`.    
        if file.file_name == create.file_name {
                return Ok(StatusCode::BAD_REQUEST); }
        // No existing , so insert and return `201 Created`.
        else{
            file.file_name = create.file_name;
            file.content = create.content;
            file.extension = create.extension;
            Ok(StatusCode::CREATED)
    } 
}

// Getting file
    pub async fn get_file(db: Db) -> Result<impl warp::Reply, Infallible> {
        // Trying to fetch the struct 

        let file = db.lock().await;
        let todos  = FileCreator{
            file_name:file.file_name.clone() ,
            content:file.content.clone(),
            extension:file.extension.clone()
        } ;
        Ok(warp::reply::json(&todos))
}

// Updatting File
pub async fn update_file(filename: String,update: FileCreator,db: Db,) -> Result<impl warp::Reply, Infallible> {
    let mut file = db.lock().await;

    // if file name match updte it
    if file.file_name == filename {
            *file = update;
            return Ok(StatusCode::OK);
    }// otherwise not found error
    else{
              Ok(StatusCode::NOT_FOUND)
    }
}

//Deleting File
pub async fn delete_file(filename:String,db: Db) -> Result<impl warp::Reply, Infallible> {
    let mut file  = db.lock().await;
// if file name match it will be deleted
    if file.file_name == filename {
        *file = FileCreator{
            
            file_name: String::from(""),
            content:  String::from(""),
            extension:  String::from(""),
        };
        Ok(StatusCode::NO_CONTENT)
    } //otherwie notefound status will rise 
    else {
//
        Ok(StatusCode::NOT_FOUND)
    }
}

