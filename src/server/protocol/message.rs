use serde_derive::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize, Debug)]
struct ReqData {
    message_id: u64,
    sender_id: u64,
    sender_nickname: String,
    message_type: MsgType,
    content: Vec<u8>,
    receiver: String,
    timestamp: Vec<u8>,
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

#[derive(Serialize, Deserialize, Debug)]
struct ResData {
    message_id: u64,
    message_status: MsgStatus,
    timestamp: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Response {
    #[serde(rename = "type")]
    m_type: ReType,
    status: Status,
    data: ResData,
}
