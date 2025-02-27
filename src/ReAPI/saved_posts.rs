use url::Url;

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use super::{images, Client};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedList {
    posts: Vec<Post>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub title: String,
    pub subreddit_name: String,
    pub permalink: String,
    pub img_url: Vec<String>,
}

pub fn download_saved_posts(client: &Client, image_download: bool) -> Vec<Post> {
    info!("Getting Saved Posts");

    let mut after_token = String::new();
    let mut saved_list: Vec<Post> = Vec::<Post>::new();

    loop {
        let url = format!("https://www.reddit.com/saved.json?after={after_token}");

        let response = client
            .reqwest_client
            .get(url)
            .send()
            .expect("Failed to send HTTP request");

        let saved_posts: Result<Value, _> = serde_json::from_str(response.text().unwrap().as_str());
        if saved_posts.is_err() {
            return vec![];
        }
        let saved_posts = saved_posts.unwrap();

        // Iterates over all saved posts in the response array
        for post in saved_posts["data"]["children"].as_array().unwrap() {
            // Get all image urls
            let mut images = Vec::<String>::new();

            // If post has images
            if !post["data"]["preview"].is_null() {
                for image in post["data"]["preview"]["images"].as_array().unwrap() {
                    // By default these urls are for the reddit cache that requires auth
                    // but the img ID is same as the non-cached one (i.redd.it/)
                    let url = image["source"]["url"].as_str().unwrap().to_string();
                    let fixed_url = Url::parse(&url).unwrap();
                    let final_url = format!("https://i.redd.it{}", fixed_url.path());

                    if image_download {
                        images::get_image(&client, final_url.clone());
                    }

                    images.push(final_url.to_owned())
                }
            }

            // Link posts require extra massaging to make work
            if !post["data"]["link_title"].is_null() {
                let post = Post {
                    title: post["data"]["link_title"].as_str().unwrap().to_string(),
                    subreddit_name: post["data"]["subreddit_name_prefixed"]
                        .as_str()
                        .unwrap()
                        .to_string(),
                    permalink: post["data"]["permalink"].as_str().unwrap().to_string(),
                    img_url: images,
                };

                saved_list.push(post);
            } else {
                // Normal text post
                let post = Post {
                    title: post["data"]["title"].as_str().unwrap().to_string(),
                    subreddit_name: post["data"]["subreddit_name_prefixed"]
                        .as_str()
                        .unwrap()
                        .to_string(),
                    permalink: post["data"]["permalink"].as_str().unwrap().to_string(),
                    img_url: images,
                };

                saved_list.push(post);
            }
        }
        if saved_posts["data"]["after"] == json!(null) {
            break;
        }

        after_token = saved_posts["data"]["after"].as_str().unwrap().to_string();
    }

    return saved_list;
}
