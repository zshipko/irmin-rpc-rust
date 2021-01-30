#[allow(unused)]
pub(crate) mod irmin_api_capnp {
    include!(concat!(env!("OUT_DIR"), "/irmin_api_capnp.rs"));
}

mod client;
mod commit;
mod error;
mod repo;
mod repr;
mod store;
mod tree;

pub(crate) use irmin_api_capnp::irmin;

pub use client::Client;
pub use commit::Commit;
pub use error::Error;
pub use repo::Repo;
pub use store::{ContentsHash, Info, Store};
pub use tree::{ConcreteTree, Tree};

pub type Contents = Vec<u8>;

pub type Hash = Vec<u8>;

#[cfg(test)]
mod tests {}
