//! Esplora API
//!
//! Author: Vincenzo Palazzo <vincenzopalazzo@member.fsf.org>
use curl::easy::Easy;
use curl::Error;
use serde_json as json;

type Result<T> = core::result::Result<T, Error>;

pub struct EsploraAPI {
    base_url: String,
}

impl EsploraAPI {
    /// Create a new instance of the esplora API client.
    pub fn new(url: &str) -> Result<Self> {
        Ok(Self {
            base_url: url.to_owned(),
        })
    }

    pub fn client(&self, addons: &str) -> Result<Easy> {
        let mut easy = Easy::new();
        easy.url(&format!("{}/{addons}", self.base_url))?;
        Ok(easy)
    }

    pub fn raw_post(&self, addons: &str, body: &[u8]) -> Result<Vec<u8>> {
        let mut easy = self.client(addons)?;
        easy.post(true)?;
        easy.post_fields_copy(body)?;

        let mut body = Vec::new();
        {
            let mut transfer = easy.transfer();
            transfer.write_function(|data| {
                body.extend_from_slice(data);
                Ok(data.len())
            })?;

            transfer.perform()?;
        }
        Ok(body)
    }

    pub fn raw_call(&self, addons: &str) -> Result<Vec<u8>> {
        let mut easy = self.client(addons)?;
        let mut body = Vec::new();
        {
            let mut transfer = easy.transfer();
            transfer.write_function(|data| {
                body.extend_from_slice(data);
                Ok(data.len())
            })?;

            transfer.perform()?;
        }
        Ok(body)
    }

    // perform a generic call
    pub fn call<D: serde::de::DeserializeOwned>(&self, addons: &str) -> Result<D> {
        let body = self.raw_call(addons)?;
        let parsed_json: D = json::from_slice(&body).map_err(|e| {
            let mut err = Error::new(400);
            err.set_extra(format!("{e}"));
            err
        })?;
        Ok(parsed_json)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tip() -> Result<()> {
        let api = EsploraAPI::new("https://blockstream.info/api")?;
        let hash = api.raw_call("blocks/tip/hash")?;
        let hash = String::from_utf8(hash).unwrap();
        assert_eq!(
            hash,
            "0000000000000000000099819a9e23a5068a2a6f0e842e4f9568f53ede446300"
        );
        Ok(())
    }
}
