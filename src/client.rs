use futures::AsyncReadExt;
use futures::FutureExt;

use crate::*;

pub struct Client {
    irmin: irmin::Client,
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

