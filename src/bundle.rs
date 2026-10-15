use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use oci_spec::runtime::Spec;

/// Name of the runtime configuration file inside an OCI bundle.
pub const CONFIG_FILE: &str = "config.json";

/// An OCI bundle: a directory holding `config.json` and the container rootfs.
#[derive(Debug)]
pub struct Bundle {
    /// Absolute path to the bundle directory.
    pub path: PathBuf,
    /// The parsed runtime configuration.
    pub spec: Spec,
}

impl Bundle {
    /// Load and validate the bundle located at `path`.
    pub fn load(path: &Path) -> Result<Self> {
        let path = path
            .canonicalize()
            .with_context(|| format!("cannot resolve bundle path `{}`", path.display()))?;
        let config = path.join(CONFIG_FILE);
        let spec = Spec::load(&config)
            .with_context(|| format!("cannot load `{}`", config.display()))?;

        let bundle = Self { path, spec };
        bundle.validate()?;
        Ok(bundle)
    }

    /// Absolute path to the container root filesystem.
    ///
    /// A relative `root.path` is resolved against the bundle directory.
    pub fn rootfs(&self) -> Result<PathBuf> {
        let root = self
            .spec
            .root()
            .as_ref()
            .context("config.json has no `root` section")?;
        Ok(self.path.join(root.path()))
    }

    fn validate(&self) -> Result<()> {
        let version = self.spec.version();
        if !version.starts_with("1.") {
            bail!("unsupported OCI spec version `{version}`");
        }

        let rootfs = self.rootfs()?;
        if !rootfs.is_dir() {
            bail!("rootfs `{}` is not a directory", rootfs.display());
        }

        let process = self
            .spec
            .process()
            .as_ref()
            .context("config.json has no `process` section")?;
        if process.args().as_ref().is_none_or(|args| args.is_empty()) {
            bail!("`process.args` must not be empty");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_bundle(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!("rrun-test-{name}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(dir.join("rootfs")).unwrap();
        dir
    }

    #[test]
    fn loads_default_spec() {
        let dir = temp_bundle("default");
        Spec::default().save(dir.join(CONFIG_FILE)).unwrap();

        let bundle = Bundle::load(&dir).unwrap();
        assert_eq!(bundle.rootfs().unwrap(), dir.canonicalize().unwrap().join("rootfs"));

        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn rejects_missing_rootfs() {
        let dir = temp_bundle("norootfs");
        Spec::default().save(dir.join(CONFIG_FILE)).unwrap();
        std::fs::remove_dir(dir.join("rootfs")).unwrap();

        let err = Bundle::load(&dir).unwrap_err();
        assert!(err.to_string().contains("is not a directory"), "{err:#}");

        std::fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn rejects_missing_config() {
        let dir = temp_bundle("noconfig");
        assert!(Bundle::load(&dir).is_err());
        std::fs::remove_dir_all(&dir).unwrap();
    }
}
