use crate::{
    error::{Error, Result},
    node::Node,
    protocol::{Message, Payload},
};
use std::io::{self, BufRead, Write};

pub struct Runtime<N: Node> {
    pub node: Option<N>,
}

impl<N: Node> Runtime<N> {
    pub fn new() -> Self {
        Self { node: None }
    }

    pub fn start(&mut self) -> Result<()> {
        let stdin = io::stdin().lock();
        let mut stdout = io::stdout().lock();
        let mut stderr = io::stderr().lock();

        for line in stdin.lines() {
            let line = line?;
            match Message::try_from(line.as_ref()) {
                Ok(msg) => {
                    if let Payload::Init { .. } = msg.get_type() {
                        self.node = Some(N::new(&msg)?);
                        let response = &self
                            .node
                            .as_mut()
                            .expect("node to exist")
                            .response_init_ok(&msg)?;
                        Self::send(&response, &mut stdout)?;
                    } else if self.node.is_none() {
                        let response = N::response_node_not_initialized(&msg);
                        Self::send(&response, &mut stderr)?;
                    } else {
                        let response = &self
                            .node
                            .as_mut()
                            .expect("to find an initialized node")
                            .response(&msg);
                        match &response {
                            Ok(msg) => Self::send(msg, &mut stdout)?,
                            Err(Error::NodeError { msg: Some(err), .. }) => {
                                Self::send(err, &mut stderr)?
                            }
                            _ => (),
                        }
                    }
                }
                Err(e) => write!(stderr, "{:?}\n", e)?,
            }
        }

        Ok(())
    }

    pub fn send(msg: &Message, dest: &mut dyn Write) -> Result<()> {
        Ok(write!(dest, "{}\n", msg)?)
    }
}
