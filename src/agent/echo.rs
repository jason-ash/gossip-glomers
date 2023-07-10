use crate::{
    error::{Error, Result},
    node::Node,
    protocol::{Body, Message, MessageId, NodeId},
};

#[derive(Debug)]
pub struct EchoAgent {
    pub node_id: Option<NodeId>,
    pub msg_id: MessageId,
}

impl EchoAgent {
    pub fn new() -> Self {
        Self {
            node_id: None,
            msg_id: 0,
        }
    }

    fn generate_msg_id(&mut self) -> MessageId {
        let out = self.msg_id;
        self.msg_id = out + 1;
        out
    }
}

impl Node for EchoAgent {
    fn handler(&mut self, msg: &Message) -> Result<Vec<Message>> {
        match &msg.body {
            Body::Init { .. } => {
                self.node_id = Some(msg.dest.clone());
                let reply = Message {
                    src: msg.dest.clone(),
                    dest: msg.src.clone(),
                    body: Body::InitOk {
                        in_reply_to: msg.msg_id().expect("to find a msg_id"),
                    },
                };
                Ok(vec![reply])
            }
            Body::Echo { echo, .. } => {
                let msg_id = self.generate_msg_id();
                let reply = Message {
                    src: msg.dest.clone(),
                    dest: msg.src.clone(),
                    body: Body::EchoOk {
                        msg_id,
                        in_reply_to: msg.msg_id().expect("to find a msg_id"),
                        echo: echo.clone(),
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
