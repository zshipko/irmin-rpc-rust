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
}
