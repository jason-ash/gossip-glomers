use crate::{error::Result, protocol::Message};
use std::fmt::Debug;

pub trait Node: Debug + Sized {
    fn handler(&mut self, msg: &Message) -> Result<Vec<Message>>;
}
