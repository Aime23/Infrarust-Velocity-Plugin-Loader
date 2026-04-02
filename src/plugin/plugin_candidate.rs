use std::{
    fs::File,
    path::{Path, PathBuf},
};

use zip::ZipArchive;

use crate::types::velocity_plugin_metadata::VelocityPluginMetadata;

#[derive(Debug)]
pub enum PluginCandidateLoadingError {
    FileDoesNotExist,
    NotAFile, // When passed a directory
    UnableToOpen,
    NotAJar,
    NotAVelocityPlugin, // When no velocity-plugin.json is found
}
// This represents a plugin candidate; it needs to be built from a source file.
// The source file must be a valid jar and contain a valid `velocity-plugin.json`.
// It does not guarantee that the candidate will be loaded successfully.
#[derive(Debug, Clone)]
pub struct PluginCandidate {
    metadata: VelocityPluginMetadata,
    path: PathBuf,
}

impl PluginCandidate {
    pub fn metadata(&self) -> &VelocityPluginMetadata {
        return &self.metadata;
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}

impl TryFrom<&Path> for PluginCandidate {
    type Error = PluginCandidateLoadingError;

    fn try_from(value: &Path) -> Result<Self, Self::Error> {
        if !value.exists() {
            return Err(Self::Error::FileDoesNotExist);
        }

        if !value.is_file() {
            return Err(Self::Error::NotAFile);
        }

        let file = File::open(value).map_err(|_| Self::Error::UnableToOpen)?;
        let mut archive = ZipArchive::new(file).map_err(|_| Self::Error::NotAJar)?;

        let plugin_metadata = archive
            .by_name("velocity-plugin.json")
            .map_err(|_| Self::Error::NotAVelocityPlugin)?;

        let plugin_metadata: VelocityPluginMetadata = serde_json::from_reader(plugin_metadata)
            .map_err(|_| Self::Error::NotAVelocityPlugin)?;

        return Ok(Self {
            metadata: plugin_metadata,
            path: value.to_owned(),
        });
    }
}

#[cfg(test)]
mod tests {
    use crate::types::velocity_plugin_metadata::VelocityPluginDependency;

    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_successful_jar_loading() {
        let test_jar_path = PathBuf::from("tests/resources/test_plugin.jar");
        let plugin_candidate = PluginCandidate::try_from(test_jar_path.as_path()).unwrap();

        assert_eq!(plugin_candidate.metadata.id, "infrarust_test_plugin");
        assert_eq!(plugin_candidate.metadata.name, Some("test_plugin".to_owned()));
        assert_eq!(plugin_candidate.metadata.version, Some("1.0-SNAPSHOT".to_owned()));
        assert_eq!(
            plugin_candidate.metadata.authors,
            Some(Vec::<String>::new())
        );
        assert_eq!(
            plugin_candidate.metadata.dependencies,
            Some(Vec::<VelocityPluginDependency>::new())
        );
        assert_eq!(
            plugin_candidate.metadata.main,
            "dev.infrarust.test_plugin.TestPlugin"
        );
    }

    #[test]
    fn test_file_does_not_exist() {
        let non_existent_path = PathBuf::from("tests/resources/nonexistent.jar");
        let result = PluginCandidate::try_from(non_existent_path.as_path());

        assert!(matches!(
            result,
            Err(PluginCandidateLoadingError::FileDoesNotExist)
        ));
    }

    #[test]
    fn test_not_a_file() {
        let directory_path = PathBuf::from("tests/resources/test_directory");
        let result = PluginCandidate::try_from(directory_path.as_path());

        assert!(matches!(result, Err(PluginCandidateLoadingError::NotAFile)));
    }

    #[test]
    fn test_not_a_jar() {
        let invalid_jar_path = PathBuf::from("tests/resources/invalid_jar.jar");
        let result = PluginCandidate::try_from(invalid_jar_path.as_path());

        assert!(matches!(result, Err(PluginCandidateLoadingError::NotAJar)));
    }

    #[test]
    fn test_not_a_velocity_plugin() {
        let jar_without_metadata_path = PathBuf::from("tests/resources/jar_without_metadata.jar");
        let result = PluginCandidate::try_from(jar_without_metadata_path.as_path());

        assert!(matches!(
            result,
            Err(PluginCandidateLoadingError::NotAVelocityPlugin)
        ));
    }
}
