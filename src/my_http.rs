use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::to_string;

pub fn start() {
    // Create a new HTTP client
    let client = Client::new();

    // =================================== GET request example ===================================
    let response = client
        .get("https://jsonplaceholder.typicode.com/users/1")
        .send();

    // Check if the response status is success
    match response {
        Ok(resp) => {
            // Print the response status
            println!("Response Status: {}", resp.status());
            // Print the response body
            match resp.text() {
                Ok(body) => println!("Response Body: {}", body),
                Err(e) => eprintln!("Error reading response body: {}", e),
            }
        }
        Err(e) => eprintln!("Error making request: {}", e),
    }

    // =================================== POST request example ===================================
    // title: 'foo',
    // body: 'bar',
    // userId: 1,
    #[derive(Serialize, Deserialize, Debug)]
    struct Post {
        title: String,
        body: String,
        #[serde(rename = "userId")]
        user_id: u32,
    }
    // Create a new Post instance
    let new_post = Post {
        title: String::from("foo"),
        body: String::from("bar"),
        user_id: 1000,
    };
    // Serialize the Post instance to a JSON string
    let mut post_body: String = "".to_string();
    // Use to_string to serialize the Post instance
    let serialized_post = to_string(&new_post);
    // Assign the serialized JSON string to post_body
    match serialized_post {
        Ok(json_string) => post_body = json_string,
        Err(e) => {
            println!("Serialization failed: {}", e);
            return;
        }
    }
    // Make a POST request with the serialized JSON body
    let post_response = client
        .post("https://jsonplaceholder.typicode.com/posts")
        .body(post_body)
        .send();
    // Check if the POST response status is success
    match post_response {
        Ok(resp) => {
            // Print the POST response status
            println!("POST Response Status: {}", resp.status());
            // Print the POST response body
            match resp.text() {
                Ok(body) => println!("POST Response Body: {}", body),
                Err(e) => eprintln!("Error reading POST response body: {}", e),
            }
        }
        Err(e) => eprintln!("Error making POST request: {}", e),
    }
}
