use std::collections::{HashMap, HashSet};

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
            Body::Read { msg_id, .. } => Some(msg_id),
            Body::Topology { msg_id, .. } => Some(msg_id),
            Body::Error { .. } => None,
            Body::BroadcastOk { .. } => None,
            Body::EchoOk { .. } => None,
            Body::GenerateOk { .. } => None,
            Body::InitOk { .. } => None,
            Body::ReadOk { .. } => None,
            Body::TopologyOk { .. } => None,
        }
    }

    pub fn in_reply_to(&self) -> Option<MessageId> {
        match self.body {
            Body::BroadcastOk { in_reply_to, .. } => Some(in_reply_to),
            Body::EchoOk { in_reply_to, .. } => Some(in_reply_to),
            Body::GenerateOk { in_reply_to, .. } => Some(in_reply_to),
            Body::InitOk { in_reply_to, .. } => Some(in_reply_to),
            Body::ReadOk { in_reply_to, .. } => Some(in_reply_to),
            Body::TopologyOk { in_reply_to, .. } => Some(in_reply_to),
            Body::Error { in_reply_to, .. } => Some(in_reply_to),
            Body::Echo { .. } => None,
            Body::Broadcast { .. } => None,
            Body::Generate { .. } => None,
            Body::Init { .. } => None,
            Body::Read { .. } => None,
            Body::Topology { .. } => None,
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
        message: usize,
    },
    BroadcastOk {
        in_reply_to: MessageId,
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
    Read {
        msg_id: MessageId,
    },
    ReadOk {
        in_reply_to: MessageId,
        messages: HashSet<usize>,
    },
    Topology {
        msg_id: MessageId,
        topology: HashMap<NodeId, Vec<NodeId>>,
    },
    TopologyOk {
        in_reply_to: MessageId,
    },
}

impl std::fmt::Display for Body {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Broadcast { .. } => write!(f, "broadcast"),
            Self::BroadcastOk { .. } => write!(f, "broadcast_ok"),
            Self::Echo { .. } => write!(f, "echo"),
            Self::EchoOk { .. } => write!(f, "echo_ok"),
            Self::Error { .. } => write!(f, "error"),
            Self::Generate { .. } => write!(f, "generate"),
            Self::GenerateOk { .. } => write!(f, "generate_ok"),
            Self::Init { .. } => write!(f, "init"),
            Self::InitOk { .. } => write!(f, "init_ok"),
            Self::Read { .. } => write!(f, "read"),
            Self::ReadOk { .. } => write!(f, "read_ok"),
            Self::Topology { .. } => write!(f, "topology"),
            Self::TopologyOk { .. } => write!(f, "topology_ok"),
        }
    }
}
