use crate::{
    error::{Error, Result},
    node::Node,
    protocol::{Body, Message, MessageId, NodeId, Payload},
};

#[derive(Debug)]
pub struct UniqueIdAgent {
    pub node_id: NodeId,
    pub msg_id: MessageId,
}

impl Node for UniqueIdAgent {
    fn new(msg: &Message) -> Result<Self> {
        match &msg.body.payload {
            Payload::Init { node_id, .. } => Ok(Self {
                node_id: node_id.to_owned(),
                msg_id: 0,
            }),
            _ => Err(crate::error::Error::NodeError(
                "Expected an init message.".to_string(),
            )),
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
            Payload::Generate {} => {
                let msg_id = self.generate_msg_id();
                Ok(Message {
                    src: msg.dest.clone(),
                    dest: msg.src.clone(),
                    body: Body {
                        msg_id: Some(msg_id),
                        in_reply_to: Some(msg.body.msg_id.expect("to find a msg_id")),
                        payload: Payload::GenerateOk {
                            id: format!("{}-{}", self.node_id, msg_id),
                        },
                    },
                })
            }
            _ => Err(Error::NodeError(
                "Can only respond to 'generate' messages.".to_string(),
            )),
        }
    }
}
