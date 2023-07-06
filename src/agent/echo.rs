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
    fn init(&mut self, msg: &Message) -> Result<&mut Self> {
        if let Body::Init { node_id, .. } = &msg.body {
            self.node_id = Some(node_id.clone());
            Ok(self)
        } else {
            Err(Error::NodeError {
                msg: Some(Self::response_node_not_initialized(&msg)),
                detail: "Expected an init message.".to_string(),
            })
        }
    }

    fn response(&mut self, msg: &Message) -> Result<Message> {
        match &msg.body {
            Body::Init { .. } => {
                self.init(&msg)?;
                self.response_init_ok(&msg)
            }
            Body::Echo { msg_id, echo } => Ok(Message {
                src: msg.dest.clone(),
                dest: msg.src.clone(),
                body: Body::EchoOk {
                    msg_id: self.generate_msg_id(),
                    in_reply_to: *msg_id,
                    echo: echo.clone(),
                },
            }),
            _ => Err(Error::NodeError {
                msg: Some(Self::response_not_supported(&msg)),
                detail: "Can only respond to 'echo' messages.".to_string(),
            }),
        }
    }
}
