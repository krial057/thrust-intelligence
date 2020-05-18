extern crate rs_misp;

use chrono::{Duration, Utc};
use rs_misp::*;
use std::env;

#[async_std::main]
async fn main() -> rs_misp::MispResult<()> {
    femme::start(log::LevelFilter::Info).unwrap_or_else(|e| println!("Coudln't set logger: {}", e));
    let base_url =
        env::var("MISP_ROOT_URL").expect("Please set the MISP_ROOT_URL environment variable");
    let auth_token =
        env::var("MISP_AUTH_TOKEN").expect("Please set the MISP_AUTH_TOKEN environment variable");

    let misp = MISP::new(base_url, auth_token);
    let events = misp
        .events()
        .list()
        .from_organization("CIRCL")
        .containing_info("CSSE COVID-19 daily report")
        .after(Utc::today() - Duration::days(3))
        //.limit(1)
        .retrieve()
        .await?;

    for event in events {
        println!("{}", event.info());
    }
    Ok(())
}
