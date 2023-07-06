use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};

pub type MessageId = usize;
pub type NodeId = String;

#[derive(Debug, Deserialize, Serialize)]
pub struct Message {
    pub src: NodeId,
    pub dest: NodeId,
    pub body: Body,
}

impl Message {
    pub fn msg_id(&self) -> Option<MessageId> {
        match self.body {
            Body::Broadcast { msg_id, .. } => Some(msg_id),
            Body::Echo { msg_id, .. } => Some(msg_id),
            Body::Generate { msg_id, .. } => Some(msg_id),
            Body::Init { msg_id, .. } => Some(msg_id),
            Body::Error { .. } => None,
            Body::BroadcastOk { .. } => None,
            Body::EchoOk { .. } => None,
            Body::GenerateOk { .. } => None,
            Body::InitOk { .. } => None,
        }
    }

    pub fn in_reply_to(&self) -> Option<MessageId> {
        match self.body {
            Body::BroadcastOk { in_reply_to, .. } => Some(in_reply_to),
            Body::EchoOk { in_reply_to, .. } => Some(in_reply_to),
            Body::GenerateOk { in_reply_to, .. } => Some(in_reply_to),
            Body::InitOk { in_reply_to, .. } => Some(in_reply_to),
            Body::Error { in_reply_to, .. } => Some(in_reply_to),
            Body::Echo { .. } => None,
            Body::Broadcast { .. } => None,
            Body::Generate { .. } => None,
            Body::Init { .. } => None,
        }
    }
}

impl std::fmt::Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = serde_json::to_string(&self).map_err(|_| std::fmt::Error)?;
        write!(f, "{}", value)
    }
}

impl TryFrom<&str> for Message {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self> {
        Ok(serde_json::from_str(value)?)
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Body {
    Broadcast {
        msg_id: MessageId,
        message: String,
    },
    BroadcastOk {
        msg_id: MessageId,
        in_reply_to: MessageId,
        message: String,
    },
    Echo {
        msg_id: MessageId,
        echo: String,
    },
    EchoOk {
        msg_id: MessageId,
        in_reply_to: MessageId,
        echo: String,
    },
    Error {
        in_reply_to: MessageId,
        code: usize,
        text: String,
    },
    Generate {
        msg_id: MessageId,
    },
    GenerateOk {
        msg_id: MessageId,
        in_reply_to: MessageId,
        id: String,
    },
    Init {
        msg_id: MessageId,
        node_id: NodeId,
        node_ids: Vec<NodeId>,
    },
    InitOk {
        in_reply_to: MessageId,
    },
}
