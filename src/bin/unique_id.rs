use gossip::{agent::UniqueIdAgent, error::Result, Runtime};

fn main() -> Result<()> {
    let mut process = Runtime::<UniqueIdAgent>::new();
    process.start()
}
