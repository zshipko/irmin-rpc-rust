use crate::*;

pub type Repo = irmin_api_capnp::repo::Client;

impl Repo {
    pub async fn master(&self) -> Result<Store, Error> {
        let req = self.master_request();
        let store = req.send().pipeline.get_store();
        Ok(store)
    }

    pub async fn branch(&self, name: impl AsRef<str>) -> Result<Store, Error> {
        let mut req = self.of_branch_request();
        req.get().set_branch(name.as_ref());
        let store = req.send().pipeline.get_store();
        Ok(store)
    }

    pub async fn contents_of_hash(&self, hash: &Hash) -> Result<Option<Contents>, Error> {
        let mut req = self.contents_of_hash_request();
        req.get().set_hash(hash);
        let tmp = req.send().promise.await?;
        if !tmp.get()?.has_contents() {
            return Ok(None);
        }
        let contents = tmp.get()?.get_contents()?;
        Ok(Some(contents.to_vec()))
    }

    pub async fn commit_of_hash(&self, hash: &Hash) -> Result<Option<Commit>, Error> {
        let mut req = self.commit_of_hash_request();
        req.get().set_hash(hash);
        let tmp = req.send().promise.await?;
        let commit = tmp.get()?.get_commit()?;
        Ok(Some(commit))
    }
}
