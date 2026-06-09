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
    pub optional: bool,
}

impl Into<PluginMetadata> for VelocityPluginMetadata {
    fn into(self) -> PluginMetadata {
        let mut metadata = PluginMetadata::new(
            self.id.clone(),
            self.name.unwrap_or(self.id),
            self.version.unwrap_or("unknown".to_owned()),
        );
        if let Some(description) = self.description {
            metadata = metadata.description(description);
        }
        for author in self.authors.unwrap_or_default().iter() {
            metadata = metadata.author(author);
        }
        for dependency in self.dependencies.unwrap_or_default().into_iter() {
            if dependency.optional {
                metadata = metadata.optional_dependency(dependency.id);
            } else {
                metadata = metadata.depends_on(dependency.id);
            }
        }
        return metadata;
    }
}
