use crate::*;

pub enum ConcreteTree {
    Contents(Hash),
    Node(String, Box<ConcreteTree>),
}

pub type Tree = irmin_api_capnp::tree::Client;

impl Tree {
    pub async fn find(&self, key: impl AsRef<str>) -> Result<Option<ContentsHash>, Error> {
        let mut req = self.find_hash_request();
        req.get().set_key(key.as_ref());
        let x = req.send().promise.await?;
        let r = x.get()?;
        if !r.has_hash() {
            return Ok(None);
        }
        let hash = x.get()?.get_hash()?;
        Ok(Some(ContentsHash::new(hash.to_vec())))
    }

    pub async fn mem_tree(&self, key: impl AsRef<str>) -> Result<bool, Error> {
        let mut req = self.mem_tree_request();
        req.get().set_key(key.as_ref());
        let exists = req.send().promise.await?.get()?.get_exists();
        Ok(exists)
    }

    pub async fn mem(&self, key: impl AsRef<str>) -> Result<bool, Error> {
        let mut req = self.mem_request();
        req.get().set_key(key.as_ref());
        let exists = req.send().promise.await?.get()?.get_exists();
        Ok(exists)
    }

    pub fn add(&self, key: impl AsRef<str>, value: impl AsRef<[u8]>) -> Tree {
        let mut req = self.add_request();
        let mut r = req.get();
        r.set_key(key.as_ref());
        r.set_contents(value.as_ref());
        req.send().pipeline.get_tree()
    }

    pub fn set_tree(&self, key: impl AsRef<str>, tree: &Tree) -> Tree {
        let mut req = self.add_tree_request();
        let mut r = req.get();
        r.set_key(key.as_ref());
        r.set_tree(tree.clone());
        req.send().pipeline.get_tree()
    }

    pub async fn find_tree(&self, key: impl AsRef<str>) -> Result<Option<Tree>, Error> {
        let mut req = self.get_tree_request();
        req.get().set_key(key.as_ref());
        let x = req.send().promise.await?;
        let r = x.get()?;
        match r.get_tree() {
            Ok(x) => Ok(Some(x)),
            Err(e) => Err(e.into()),
        }
    }

    pub async fn remove(&self, key: impl AsRef<str>) -> Tree {
        let mut req = self.remove_request();
        let mut r = req.get();
        r.set_key(key.as_ref());
        req.send().pipeline.get_tree()
    }
}
