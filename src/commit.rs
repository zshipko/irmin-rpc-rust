use crate::*;

pub type Commit = irmin_api_capnp::commit::Client;

impl Commit {
    pub fn tree(&self) -> Tree {
        let req = self.tree_request();
        req.send().pipeline.get_tree()
    }

    pub async fn info(&self) -> Result<Info, Error> {
        let req = self.info_request();
        let info = req.send().promise.await?;
        let info = info.get()?.get_info()?;
        let author = info.get_author()?.to_string();
        let message = info.get_message()?.to_string();
        let timestamp = info.get_date();
        Ok(Info {
            author,
            message,
            timestamp,
        })
    }

    pub async fn hash(&self) -> Result<Hash, Error> {
        let req = self.hash_request();
        let hash = req.send().promise.await?;
        let hash = hash.get()?.get_hash()?;
        Ok(hash.to_vec())
    }

    pub async fn parents(&self) -> Result<Vec<Hash>, Error> {
        let req = self.parents_request();
        let hash = req.send().promise.await?;
        let hash = hash.get()?.get_hashes()?;
        Ok(hash
            .iter()
            .filter_map(|x| match x {
                Ok(x) => Some(x.to_vec()),
                Err(_) => None,
            })
            .collect())
    }
}
