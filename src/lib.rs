#[allow(unused)]
mod irmin_api_capnp {
    include!(concat!(env!("OUT_DIR"), "/irmin_api_capnp.rs"));
}

pub struct Client {
    irmin: irmin::Client,
}

use futures::AsyncReadExt;

use futures::FutureExt;

use irmin_api_capnp::irmin;

pub type Repo = irmin_api_capnp::repo::Client;
pub type Store = irmin_api_capnp::store::Client;
pub type Commit = irmin_api_capnp::commit::Client;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O error {0}")]
    IO(#[from] std::io::Error),

    #[error("Capnp error {0}")]
    Capnp(#[from] capnp::Error),
}

impl Client {
    pub async fn new<A: tokio::net::ToSocketAddrs>(addr: A) -> Result<Client, std::io::Error> {
        let stream = tokio::net::TcpStream::connect(addr).await?;
        stream.set_nodelay(true)?;

        let (reader, writer) = tokio_util::compat::TokioAsyncReadCompatExt::compat(stream).split();
        let rpc_network = Box::new(capnp_rpc::twoparty::VatNetwork::new(
            reader,
            writer,
            capnp_rpc::rpc_twoparty_capnp::Side::Client,
            Default::default(),
        ));
        let mut rpc = capnp_rpc::RpcSystem::new(rpc_network, None);
        let irmin: irmin::Client = rpc.bootstrap(capnp_rpc::rpc_twoparty_capnp::Side::Server);

        tokio::task::spawn_local(Box::pin(rpc.map(|_| ())));

        Ok(Client { irmin })
    }

    pub async fn repo(&self) -> Result<Repo, Error> {
        let req = self.irmin.repo_request();
        let repo = req.send().promise.await?.get()?.get_repo()?;
        Ok(repo)
    }

    pub async fn ping(&self) -> Result<(), Error> {
        let req = self.irmin.ping_request();
        let _ = req.send().promise.await?.get()?;
        Ok(())
    }
}

impl Repo {
    pub async fn master(&self) -> Result<Store, Error> {
        let req = self.master_request();
        let store = req.send().promise.await?.get()?.get_store()?;
        Ok(store)
    }

    pub async fn branch(&self, name: impl AsRef<str>) -> Result<Store, Error> {
        let mut req = self.of_branch_request();
        req.get().set_branch(name.as_ref());
        let store = req.send().promise.await?.get()?.get_store()?;
        Ok(store)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
