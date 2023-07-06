use crate::{
    error::{Error, Result},
    node::Node,
    protocol::Message,
};
use std::io::{self, BufRead, Write};

pub struct Runtime<'a, N: Node> {
    pub node: &'a mut N,
}

impl<'a, N: Node> Runtime<'a, N> {
    pub fn new(node: &'a mut N) -> Self {
        Self { node }
    }

    pub fn start(&mut self) -> Result<()> {
        let stdin = io::stdin().lock();
        let mut stdout = io::stdout().lock();
        let mut stderr = io::stderr().lock();

        for line in stdin.lines() {
            let line = line?;
            match Message::try_from(line.as_ref()) {
                Ok(msg) => match self.node.response(&msg) {
                    Ok(response) => Self::send(&response, &mut stdout)?,
                    Err(Error::NodeError {
                        msg: Some(response),
                        detail,
                    }) => {
                        Self::send(&response, &mut stdout)?;
                        write!(stderr, "{}\n", detail)?;
                    }
                    Err(e) => write!(stderr, "{:?}\n", e)?,
                },
                Err(e) => write!(stderr, "{:?}\n", e)?,
            }
        }

        Ok(())
    }

    pub fn send(msg: &Message, dest: &mut dyn Write) -> Result<()> {
        Ok(write!(dest, "{}\n", msg)?)
    }
}
