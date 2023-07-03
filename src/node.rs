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
    fn new(msg: &Message) -> Result<Self>;

    /// return the node_id for this node, e.g. "n1".
    fn node_id(&self) -> NodeId;

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
                    msg_id: Some(self.generate_msg_id()),
                    in_reply_to: Some(msg.body.msg_id.expect("to find a msg_id")),
                    payload: Payload::InitOk,
                },
            })
        } else {
            Err(Error::NodeError {
                msg: None,
                detail: "Need an init message to respond to.".to_string(),
            })
        }
    }
}
