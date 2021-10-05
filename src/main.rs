mod Files;
mod Database;
use Files::*;
use warp::Filter;
use Database::*;


#[tokio::main]
async fn main() {
    
    let db = blank_db();

    let post =   warp::path!("filecreator")
    .and(warp::post())
    .and(warp::body::content_length_limit(1024 * 16).and(warp::body::json()))
    .and(with_db(db)) /*
    here is an issue ,
     i can't use db variable due to borrowship ,
      so i'm trying to use Arc pointer on it
        */
    .and_then(create_file);
    
    let get  =  warp::path!("filecreator")
    .and(warp::get())
    .and(with_db(db.clone()))
    .and_then(get_file);

    let put  =  warp::path!("filecreator" / String)
    .and(warp::put())
    .and(warp::body::content_length_limit(1024 * 16).and(warp::body::json()))
    .and(with_db(db.clone()))
    .and_then(update_file);

    let delete =  warp::path!("filecreator" / String)
    .and(warp::delete())
    .and(with_db(db))
    .and_then(delete_file);

    let routes = post
    .or(get)
    .or(put)
    .or(delete);
        warp::serve(routes).run(([127, 0, 0, 1], 3030)).await

        
}


    




    