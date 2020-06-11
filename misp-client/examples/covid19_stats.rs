extern crate misp_client;

use chrono::{Duration, Utc};
use misp_client::*;
use std::env;

#[async_std::main]
async fn main() -> MispResult<()> {
    // Setup
    let base_url =
        env::var("MISP_ROOT_URL").expect("Please set the MISP_ROOT_URL environment variable");
    let auth_token =
        env::var("MISP_AUTH_TOKEN").expect("Please set the MISP_AUTH_TOKEN environment variable");

    let misp = MISP::new(base_url, auth_token);

    // Get Covid statistic events from the last 7 days
    let events = misp
        .events()
        .list()
        .from_organization("CIRCL") // Filter by events
        .containing_info("CSSE COVID-19 daily report")
        .after(Utc::today() - Duration::days(7))
        .retrieve()
        .await?;

    for event in events {
        //Only get event objects from Luxembourg
        for object in event.objects().iter().filter(|o| {
            o.name() == "covid19-csse-daily-report"
                && o.attribute("country-region").unwrap().value() == "Luxembourg"
        }) {
            let weekday = event.date().format("%A");
            let deaths = object.attribute("death").unwrap().value();
            println!("On {} there were {} deaths in Luxembourg ", weekday, deaths);
        }
    }
    Ok(())
}
