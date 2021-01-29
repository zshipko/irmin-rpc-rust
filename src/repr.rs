use crate::*;

pub trait Type {
    type Output: serde::de::DeserializeOwned;

    fn encode(&self) -> Result<Contents, Error>;
    fn decode(contents: &Contents) -> Result<Self::Output, Error>;
}

impl<T: serde::Serialize + serde::de::DeserializeOwned> Type for T {
    type Output = T;

    fn encode(&self) -> Result<Contents, Error> {
        let x = serde_json::to_vec(self)?;
        Ok(x)
    }

    fn decode(contents: &Contents) -> Result<Self::Output, Error> {
        let x = serde_json::from_slice(contents.as_ref())?;
        Ok(x)
    }
}
