#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NodeId {
    Instance(Url),
}
