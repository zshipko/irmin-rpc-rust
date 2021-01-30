use futures::AsyncReadExt;
//use futures::FutureExt;

use crate::*;

pub struct Client {
    irmin: irmin::Client,
}

impl Client {
    pub async fn new<A: tokio::net::ToSocketAddrs>(
        addr: A,
    ) -> Result<(Client, tokio::task::LocalSet), Error> {
        let stream = tokio::net::TcpStream::connect(addr).await?;
        stream.set_nodelay(true)?;

        let (reader, writer) = tokio_util::compat::TokioAsyncReadCompatExt::compat(stream).split();
        let rpc_network = Box::new(capnp_rpc::twoparty::VatNetwork::new(
            reader,
            writer,
            capnp_rpc::twoparty::VatId::Client,
            Default::default(),
        ));
        let mut rpc = capnp_rpc::RpcSystem::new(rpc_network, None);
        let irmin: irmin::Client = rpc.bootstrap(capnp_rpc::twoparty::VatId::Server);

        let local = tokio::task::LocalSet::new();
        local.spawn_local(Box::pin(rpc));

        Ok((Client { irmin }, local))
    }

    pub async fn repo(&self) -> Result<Repo, Error> {
        let req = self.irmin.repo_request();
        let repo = req.send().pipeline.get_repo();
        Ok(Repo::new(repo))
    }

    pub async fn ping(&self) -> Result<(), Error> {
        let req = self.irmin.ping_request();
        let _ = req.send().promise.await?.get()?;
        Ok(())
    }
}
