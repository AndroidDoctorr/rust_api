use warp::Filter;
use super::models::Post;
use rand::Rng; // Add rand to your Cargo.toml dependencies

pub async fn create_blog_post(new_post: NewPost) -> Result<impl warp::Reply, warp::Rejection> {
    // Generate a random ID for the new post
    let id = rand::thread_rng().gen::<u64>();

    // Create a Post instance with the generated ID
    let post = Post {
        id,
        title: new_post.title,
        body: new_post.body,
    };

    // Here you would save the post to your database
    // For now, we'll just echo the post back as a JSON response
    Ok(warp::reply::json(&post))
}

// A function to handle GET requests at /posts/{id}
pub async fn get_blog_post(id: u64) -> Result<impl warp::Reply, warp::Rejection> {
    // For simplicity, let's say we are returning a static post
    let post = Post {
        id,
        title: String::from("Hello, Warp!"),
        body: String::from("This is a post about Warp."),
    };
    Ok(warp::reply::json(&post))
}