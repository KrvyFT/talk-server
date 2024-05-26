use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use serde_with::{base64::Base64, serde_as, DisplayFromStr};

#[derive(Serialize, Deserialize, Debug)]
enum ReType {
    Request,
    Response,
}

#[derive(Serialize, Deserialize, Debug)]
enum Action {
    Login,
    SendMessage,
}

#[derive(Serialize, Deserialize, Debug)]
enum MsgType {
    Text,
    Image,
    File,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
struct ReqData {
    message_id: u64,
    sender_id: u64,
    sender_nickname: String,
    message_type: MsgType,
    #[serde_as(as = "Base64")]
    content: Vec<u8>,
    receiver: String,
    #[serde_as(as = "DisplayFromStr")]
    timestamp: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Request {
    #[serde(rename = "type")]
    m_type: ReType,
    action: Action,
    data: ReqData,
}

#[derive(Serialize, Deserialize, Debug)]
enum Status {
    Success,
    Error,
}

#[derive(Serialize, Deserialize, Debug)]
enum MsgStatus {
    Delivered,
    Read,
    Undelivered,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug)]
struct ResData {
    message_id: u64,
    message_status: MsgStatus,
    #[serde_as(as = "DisplayFromStr")]
    timestamp: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    #[serde(rename = "type")]
    m_type: ReType,
    status: Status,
    data: ResData,
}

trait SerializeAndDeserialize {
    async fn to_json(&self) -> String;

    async fn from_json(str: &str) -> Self;

    async fn save_to_file(&self, filename: &str) -> Result<(), std::io::Error> {
        let json_string = self.to_json().await;
        let mut file = std::fs::File::create(filename)?;
        std::io::Write::write_all(&mut file, json_string.as_bytes())?;
        Ok(())
    }
}

impl SerializeAndDeserialize for Request {
    async fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    async fn from_json(str: &str) -> Self {
        serde_json::from_str(&str).unwrap()
    }
}

impl SerializeAndDeserialize for Response {
    async fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    async fn from_json(str: &str) -> Self {
        serde_json::from_str(&str).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[tokio::test]
    async fn test_request_serialization_and_deserialization() {
        // Create a sample Request
        let request = Request {
            m_type: ReType::Request,
            action: Action::SendMessage,
            data: ReqData {
                message_id: 12345,
                sender_id: 98765,
                sender_nickname: "Alice".to_string(),
                message_type: MsgType::Text,
                content: b"Hello, world!".to_vec(),
                receiver: "Bob".to_string(),
                timestamp: Utc::now(),
            },
        };

        // Serialize
        let json_string = request.to_json().await;

        // Deserialize
        let deserialized_request = Request::from_json(&json_string).await;
        deserialized_request
            .save_to_file("filename.json")
            .await
            .unwrap();
    }
}
