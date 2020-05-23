extern crate rs_misp;

use rs_misp::*;
use std::env;

#[async_std::main]
async fn main() -> MispResult<()> {
    let base_url =
        env::var("MISP_ROOT_URL").expect("Please set the MISP_ROOT_URL environment variable");
    let auth_token =
        env::var("MISP_AUTH_TOKEN").expect("Please set the MISP_AUTH_TOKEN environment variable");

    let misp = MISP::new(base_url, auth_token);
    let uuid = misp.events().get(1188).uuid().await?;
    println!("Uuid: {}", uuid);
    Ok(())
}
