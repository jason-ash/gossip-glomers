use crate::{
    error::{Error, Result},
    node::Node,
    protocol::{Body, Message, MessageId, NodeId, Payload},
};

#[derive(Debug)]
pub struct UniqueIdAgent {
    pub node_id: Option<NodeId>,
    pub msg_id: MessageId,
}

impl UniqueIdAgent {
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

impl Node for UniqueIdAgent {
    fn init(&mut self, msg: &Message) -> Result<&mut Self> {
        if let Payload::Init { node_id, .. } = msg.get_type() {
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
        match msg.get_type() {
            Payload::Init { .. } => {
                self.init(&msg)?;
                self.response_init_ok(&msg)
            }
            Payload::Generate {} => {
                let msg_id = self.generate_msg_id();
                Ok(Message {
                    src: msg.dest.clone(),
                    dest: msg.src.clone(),
                    body: Body {
                        msg_id: Some(msg_id),
                        in_reply_to: Some(msg.body.msg_id.expect("to find a msg_id")),
                        payload: Payload::GenerateOk {
                            id: format!(
                                "{}-{}",
                                self.node_id.as_ref().expect("node to be initialized."),
                                msg_id
                            ),
                        },
                    },
                })
            }
            _ => Err(Error::NodeError {
                msg: Some(Self::response_not_supported(&msg)),
                detail: "Can only respond to 'generate' messages.".to_string(),
            }),
        }
    }
}
