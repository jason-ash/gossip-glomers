use gossip::{agent::UniqueIdAgent, error::Result, Runtime};

fn main() -> Result<()> {
    let mut agent = UniqueIdAgent::new();
    let mut process = Runtime::new(&mut agent);
    process.start()
}
