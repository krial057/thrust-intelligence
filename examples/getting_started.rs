extern crate rs_misp;

use rs_misp::model::event::EventFull;
use rs_misp::*;
use std::env;

/*
pub async fn get_a_single_event(misp: &MISP) {
    let event = misp
        .event(1188)
        .await
        .map_err(|e| println!("Coudln't fetch event: {}", e));
}
*/

#[async_std::main]
async fn main() -> rs_misp::MispResult<()> {
    femme::start(log::LevelFilter::Info).unwrap_or_else(|e| println!("Coudln't set logger: {}", e));
    let base_url =
        env::var("MISP_ROOT_URL").expect("Please set the MISP_ROOT_URL environment variable");
    let auth_token =
        env::var("MISP_AUTH_TOKEN").expect("Please set the MISP_AUTH_TOKEN environment variable");

    let misp = MISP::new(base_url, auth_token);
    let test = misp.event(1188);
    let id1 = test.uuid().await?;
    let id2 = test.uuid().await?;
    //let id = misp.event(1188).uuid().await?;
    println!("{} {}", id1, id2);
    Ok(())
}
