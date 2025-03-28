use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Cache {
    etags: HashMap<String, String>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            etags: HashMap::new(),
        }
    }

    pub fn get_etag<S>(&self, key: S) -> Option<&str>
    where
        S: AsRef<str>,
    {
        self.etags.get(key.as_ref()).map(|val| val.as_str())
    }

    pub fn etag_matches<S1, S2>(&self, key: S1, etag: S2) -> bool
    where
        S1: AsRef<str>,
        S2: AsRef<str>,
    {
        self.etags
            .get(key.as_ref())
            .is_some_and(|val| val == etag.as_ref())
    }

    pub fn set_etag<S1, S2>(&mut self, key: S1, etag: S2)
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        self.etags.insert(key.into(), etag.into());
    }
}
