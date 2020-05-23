# rs_misp
[![LICENSE](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE.md)
[![Released API docs](https://docs.rs/rs_misp/badge.svg)](https://docs.rs/rs_misp)
[![Crates.io Version](https://img.shields.io/crates/v/rs_misp.svg)](https://crates.io/crates/rs_misp)
[![CI](https://github.com/krial057/rs_misp/workflows/CI/badge.svg)](https://github.com/krial057/rs_misp/actions?query=workflow%3ACI)

 `rs_misp` is an unofficial client library to communicate with a [MISP](https://www.misp-project.org/)
 server instance.
 Its focus is to be easy to use, performant and strongly typed.
 
 __This library is far from production-ready! Currently, it can almost only run the simple example provided below.__
 
 *This project is unofficial and not associated with the [MISP project](https://www.misp-project.org/).*
 ## Example
 A simple example fetching covid deaths in Luxembourg from the past 7 days using the
 COVID-19 MISP
 ```rust
 use chrono::{Duration, Utc};
 use rs_misp::*;
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
         // Add some filters
         .from_organization("CIRCL")
         .containing_info("CSSE COVID-19 daily report")
         .after(Utc::today() - Duration::days(7))
         // Download
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
 ```
 Will result in the following output
 ```text
 On Saturday there were 104 deaths in Luxembourg
 On Sunday there were 107 deaths in Luxembourg
 On Monday there were 107 deaths in Luxembourg
 On Tuesday there were 109 deaths in Luxembourg
 On Wednesday there were 109 deaths in Luxembourg
 On Thursday there were 109 deaths in Luxembourg
 On Friday there were 109 deaths in Luxembourg
 ```

## Roadmap
- [x] Design library structure 
- [x] Add passive functionallity (get) for events and attributes
- [ ] Documentation & tests
- [ ] Add more search filters 
- [ ] Add other objects (galaxies, tags, ...)
- [ ] Make active functionality working
- [ ] Evaluate

