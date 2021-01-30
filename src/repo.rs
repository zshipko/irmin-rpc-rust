use crate::*;

pub struct Repo {
    client: irmin_api_capnp::repo::Client,
    contents_cache: std::cell::RefCell<lru::LruCache<Hash, Contents>>,
}

impl Repo {
    pub(crate) fn new(client: irmin_api_capnp::repo::Client) -> Self {
        Repo {
            client,
            contents_cache: std::cell::RefCell::new(lru::LruCache::new(32)),
        }
    }

    pub async fn master(&self) -> Result<Store, Error> {
        let req = self.client.master_request();
        let store = req.send().pipeline.get_store();
        Ok(store)
    }

    pub async fn branch(&self, name: impl AsRef<str>) -> Result<Store, Error> {
        let mut req = self.client.of_branch_request();
        req.get().set_branch(name.as_ref());
        let store = req.send().pipeline.get_store();
        Ok(store)
    }

    pub async fn contents_of_hash(&self, hash: &Hash) -> Result<Option<Contents>, Error> {
        if let Some(x) = self.contents_cache.borrow_mut().get(hash) {
            return Ok(Some(x.clone()));
        }

        let mut req = self.client.contents_of_hash_request();
        req.get().set_hash(hash);
        let tmp = req.send().promise.await?;
        if !tmp.get()?.has_contents() {
            return Ok(None);
        }
        let contents = tmp.get()?.get_contents()?;
        let c = contents.to_vec();
        self.contents_cache
            .borrow_mut()
            .put(hash.clone(), c.clone());
        Ok(Some(c))
    }

    pub fn commit_of_hash(&self, hash: &Hash) -> Commit {
        let mut req = self.client.commit_of_hash_request();
        req.get().set_hash(hash);
        let tmp = req.send().pipeline;
        tmp.get_commit()
    }

    pub fn empty_tree(&self) -> Tree {
        let req = self.client.empty_tree_request();
        let tmp = req.send().pipeline;
        tmp.get_tree()
    }
}
