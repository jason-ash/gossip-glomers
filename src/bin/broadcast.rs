use gossip::{agent::BroadcastAgent, error::Result, Runtime};

fn main() -> Result<()> {
    let mut agent = BroadcastAgent::new();
    let mut process = Runtime::new(&mut agent);
    process.start()
}
