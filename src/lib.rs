#![doc(html_playground_url = "https://play.rust-lang.org/")]

//! `rs_misp` is a client library to communicate with a [MISP](https://www.misp-project.org/)
//! server instance.
//! Its focus is to be easy to use, performant and typed.
//!
//! # Example
//! A simple example fetching covid deaths in Luxembourg from the past 7 days using the
//! COVID-19 MISP
//! ```
//! use chrono::{Duration, Utc};
//! use rs_misp::*;
//! use std::env;
//!
//! #[async_std::main]
//! async fn main() -> MispResult<()> {
//!     // Setup
//!     let base_url =
//!         env::var("MISP_ROOT_URL").expect("Please set the MISP_ROOT_URL environment variable");
//!     let auth_token =
//!         env::var("MISP_AUTH_TOKEN").expect("Please set the MISP_AUTH_TOKEN environment variable");
//!
//!     let misp = MISP::new(base_url, auth_token);
//!
//!     // Get Covid statistic events from the last 7 days
//!     let events = misp
//!         .events()
//!         .list()
//!         // Add some filters
//!         .from_organization("CIRCL")
//!         .containing_info("CSSE COVID-19 daily report")
//!         .after(Utc::today() - Duration::days(7))
//!         // Download
//!         .retrieve()
//!         .await?;
//!
//!     for event in events {
//!         //Only get event objects from Luxembourg
//!         for object in event.objects().iter().filter(|o| {
//!             o.name() == "covid19-csse-daily-report"
//!                 && o.attribute("country-region").unwrap().value() == "Luxembourg"
//!         }) {
//!             let weekday = event.date().format("%A");
//!             let deaths = object.attribute("death").unwrap().value();
//!             println!("On {} there were {} deaths in Luxembourg ", weekday, deaths);
//!         }
//!     }
//!     Ok(())
//! }
//! ```
//! Will result in the following output
//! ```text
//! On Saturday there were 104 deaths in Luxembourg
//! On Sunday there were 107 deaths in Luxembourg
//! On Monday there were 107 deaths in Luxembourg
//! On Tuesday there were 109 deaths in Luxembourg
//! On Wednesday there were 109 deaths in Luxembourg
//! On Thursday there were 109 deaths in Luxembourg
//! On Friday there were 109 deaths in Luxembourg
//! ```

mod client;
mod error;
pub mod model;
pub mod requests;

pub use client::MISP;
pub use error::{MispError, MispResult};

#[cfg(test)]
mod tests {}
