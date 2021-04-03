use super::{NoCacheStrategy, Pm, PmHelper, PromptStrategy, Strategies};
use crate::{dispatch::config::Config, error::Result, exec::Cmd};
use async_trait::async_trait;
use once_cell::sync::Lazy;

pub struct Apt {
    pub cfg: Config,
}

static PROMPT_STRAT: Lazy<Strategies> = Lazy::new(|| Strategies {
    prompt: PromptStrategy::native_prompt(&["--yes"]),
    ..Default::default()
});

static INSTALL_STRAT: Lazy<Strategies> = Lazy::new(|| Strategies {
    prompt: PromptStrategy::native_prompt(&["--yes"]),
    no_cache: NoCacheStrategy::Scc,
    ..Default::default()
});

#[async_trait]
impl Pm for Apt {
    /// Gets the name of the package manager.
    fn name(&self) -> String {
        "apt".into()
    }

    fn cfg(&self) -> &Config {
        &self.cfg
    }

    /// Q generates a list of installed packages.
    async fn q(&self, kws: &[&str], flags: &[&str]) -> Result<()> {
        self.just_run_default(Cmd::new(&["apt", "list"]).kws(kws).flags(flags))
            .await
    }

    /// Qi displays local package information: name, version, description, etc.
    async fn qi(&self, kws: &[&str], flags: &[&str]) -> Result<()> {
        self.just_run_default(Cmd::new(&["dpkg-query", "-s"]).kws(kws).flags(flags))
            .await
    }

    /// Qo queries the package which provides FILE.
    async fn qo(&self, kws: &[&str], flags: &[&str]) -> Result<()> {
        self.just_run_default(Cmd::new(&["dpkg-query", "-S"]).kws(kws).flags(flags))
            .await
    }

    /// Qp queries a package supplied on the command line rather than an entry in the package management database.
    async fn qp(&self, kws: &[&str], flags: &[&str]) -> Result<()> {
        self.just_run_default(Cmd::new(&["dpkg-deb", "-I"]).kws(kws).flags(flags))
            .await
    }

    /// Qu lists packages which have an update available.
    async fn qu(&self, kws: &[&str], flags: &[&str]) -> Result<()> {
        self.just_run_default(
            Cmd::with_sudo(&["apt", "upgrade", "--trivial-only"])
                .kws(kws)
                .flags(flags),
        )
        .await
    }

    /// R removes a single package, leaving all of its dependencies installed.
    async fn r(&self, kws: &[&str], flags: &[&str]) -> Result<()> {
        self.just_run(
            Cmd::with_sudo(&["apt", "remove"]).kws(kws).flags(flags),
            Default::default(),
            &PROMPT_STRAT,
        )
        .await
    }

    /// Rn removes a package and skips the generation of configuration backup files.
    async fn rn(&self, kws: &[&str], flags: &[&str]) -> Result<()> {
        self.just_run(
            Cmd::with_sudo(&["apt", "purge"]).kws(kws).flags(flags),
            Default::default(),
            &PROMPT_STRAT,
        )
        .await
    }

    /// Rns removes a package and its dependencies which are not required by any other installed package, and skips the generation of configuration backup files.
    async fn rns(&self, kws: &[&str], flags: &[&str]) -> Result<()> {
        self.just_run(
            Cmd::with_sudo(&["apt", "autoremove", "--purge"])
                .kws(kws)
                .flags(flags),
            Default::default(),
            &PROMPT_STRAT,
        )
        .await
    }

    /// Rs removes a package and its dependencies which are not required by any other installed package,
    /// and not explicitly installed by the user.
    async fn rs(&self, kws: &[&str], flags: &[&str]) -> Result<()> {
        self.just_run(
            Cmd::with_sudo(&["apt", "autoremove"]).kws(kws).flags(flags),
            Default::default(),
            &PROMPT_STRAT,
        )
        .await
    }

    /// S installs one or more packages by name.
    async fn s(&self, kws: &[&str], flags: &[&str]) -> Result<()> {
        self.just_run(
            Cmd::with_sudo(if self.cfg.needed {
                &["apt", "install"]
            } else {
                &["apt", "install", "--reinstall"]
            })
            .kws(kws)
            .flags(flags),
            Default::default(),
            &INSTALL_STRAT,
        )
        .await
    }

    /// Sc removes all the cached packages that are not currently installed, and the unused sync database.
    async fn sc(&self, kws: &[&str], flags: &[&str]) -> Result<()> {
        self.just_run(
            Cmd::with_sudo(&["apt", "clean"]).kws(kws).flags(flags),
            Default::default(),
            &PROMPT_STRAT,
        )
        .await
    }

    /// Scc removes all files from the cache.
    async fn scc(&self, kws: &[&str], flags: &[&str]) -> Result<()> {
        self.just_run(
            Cmd::with_sudo(&["apt", "autoclean"]).kws(kws).flags(flags),
            Default::default(),
            &PROMPT_STRAT,
        )
        .await
    }

    /// Si displays remote package information: name, version, description, etc.
    async fn si(&self, kws: &[&str], flags: &[&str]) -> Result<()> {
        self.just_run_default(Cmd::new(&["apt", "show"]).kws(kws).flags(flags))
            .await
    }

    /// Sii displays packages which require X to be installed, aka reverse dependencies.
    async fn sii(&self, kws: &[&str], flags: &[&str]) -> Result<()> {
        self.just_run_default(Cmd::new(&["apt", "rdepends"]).kws(kws).flags(flags))
            .await
    }

    /// Ss searches for package(s) by searching the expression in name, description, short description.
    async fn ss(&self, kws: &[&str], flags: &[&str]) -> Result<()> {
        self.just_run_default(Cmd::new(&["apt", "search"]).kws(kws).flags(flags))
            .await
    }

    /// Su updates outdated packages.
    async fn su(&self, kws: &[&str], flags: &[&str]) -> Result<()> {
        if kws.is_empty() {
            self.just_run(
                Cmd::with_sudo(&["apt", "upgrade"]).flags(flags),
                Default::default(),
                &PROMPT_STRAT,
            )
            .await?;
            self.just_run(
                Cmd::with_sudo(&["apt", "dist-upgrade"]).flags(flags),
                Default::default(),
                &INSTALL_STRAT,
            )
            .await
        } else {
            self.s(kws, flags).await
        }
    }

    /// Suy refreshes the local package database, then updates outdated packages.
    async fn suy(&self, kws: &[&str], flags: &[&str]) -> Result<()> {
        self.sy(kws, flags).await?;
        self.su(kws, flags).await
    }

    /// Sw retrieves all packages from the server, but does not install/upgrade anything.
    async fn sw(&self, kws: &[&str], flags: &[&str]) -> Result<()> {
        self.just_run_default(
            Cmd::with_sudo(&["apt", "install", "--download-only"])
                .kws(kws)
                .flags(flags),
        )
        .await
    }

    /// Sy refreshes the local package database.
    async fn sy(&self, kws: &[&str], flags: &[&str]) -> Result<()> {
        self.just_run_default(Cmd::with_sudo(&["apt", "update"]).kws(kws).flags(flags))
            .await?;
        if !kws.is_empty() {
            self.s(kws, flags).await?;
        }
        Ok(())
    }
}
