use infrarust_api::plugin::PluginMetadata;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct VelocityPluginMetadata {
    pub id: String,
    pub name: Option<String>,
    pub version: Option<String>,
    pub authors: Option<Vec<String>>,
    pub description: Option<String>,
    pub dependencies: Option<Vec<VelocityPluginDependency>>,
    pub main: String,
}


#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct VelocityPluginDependency {
    pub id: String,
    #[serde(default)]
    pub optional: bool
}

impl Into<PluginMetadata> for VelocityPluginMetadata {
    fn into(self) -> PluginMetadata {
        PluginMetadata {
            id: self.id.clone(),
            name: self.name.unwrap_or(self.id),
            version: self.version.unwrap_or("unknown".to_owned()),
            authors: self.authors.unwrap_or_default(),
            description: self.description,
            dependencies: Vec::new(), // TODO: Make up a way to handle velocity dependencies gracefully
        }
    }
}
