use warp::Filter;
use super::handlers;

// A function to build our routes
pub fn routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_blog_post()
}

// A route to handle GET requests for a specific post
fn get_blog_post() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("posts" / u64)
        .and(warp::get())
        .and_then(handlers::get_blog_post)
}

fn post_blog_post() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("posts")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handlers::create_blog_post)
}
