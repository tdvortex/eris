#[warn(missing_docs)]

use serde::Serialize;


#[derive(Serialize)]
pub struct CacheKey<K: Serialize>(K);