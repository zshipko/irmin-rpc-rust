use crate::*;

pub type Repo = irmin_api_capnp::repo::Client;

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

