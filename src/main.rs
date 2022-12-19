use std::time::Duration;

use clap::Parser;

mod cli;
mod sources;
mod train;

#[tokio::main]
pub async fn main() {
    // Get CLI args
    let args = cli::Args::parse();

    // Get all train locations
    println!("Fetching train locations");
    let trains = sources::get_trains().await.unwrap();
    println!("Found {} trains", trains.len());

    // Set up a request client
    let client = reqwest::Client::new();

    // For each train, construct, log, and send its position report
    for train in trains {
        // Crude packet creation
        let packet = format!(
            "{}>APRS,qAS:;{}*111111z{}={}",
            args.callsign,
            train.identifier_padded(),
            train.location_ddm(),
            train.status
        );
        println!("{}", packet);

        // Push the packet to the server
        if !args.dry_run {
            let response = client
                .post("http://rotate.aprs.net:8080/")
                .header("Accept-Type", "text/plain")
                .header("Content-Type", "application/octet-stream")
                .body(format!(
                    "user {} pass {} vers aprs-trains 0.1.0\n{}",
                    args.callsign, args.password, packet
                ))
                .timeout(Duration::from_secs(3))
                .send()
                .await;

            if let Ok(response) = response {
                println!("<- {}", response.status());
            } else {
                let err = response.unwrap_err();
                if err.is_timeout() {
                    println!("<- TIMED OUT");
                } else {
                    println!("<- ERR: {:?}", err);
                }
            }
        }
    }
}
