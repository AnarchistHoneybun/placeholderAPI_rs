use reqwest;
use serde::Deserialize;

// Define a struct to represent the data structure of a post
#[derive(Debug, Deserialize)]
struct Post {
    userId: u32,
    id: u32,
    title: String,
    body: String,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // Specify the API endpoint URL
    let url = "https://jsonplaceholder.typicode.com/posts/1";

    // Make a GET request to the API endpoint
    let response = reqwest::get(url).await?;

    // Check if the request was successful (HTTP status code 200)
    if response.status().is_success() {
        // Parse the JSON response into a Post struct
        let post: Post = response.json().await?;

        // Display the post details
        println!("Post ID: {}", post.id);
        println!("User ID: {}", post.userId);
        println!("Title: {}", post.title);
        println!("Body: {}", post.body);
    } else {
        // Display an error message if the request was not successful
        eprintln!("Failed to fetch data: {}", response.status());
    }

    Ok(())
}
