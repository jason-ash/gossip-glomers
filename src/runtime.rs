use crate::{
    error::Result,
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
            match serde_json::from_str::<Message>(&line) {
                Ok(msg) => match msg.get_type() {
                    Payload::Init { .. } => {
                        self.node = Some(N::new(&msg)?);
                        write!(
                            stdout,
                            "{}\n",
                            serde_json::to_string(
                                &self.node.as_mut().unwrap().response_init_ok(&msg)?
                            )?
                        )?;
                    }
                    _ => {
                        let response = &self.node.as_mut().unwrap().response(&msg)?;
                        write!(stdout, "{}\n", serde_json::to_string(&response)?)?
                    }
                },
                Err(e) => write!(stderr, "{:?}\n", e)?,
            }
        }

        Ok(())
    }
}
