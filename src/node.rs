use crate::{
    error::{Error, Result},
    protocol::{Body, Message, MessageId, NodeId, Payload},
};
use std::fmt::Debug;

pub trait Node
where
    Self: Debug + Sized,
{
    /// construct a new node from an init message,
    fn init(msg: &Message) -> Result<Self>;

    /// return the node_id for this node, e.g. "n1".
    fn node_id(&self) -> &NodeId;

    /// return a unique msg_id for messages from this node.
    fn generate_msg_id(&mut self) -> MessageId;

    /// return a message in resonse to other messages
    fn response(&mut self, msg: &Message) -> Result<Message>;

    /// generate an init_ok message in response to an init message.
    fn response_init_ok(&mut self, msg: &Message) -> Result<Message> {
        if let Payload::Init { .. } = msg.get_type() {
            Ok(Message {
                src: msg.dest.clone(),
                dest: msg.src.clone(),
                body: Body {
                    msg_id: None,
                    in_reply_to: Some(msg.body.msg_id.expect("to find a msg_id")),
                    payload: Payload::InitOk,
                },
            })
        } else {
            Err(Error::NodeError {
                msg: Some(Self::response_node_not_initialized(&msg)),
                detail: "Need an init message to respond to.".to_string(),
            })
        }
    }

    /// generate an error message if the node hasn't been initialized, yet receives a message that
    /// is not init
    fn response_node_not_initialized(msg: &Message) -> Message {
        Message {
            src: msg.dest.clone(),
            dest: msg.src.clone(),
            body: Body {
                msg_id: None,
                in_reply_to: Some(msg.body.msg_id.expect("to find a msg_id")),
                payload: Payload::Error {
                    code: 1,
                    text: "This node doesn't exist; expecting an `init` message first.".into(),
                },
            },
        }
    }

    fn response_not_supported(msg: &Message) -> Message {
        Message {
            src: msg.dest.clone(),
            dest: msg.src.clone(),
            body: Body {
                msg_id: None,
                in_reply_to: Some(msg.body.msg_id.expect("to find a msg_id")),
                payload: Payload::Error {
                    code: 10,
                    text: "This node doesn't support messages of this type.".into(),
                },
            },
        }
    }
}
