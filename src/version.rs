use std::str::FromStr;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone)]
pub enum Version {
    Semver(semver::Version),
    Alias(String),
}

fn start_with_number(s: &str) -> bool {
    s.chars().next().map(|x| x.is_digit(10)).unwrap_or(false)
}

impl Version {
    pub fn parse<S: AsRef<str>>(version_str: S) -> Result<Self, semver::SemVerError> {
        let lowercased = version_str.as_ref().to_lowercase();
        if start_with_number(lowercased.trim_start_matches('v')) {
            let version_plain = lowercased.trim_start_matches('v');
            let sver = semver::Version::parse(&version_plain)?;
            Ok(Self::Semver(sver))
        } else {
            Ok(Self::Alias(lowercased))
        }
    }

    pub fn alias_name(&self) -> Option<String> {
        match self {
            l @ Self::Alias(_) => Some(l.v_str()),
            _ => None,
        }
    }

    pub fn find_aliases(
        &self,
        config: &crate::config::FarmConfig,
    ) -> std::io::Result<Vec<crate::alias::StoredAlias>> {
        let aliases = crate::alias::list_aliases(&config)?
            .drain(..)
            .filter(|alias| alias.s_ver() == self.v_str())
            .collect();
        Ok(aliases)
    }

    pub fn v_str(&self) -> String {
        format!("{}", self)
    }

    pub fn installation_path(
        &self,
        config: &crate::config::FarmConfig,
    ) -> Option<std::path::PathBuf> {
        match self {
            v @ Self::Alias(_) => Some(config.aliases_dir().join(v.alias_name().unwrap())),
            v @ Self::Semver(_) => Some(config.versions_dir().join(v.v_str()).join("installation")),
        }
    }

    pub fn root_path(&self, config: &crate::config::FarmConfig) -> Option<std::path::PathBuf> {
        match self.installation_path(&config) {
            None => None,
            Some(path) => {
                let mut canon_path = path.canonicalize().ok()?;
                canon_path.pop();
                Some(canon_path)
            }
        }
    }
}

impl<'de> serde::Deserialize<'de> for Version {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let version_str = String::deserialize(deserializer)?;
        Version::parse(version_str).map_err(serde::de::Error::custom)
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Semver(semver) => write!(f, "v{}", semver),
            Self::Alias(alias) => write!(f, "{}", alias),
        }
    }
}

impl FromStr for Version {
    type Err = semver::SemVerError;
    fn from_str(s: &str) -> Result<Version, Self::Err> {
        Self::parse(s)
    }
}

impl PartialEq<semver::Version> for Version {
    fn eq(&self, other: &semver::Version) -> bool {
        match self {
            Self::Semver(v) => v == other,
            Self::Alias(_) => false,
        }
    }
}
