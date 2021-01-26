use crate::*;

pub enum ConcreteTree {
    Contents(Hash),
    Node(String, Box<ConcreteTree>),
}

pub type Tree = irmin_api_capnp::tree::Client;

impl Tree {
    pub async fn find(&self, key: impl AsRef<str>) -> Result<Option<Contents>, Error> {
        let mut req = self.find_request();
        req.get().set_key(key.as_ref());
        let x = req.send().promise.await?;
        let r = x.get()?;
        if !r.has_contents() {
            return Ok(None);
        }
        let contents = x.get()?.get_contents()?;
        Ok(Some(contents.to_vec()))
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

    pub async fn add(&self, key: impl AsRef<str>, value: impl AsRef<[u8]>) -> Result<(), Error> {
        let mut req = self.add_request();
        let mut r = req.get();
        r.set_key(key.as_ref());
        r.set_contents(value.as_ref());
        let _ = req.send().promise.await?.get()?;
        Ok(())
    }

    pub async fn set_tree(&self, key: impl AsRef<str>, tree: &Tree) -> Result<(), Error> {
        let mut req = self.add_tree_request();
        let mut r = req.get();
        r.set_key(key.as_ref());
        r.set_tree(tree.clone());
        let _ = req.send().promise.await?.get()?;
        Ok(())
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

    pub async fn find_hash(&self, key: impl AsRef<str>) -> Result<Option<Hash>, Error> {
        let mut req = self.find_hash_request();
        req.get().set_key(key.as_ref());
        let x = req.send().promise.await?;
        let r = x.get()?;
        if !r.has_hash() {
            return Ok(None);
        }
        let contents = x.get()?.get_hash()?;
        Ok(Some(contents.to_vec()))
    }

    pub async fn remove(&self, key: impl AsRef<str>) -> Result<(), Error> {
        let mut req = self.remove_request();
        let mut r = req.get();
        r.set_key(key.as_ref());
        let _ = req.send().promise.await?.get()?;
        Ok(())
    }
}
