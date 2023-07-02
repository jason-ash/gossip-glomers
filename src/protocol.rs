use serde::{Deserialize, Serialize};

pub type MessageId = usize;
pub type NodeId = String;

#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    pub src: NodeId,
    pub dest: NodeId,
    pub body: Body<Payload>,
}

impl Message {
    pub fn get_type(&self) -> &Payload {
        &self.body.payload
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Body<Payload> {
    pub msg_id: Option<MessageId>,
    pub in_reply_to: Option<MessageId>,
    #[serde(flatten)]
    pub payload: Payload,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Payload {
    Echo {
        echo: String,
    },
    EchoOk {
        echo: String,
    },
    Error {
        text: String,
        code: usize,
    },
    Init {
        node_id: NodeId,
        mode_ids: Vec<NodeId>,
    },
    InitOk,
}

// #[derive(Debug, Deserialize, Serialize)]
// pub struct EchoPayload {
//     pub echo: String,
// }
//
// #[derive(Debug, Deserialize, Serialize)]
// pub struct EchoOkPayload {
//     pub echo: String,
// }
//
// #[derive(Debug, Deserialize, Serialize)]
// pub struct ErrorPayload {
//     pub text: String,
//     pub code: usize,
// }
//
// #[derive(Debug, Deserialize, Serialize)]
// pub struct InitPayload {
//     pub node_id: NodeId,
//     pub node_ids: Vec<NodeId>,
// }
