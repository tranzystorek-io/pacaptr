use std::{collections::BTreeMap, ffi::OsString, fmt::Debug, fs, path::Path, str::FromStr};

use anyhow::Context;
use itertools::{chain, Itertools};
use once_cell::sync::Lazy;
use proc_macro2::{Span, TokenStream};
use regex::Regex;
use syn::{Error, Result};
use tabled::{style::Style as TableStyle, Table, Tabled};

const PM_IMPL_DIR: &str = "src/pm/";

// We have to specify the length there (the elision is blocked by https://github.com/rust-lang/rfcs/pull/2545).
// TODO: Fix this when the issue is resolved.
const METHODS: [&str; 30] = [
    "q", "qc", "qe", "qi", "qk", "ql", "qm", "qo", "qp", "qs", "qu", "r", "rn", "rns", "rs", "rss",
    "s", "sc", "scc", "sccc", "sg", "si", "sii", "sl", "ss", "su", "suy", "sw", "sy", "u",
];

/// Checks the implementation status of `pacman` commands in a specific file
/// (eg. `homebrew.rs`).
fn check_methods(file: &Path) -> anyhow::Result<BTreeMap<String, bool>> {
    let bytes = fs::read(file)?;
    let contents = String::from_utf8(bytes)?;

    METHODS
        .iter()
        .map(|&method| {
            // A function definition (rg. `rs`) is written as follows:
            // `(async) fn rs(..) {..}`
            let found = Regex::new(&format!(r#"fn\s+{}\s*\("#, method))?.is_match(&contents);
            Ok((method.to_owned(), found))
        })
        .try_collect()
}

struct CompatRow {
    fields: Vec<String>,
}

impl Tabled for CompatRow {
    const LENGTH: usize = 60;

    fn fields(&self) -> Vec<String> {
        self.fields.clone()
    }

    fn headers() -> Vec<String> {
        static HEADERS: Lazy<Vec<String>> = Lazy::new(|| {
            // `["Module", "q", "qc", "qe", ..]`
            chain!(["Module"], METHODS).map_into().collect()
        });
        HEADERS.clone()
    }
}

fn make_table() -> anyhow::Result<String> {
    let paths: Vec<fs::DirEntry> = fs::read_dir(PM_IMPL_DIR)
        .context("Failed while reading PM_IMPL_DIR")?
        .map(|entry| entry.context("Error while reading path"))
        .try_collect()?;

    let excluded_names = ["mod.rs", "unknown.rs"];
    let impls: BTreeMap<OsString, BTreeMap<String, bool>> = paths
        .iter()
        .filter(|entry| !excluded_names.iter().any(|&ex| ex == entry.file_name()))
        .map(|entry| check_methods(&entry.path()).map(|impl_| (entry.file_name(), impl_)))
        .try_collect()?;

    let make_row = |name, data| {
        let fields = chain!([name], data).map_into().collect_vec();
        CompatRow { fields }
    };

    let data: Vec<_> = impls
        .iter()
        .map(|(file, items)| {
            let data = METHODS.map(|method| {
                items
                    .get(method)
                    .expect("Implementation details not registered")
                    .then(|| "*")
                    .unwrap_or("")
            });
            file.to_str()
                .context("Failed to convert `file: OsString` to `&str`")
                .map(|file| make_row(file, data))
        })
        .try_collect()?;

    let table = Table::new(&data).with(TableStyle::blank());
    Ok(format!("```\n{table}```\n"))
}

pub(crate) fn compat_table_impl() -> Result<TokenStream> {
    fn throw(e: &dyn Debug) -> Error {
        let msg = format!("{e:?}");
        Error::new(Span::call_site(), msg)
    }

    let table = make_table().map_err(|e| throw(&e))?;
    let docstring = format!(r##"r#"{table}"#"##);
    Ok(TokenStream::from_str(&docstring)?)
}
