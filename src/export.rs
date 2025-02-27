use std::fs::{self, OpenOptions};
use std::io::Write;

use crate::ReAPI::{self, Post};

/// Export the chats into a .txt file
pub fn export_room_chats_txt(room: ReAPI::Room, out_folder: String) {
    let mut output_buffer: String = String::new();
    let path = format!("{}/messages/{}.txt", out_folder, &room.id[1..10]);

    for message in room.messages() {
        if let ReAPI::Content::Message(text) = message.content {
            let line: String = format!(
                "[{}] {}: {}\n",
                message
                    .timestamp
                    .to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
                    .to_string(),
                message.author,
                text
            );

            output_buffer.push_str(line.as_str());
        } else if let ReAPI::Content::Image(image) = message.content {
            let image_text = format!("FILE: {}", image.id);

            let line: String = format!(
                "[{}] {}: {}\n",
                message
                    .timestamp
                    .to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
                    .to_string(),
                message.author,
                image_text
            );

            output_buffer.push_str(line.as_str());
        }
    }

    std::fs::write(path, output_buffer).unwrap();
}

/// Export the chats into .json files.
pub fn export_room_chats_json(room: ReAPI::Room, out_folder: String) {
    let path = format!("{}/messages/{}.json", out_folder, &room.id[1..10]);

    let file_data = serde_json::to_string(&room).unwrap();

    fs::write(path, file_data).expect("Unable to write file");
}

/// Export chats into csv
pub fn export_room_chats_csv(room: ReAPI::Room, out_folder: String) {
    // Create the file for each chat / room
    let path = format!("{}/messages/{}.csv", out_folder, &room.id[1..10]);

    std::fs::write(path.clone(), "timestamp, author, message \n").unwrap();

    // Iterate over each message in the chat; append to the file
    for message in room.messages() {
        // Format for the line to be appended
        let mut line: String = String::new();

        if let ReAPI::Content::Message(text) = message.content {
            line = format!(
                "{}, {}, {},",
                message
                    .timestamp
                    .to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
                    .to_string(),
                message.author,
                text
            );
        } else if let ReAPI::Content::Image(image) = message.content {
            let image_text = format!("FILE: {}", image.id);

            line = format!(
                "{}, {}, {},",
                message
                    .timestamp
                    .to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
                    .to_string(),
                message.author,
                image_text
            );
        }

        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(path.clone())
            .unwrap();

        if let Err(e) = writeln!(file, "{}", line) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
}

/// Export images from chats
pub fn export_room_images(room: ReAPI::Room, out_folder: String) {
    for message in room.messages() {
        if let ReAPI::Content::Image(image) = message.content {
            std::fs::write(
                format!(
                    "{}/messages/images/{}.{}",
                    out_folder, image.id, image.extension
                ),
                image.data,
            )
            .unwrap();
        }
    }
}

/// Export saved posts
pub fn export_saved_posts(post_array: Vec<Post>, formats: Vec<&str>, out_folder: String) {
    // Export to JSON
    if formats.contains(&"json") {
        let path = format!("{}/saved_posts/saved_posts.json", out_folder);

        let file_data = serde_json::to_string(&post_array).unwrap();

        fs::write(path, file_data).expect("Unable to write file");
    }

    // Export to txt
    if formats.contains(&"txt") {
        let path = format!("{}/saved_posts/saved_posts.txt", out_folder);
        let mut output_buffer: String = String::new();

        for post in &post_array {
            // Iterate over each line and append to .txt file
            let line: String = format!(
                "Title: {}, Subreddit: {}, Permalink: {}, Images {:?}\n",
                post.title, post.subreddit_name, post.permalink, post.img_url
            );

            output_buffer.push_str(line.as_str());
        }
        std::fs::write(path, output_buffer).unwrap();
    }

    if formats.contains(&"csv") {
        // Export to CSV
        let path = format!("{}/saved_posts/saved_posts.csv", out_folder);
        let mut output_buffer: String = "Title, Subreddit, Permalink, Images\n".to_owned();

        for post in post_array {
            // Iterate over each line and append to .txt file
            let line: String = format!(
                "{}, {}, {}, {:?}\n",
                post.title, post.subreddit_name, post.permalink, post.img_url
            );

            output_buffer.push_str(line.as_str());
        }
        std::fs::write(path, output_buffer).unwrap();
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use chrono::Utc;

    use crate::ReAPI;

    #[test]
    fn export_room_chats() {
        // Make folders needed
        if PathBuf::from("./out1").exists() {
            std::fs::remove_dir_all("./out1").expect("Error deleting out folder");
        }

        std::fs::create_dir("./out1").unwrap();
        std::fs::create_dir("./out1/messages").unwrap();
        std::fs::create_dir("./out1/messages/images").unwrap();

        let messages_array: Option<Vec<ReAPI::Message>> = Some(Vec::new());

        let message = ReAPI::Message {
            author: "rexitTest".to_owned(),
            timestamp: Utc::now(),
            content: ReAPI::Content::Message("Testing".to_owned()),
        };
        messages_array.clone().unwrap().push(message);

        let room = ReAPI::Room {
            id: "!fTxOL9GzJaZR71aLRSYstHNVR5j_Zi82L4hIVyjdHuw:reddit.com".to_owned(),
            messages: messages_array,
        };

        // Export it
        super::export_room_chats_csv(room.to_owned(), "./out1".to_owned());
        super::export_room_chats_txt(room.to_owned(), "./out1".to_owned());
        super::export_room_chats_json(room.to_owned(), "./out1".to_owned());
    }

    #[test]
    fn export_saved_posts() {
        // Make folders needed
        if PathBuf::from("./out/saved_posts").exists() {
            std::fs::remove_dir_all("./out").expect("Error deleting out folder");
        }

        std::fs::create_dir("./out/").unwrap();
        std::fs::create_dir("./out/saved_posts").unwrap();

        let mut posts: Vec<ReAPI::Post> = Vec::new();

        let post = ReAPI::Post {
            title: "Da fehlt doch was".to_owned(),
            subreddit_name: "r/hamburg".to_owned(),
            permalink: "/r/hamburg/comments/134bv4v/da_fehlt_doch_was/".to_owned(),
            img_url: ["https://preview.redd.it/…051acd31351105e323c5d7a6".to_owned()].to_vec(),
        };
        posts.push(post);

        super::export_saved_posts(posts, ["txt", "json", "csv"].to_vec(), "./out".to_owned())
    }
}
