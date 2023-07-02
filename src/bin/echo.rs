use gossip::{agent::EchoAgent, error::Result, Runtime};

fn main() -> Result<()> {
    let mut process = Runtime::<EchoAgent>::new();
    process.start()
}
