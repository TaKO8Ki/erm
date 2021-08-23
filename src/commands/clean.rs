use crate::outln;
use colored::Colorize;
use std::fs;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FrumError {}

pub struct Clean {}

impl crate::command::Command for Clean {
    type Error = FrumError;

    fn apply(&self, config: &crate::config::FrumConfig) -> Result<(), Self::Error> {
        let temp_installations_dir = config.temp_installations_dir();
        outln!(config#Info, "{} Removing old downloads", "==>".green());
        fs::remove_dir_all(&temp_installations_dir).expect("Couldn't remove downloads");
        fs::create_dir(&temp_installations_dir)
            .expect("Couldn't create temporary installation directory");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command::Command;
    use crate::config::FrumConfig;
    use tempfile::tempdir;

    #[test]
    fn test_remove_downloads() {
        let config = FrumConfig {
            base_dir: Some(tempdir().unwrap().path().to_path_buf()),
            ..Default::default()
        };

        let temp_installations_dir = config.temp_installations_dir();
        fs::create_dir_all(&temp_installations_dir)
            .expect("Can't generate temporary installation directory");
        tempfile::TempDir::new_in(&temp_installations_dir)
            .expect("Can't generate temporary directory");

        Clean {}.apply(&config).unwrap();

        assert!(temp_installations_dir.read_dir().unwrap().next().is_none());
    }
}
