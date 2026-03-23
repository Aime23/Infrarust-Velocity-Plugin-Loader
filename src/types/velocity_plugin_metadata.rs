use infrarust_api::plugin::PluginMetadata;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct VelocityPluginMetadata {
    pub id: String,
    pub name: String,
    pub version: String,
    pub authors: Vec<String>,
    pub description: Option<String>,
    pub dependencies: Vec<String>,
    pub main: String,
}


impl Into<PluginMetadata> for VelocityPluginMetadata {
    fn into(self) -> PluginMetadata {
        PluginMetadata {
            id: self.id,
            name: self.name,
            version: self.version,
            authors: self.authors,
            description: self.description,
            dependencies: Vec::new(), // TODO: Make up a way to handle velocity dependencies gracefully
        }
    }
}
