use crate::error::{Error, Result};
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

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string(&self).map_err(|_| std::fmt::Error)?
        )
    }
}

impl TryFrom<&str> for Message {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self> {
        serde_json::from_str(value)?
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
