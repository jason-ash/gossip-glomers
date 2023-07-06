use gossip::{agent::EchoAgent, error::Result, Runtime};

fn main() -> Result<()> {
    let mut agent = EchoAgent::new();
    let mut process = Runtime::new(&mut agent);
    process.start()
}
