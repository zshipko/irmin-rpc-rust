use crate::*;

pub type Store = irmin_api_capnp::store::Client;

pub struct Info {
    author: String,
    message: String,
    timestamp: i64,
}

impl Info {
    pub fn new(author: impl Into<String>, message: impl Into<String>) -> Result<Info, Error> {
        Ok(Info {
            author: author.into(),
            message: message.into(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs() as i64,
        })
    }
}

impl Store {
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

    pub async fn set(
        &self,
        key: impl AsRef<str>,
        value: impl AsRef<[u8]>,
        info: &Info,
    ) -> Result<(), Error> {
        let mut req = self.set_request();
        let mut r = req.get();
        r.set_key(key.as_ref());
        r.set_contents(value.as_ref());
        let mut i = r.init_info();
        i.set_author(&info.author);
        i.set_message(&info.message);
        i.set_date(info.timestamp);
        let _ = req.send().promise.await?.get()?;
        Ok(())
    }

    pub async fn remove(&self, key: impl AsRef<str>, info: &Info) -> Result<(), Error> {
        let mut req = self.remove_request();
        let mut r = req.get();
        r.set_key(key.as_ref());
        let mut i = r.init_info();
        i.set_author(&info.author);
        i.set_message(&info.message);
        i.set_date(info.timestamp);
        let _ = req.send().promise.await?.get()?;
        Ok(())
    }

    pub async fn set_tree(
        &self,
        key: impl AsRef<str>,
        tree: &Tree,
        info: &Info,
    ) -> Result<(), Error> {
        let mut req = self.set_tree_request();
        let mut r = req.get();
        r.set_key(key.as_ref());
        r.set_tree(tree.clone());
        let mut i = r.init_info();
        i.set_author(&info.author);
        i.set_message(&info.message);
        i.set_date(info.timestamp);
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

    pub async fn merge_with_branch(
        &self,
        branch: impl AsRef<str>,
        info: &Info,
    ) -> Result<(), Error> {
        let mut req = self.merge_with_branch_request();
        let mut r = req.get();
        r.set_branch(branch.as_ref());
        let mut i = r.init_info();
        i.set_author(&info.author);
        i.set_message(&info.message);
        i.set_date(info.timestamp);
        let _ = req.send().promise.await?.get()?;
        Ok(())
    }
}
