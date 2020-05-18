mod client;
mod error;
pub mod model;
pub mod requests;

#[macro_use]
mod util;

pub use client::MISP;
pub use error::{MispError, MispResult};

#[cfg(test)]
mod tests {
    use super::*;
    use async_std::task;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
