use covid_tracker::run;
use std::{error::Error, process};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // let args = Arguments::parse();
    if let Err(_) = run().await {
            println!("Was not able to get data from worldometers, please try again later.");
            process::exit(1);
    };

    Ok(())
}
