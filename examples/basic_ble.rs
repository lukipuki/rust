/// This example connects via Bluetooth LE to the radio, and prints out all received packets.
extern crate meshtastic;

use std::io::{self, BufRead};

use meshtastic::api::StreamApi;
use meshtastic::utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stream_api = StreamApi::new();

    println!("Enter the short name of a BLE device to connect to:");

    let stdin = io::stdin();
    let entered_name = stdin
        .lock()
        .lines()
        .next()
        .expect("Failed to find next line")
        .expect("Could not read next line");

    let ble_stream = utils::stream::build_ble_stream(entered_name).await?;
    let (mut decoded_listener, stream_api) = stream_api.connect(ble_stream).await;

    let config_id = utils::generate_rand_id();
    let stream_api = stream_api.configure(config_id).await?;

    // This loop can be broken with ctrl+c, disabling bluetooth or by turning off the radio.
    while let Some(decoded) = decoded_listener.recv().await {
        println!("Received: {:?}", decoded);
    }

    // Note that in this specific example, this will only be called when
    // the radio is disconnected, as the above loop will never exit.
    // Typically you would allow the user to manually kill the loop,
    // for example with tokio::select!.
    let _stream_api = stream_api.disconnect().await?;

    Ok(())
}
