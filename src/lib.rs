#[allow(unused)]
mod irmin_api_capnp {
    include!(concat!(env!("OUT_DIR"), "/irmin_api_capnp.rs"));
}

mod client;
mod error;
mod repo;
mod store;

pub(crate) use irmin_api_capnp::irmin;

pub use client::Client;
pub use error::Error;
pub use repo::Repo;
pub use store::{Info, Store};

pub type Commit = irmin_api_capnp::commit::Client;

pub type Contents = Vec<u8>;

#[cfg(test)]
mod tests {}
