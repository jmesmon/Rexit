use super::Client;
use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Clone, Serialize)]
pub struct Room {
    pub id: String,
    pub(crate) messages: Option<Vec<super::Message>>,
}

impl Room {
    fn download(id: String, client: &Client) -> Room {
        Room {
            id: id.clone(),
            messages: download_messages(&client, id.clone()),
        }
    }

    pub fn messages(&self) -> Vec<super::Message> {
        return self.messages.clone().unwrap();
    }
}

fn download_messages(client: &Client, id: String) -> Option<Vec<super::Message>> {
    Some(super::messages::list_messages(client, id))
}

/// Returns list of all rooms that the user is joined to as per [SPEC](https://spec.matrix.org/v1.6/client-server-api/#get_matrixclientv3directorylistroomroomid)
pub fn download_rooms(client: &Client) -> Vec<Room> {
    let resp = client
        .reqwest_client
        .get("https://matrix.redditspace.com/_matrix/client/v3/joined_rooms")
        .header("Authorization", format!("Bearer {}", client.bearer_token()))
        .send()
        .expect("Failed to send HTTP request; to obtain rooms");

    // Parse json
    let json: Value =
        serde_json::from_str(&resp.text().unwrap()).expect("Error parsing Rooms list JSON");

    // Read rooms from json
    let rooms = json["joined_rooms"]
        .as_array()
        .expect("Error parsing array")
        .to_owned();

    // Move rooms into a Vec<Room>
    let rooms: Vec<Room> = rooms
        .iter()
        .map(|room| Room::download(room.to_string().replace("\"", ""), client))
        .collect();

    info!("Found {} room(s) ", rooms.len());

    return rooms;
}

#[cfg(test)]
mod tests {
    #[test]
    #[ignore = "creds"]
    fn list_rooms() {
        let (username, password) = get_login();
        let mut client = super::super::new_client(true);

        client.login(username, password);

        let rooms = super::download_rooms(&client);

        println!("{:?}", rooms);
    }

    fn get_login() -> (String, String) {
        let username = std::env::var("REXIT_USERNAME").expect("Could not find username in env");
        let password = std::env::var("REXIT_PASSWORD").expect("Could not find password in env");

        (username, password)
    }
}
