use std::collections::HashSet;

use crate::{
    error::{Error, Result},
    node::Node,
    protocol::{Body, Message, MessageId, NodeId},
};

#[derive(Debug)]
pub struct BroadcastAgent {
    pub node_id: Option<NodeId>,
    pub node_ids: Option<Vec<NodeId>>,
    pub neighbors: Option<Vec<NodeId>>,
    pub msg_id: MessageId,
    pub messages: HashSet<usize>,
}

impl BroadcastAgent {
    pub fn new() -> Self {
        Self {
            node_id: None,
            node_ids: None,
            neighbors: None,
            msg_id: 0,
            messages: HashSet::new(),
        }
    }

    fn generate_msg_id(&mut self) -> MessageId {
        let out = self.msg_id;
        self.msg_id = out + 1;
        out
    }
}

impl Node for BroadcastAgent {
    fn handler(&mut self, msg: &Message) -> Result<Vec<Message>> {
        match &msg.body {
            Body::Init {
                msg_id,
                node_id,
                node_ids,
            } => {
                self.node_id = Some(node_id.clone());
                self.node_ids = Some(node_ids.clone());
                let reply = Message {
                    src: msg.dest.clone(),
                    dest: msg.src.clone(),
                    body: Body::InitOk {
                        in_reply_to: msg_id.clone(),
                    },
                };
                Ok(vec![reply])
            }
            Body::Topology { msg_id, topology } => {
                if let Some(neighbors) =
                    topology.get(self.node_id.as_ref().expect("to find node_id"))
                {
                    self.neighbors = Some(neighbors.clone());
                }

                let reply = Message {
                    src: msg.dest.clone(),
                    dest: msg.src.clone(),
                    body: Body::TopologyOk {
                        in_reply_to: msg_id.clone(),
                    },
                };
                Ok(vec![reply])
            }
            Body::Broadcast { msg_id, message } => {
                let mut replies = Vec::new();
                let reply = Message {
                    src: msg.dest.clone(),
                    dest: msg.src.clone(),
                    body: Body::BroadcastOk {
                        in_reply_to: msg_id.clone(),
                    },
                };
                replies.push(reply);
                if self.messages.insert(message.clone()) {
                    let neighbors = self.neighbors.as_ref().expect("to find neighbors").clone();
                    let broadcasts = neighbors.iter().map(|n| {
                        let generated_id = &self.generate_msg_id();
                        Message {
                            src: self.node_id.as_ref().expect("to find node_id").clone(),
                            dest: n.clone(),
                            body: Body::Broadcast {
                                msg_id: generated_id.clone(),
                                message: message.clone(),
                            },
                        }
                    });
                    replies.extend(broadcasts);
                };
                Ok(replies)
            }
            Body::BroadcastOk { .. } => Ok(Vec::new()),
            Body::Read { msg_id } => {
                let reply = Message {
                    src: msg.dest.clone(),
                    dest: msg.src.clone(),
                    body: Body::ReadOk {
                        in_reply_to: msg_id.clone(),
                        messages: self.messages.clone(),
                    },
                };
                Ok(vec![reply])
            }
            _ => Err(Error::NodeError {
                msg: Some(Message {
                    src: msg.dest.clone(),
                    dest: msg.src.clone(),
                    body: Body::Error {
                        in_reply_to: msg.msg_id().expect("to find a msg_id"),
                        code: 10,
                        text: format!(
                            "This node does not handle messages with body type: '{}'",
                            msg.body
                        ),
                    },
                }),
                detail: String::default(),
            }),
        }
    }
}
