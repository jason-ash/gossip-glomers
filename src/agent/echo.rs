use crate::{
    error::{Error, Result},
    node::Node,
    protocol::{Body, Message, MessageId, NodeId, Payload},
};

#[derive(Debug)]
pub struct EchoAgent {
    pub node_id: NodeId,
    pub msg_id: MessageId,
}

impl Node for EchoAgent {
    fn new(msg: &Message) -> Result<Self> {
        if let Payload::Init { node_id, .. } = msg.get_type() {
            Ok(Self {
                node_id: node_id.clone(),
                msg_id: 0,
            })
        } else {
            Err(Error::NodeError {
                msg: None,
                detail: "Expected an init message.".to_string(),
            })
        }
    }

    fn node_id(&self) -> NodeId {
        self.node_id.clone()
    }

    fn generate_msg_id(&mut self) -> MessageId {
        let out = self.msg_id;
        self.msg_id = out + 1;
        out
    }

    fn response(&mut self, msg: &Message) -> Result<Message> {
        match msg.get_type() {
            Payload::Echo { echo } => Ok(Message {
                src: msg.dest.clone(),
                dest: msg.src.clone(),
                body: Body {
                    msg_id: Some(self.generate_msg_id()),
                    in_reply_to: Some(msg.body.msg_id.expect("to find a msg_id")),
                    payload: Payload::EchoOk { echo: echo.clone() },
                },
            }),
            _ => Err(Error::NodeError {
                msg: None,
                detail: "Can only respond to 'echo' messages.".to_string(),
            }),
        }
    }
}
