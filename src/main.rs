extern crate futures;
extern crate failure;
extern crate tokio;

use std::error::Error;

mod taxonomy;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    Ok(())
}
