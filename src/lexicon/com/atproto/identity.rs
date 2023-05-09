use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ResolveHandleOutput{ 
    pub did: String
}