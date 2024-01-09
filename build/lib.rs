mod pwa;
use pwa::is_pwa;

use anyhow::Result;
use std::{
    env, format as f,
    fs::{read_to_string, rename, write},
    process::Command,
    time::Instant,
};
use webmanifest::{DisplayMode, Icon, Manifest};

#[cfg(feature = "typescript")]
mod typescript;
#[cfg(feature = "typescript")]
pub use typescript::*;

#[cfg(feature = "sass")]
mod sass {
    use std::{path::Path, fs::write};
    pub fn bundle_sass(path: &str) {
        let css = grass::from_path(path, &Default::default())?;
        let scss_filename = Path::new(path).file_name()?.to_str()?;
        let css_filename = scss_filename.replace(".scss", ".css").replace(".sass", ".css");
        let out_file = super::out_path(&css_filename);
        write(out_file, css)?;
    }
}
#[cfg(feature = "sass")]
pub use sass::bundle_sass;

pub struct PWAOptions<'a> {
    pub listeners: Vec<(&'a str, &'a str)>,
    pub name: String,
    pub desc: String,
    pub background: String,
    pub theme: String,
    pub start: String,
    pub display: DisplayMode,
    pub icons: Vec<Icon<'a>>,
}
impl Default for PWAOptions<'_> {
    fn default() -> Self {
        Self {
            listeners: vec![
                (
                    "install",
                    "event.waitUntil(Promise.all([__wbg_init('/sw.wasm'), self.skipWaiting()]))",
                ),
                ("activate", "event.waitUntil(self.clients.claim())"),
                ("fetch", "handle_fetch(self, event)"),
            ],
            name: std::env::var("CARGO_PKG_NAME").unwrap(),
            desc: if let Ok(desc) = std::env::var("CARGO_PKG_DESCRIPTION") {
                desc
            } else {
                "An installable web application".to_owned()
            },
            background: "#1e293b".to_owned(),
            theme: "#a21caf".to_owned(),
            start: "/".to_owned(),
            display: DisplayMode::Standalone,
            icons: vec![Icon::new("logo.png", "512x512")],
        }
    }
}

const SW_TARGET: &str = "service-worker";

static LOGO: &[u8] = include_bytes!("default-logo.png");

static LISTENER_TEMPLATE: &str = "self.addEventListener('NAME', event => LISTENER);\n";

pub fn build_pwa(opts: PWAOptions) -> Result<()> {
    if env::var("SELF_PWA_BUILD").is_ok() || !is_pwa() {
        return Ok(());
    }
    let start = Instant::now();
    let lib_name = &read_lib_name()?;
    let target_dir = sw_target_dir();
    let profile_dir = match cfg!(debug_assertions) {
        true => "debug",
        false => "release",
    };
    let profile_path = &f!("{target_dir}/wasm32-unknown-unknown/{profile_dir}");
    let lib_path = &f!("{profile_path}/{lib_name}");

    // build in a separate target dir to avoid build deadlock with the host
    let mut cmd = Command::new("cargo");
    cmd.env("SELF_PWA_BUILD", "true")
        .arg("rustc")
        .arg("--lib")
        .args(["--crate-type", "cdylib"])
        //.args(["--features", "traces html embed"])
        .args(["--target", "wasm32-unknown-unknown"])
        .args(["--target-dir", &target_dir]);
    
    if !cfg!(debug_assertions) {
        cmd.arg("--release");
    }
    
    assert!(cmd.status()?.success());

    // generate bindings for the wasm binary
    wasm_bindgen_cli_support::Bindgen::new()
        .input_path(f!("{lib_path}.wasm"))
        .web(true)?
        .remove_name_section(cfg!(not(debug_assertions)))
        .remove_producers_section(cfg!(not(debug_assertions)))
        .keep_debug(cfg!(debug_assertions))
        .omit_default_module_path(true)
        .generate(profile_path)?;

    // move the processed wasm binary into final dist
    rename(&f!("{lib_path}_bg.wasm"), out_path("sw.wasm"))?;

    // append event listeners and save js bindings
    let mut js = read_to_string(&f!("{lib_path}.js"))?;
    for listener in opts.listeners.iter() {
        js += LISTENER_TEMPLATE
            .replace("NAME", listener.0)
            .replace("LISTENER", listener.1)
            .as_str();
    }
    write(out_path("sw.js"), &js)?;

    // compose .webmanifest with app metadata
    write(out_path(".webmanifest"), gen_manifest(opts))?;

    // at least one logo is required for PWA installability
    write(out_path("logo.png"), LOGO)?;

    println!(
        "cargo:warning={}",
        f!("composed PWA in {}ms", start.elapsed().as_millis())
    );

    Ok(())
}

fn gen_manifest(opts: PWAOptions) -> String {
    let mut manifest = Manifest::builder(&opts.name)
        .description(&opts.desc)
        .bg_color(&opts.background)
        .theme_color(&opts.theme)
        .start_url(&opts.theme)
        .display_mode(opts.display.clone());
    for icon in &opts.icons {
        manifest = manifest.icon(icon);
    }
    manifest.build().unwrap()
}

fn read_lib_name() -> Result<String> {
    use toml::{Table, Value};
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")?;
    let manifest_path = &format!("{manifest_dir}/Cargo.toml");
    let manifest = read_to_string(manifest_path)?;
    let parsed = manifest.parse::<Table>()?;
    let lib_name = if parsed.contains_key("lib") {
        let Value::Table(lib_table) = &parsed["lib"] else {
            panic!("should be unreachable");
        };
        if lib_table.contains_key("name") {
            lib_table["name"].as_str().unwrap().to_owned()
        } else {
            parsed["package"]["name"].as_str().unwrap().to_owned()
        }
    } else {
        parsed["package"]["name"].as_str().unwrap().to_owned()
    };
    Ok(lib_name.replace("-", "_"))
}

fn sw_target_dir() -> String {
    if let Some(dir) = find_target_dir() {
        dir + "/" + SW_TARGET
    } else {
        "target/".to_owned() + SW_TARGET
    }
}

/// Utility that attempts to find the path of the current build's target path
pub fn find_target_dir() -> Option<String> {
    use std::{ffi::OsStr, path::PathBuf};
    if let Some(target_dir) = std::env::var_os("CARGO_TARGET_DIR") {
        let target_dir = PathBuf::from(target_dir);
        if target_dir.is_absolute() {
            if let Some(str) = target_dir.to_str() {
                return Some(str.to_owned());
            } else {
                return None;
            }
        } else {
            return None;
        };
    }

    let mut dir = PathBuf::from(out_path(""));
    loop {
        if dir.join(".rustc_info.json").exists()
            || dir.join("CACHEDIR.TAG").exists()
            || dir.file_name() == Some(OsStr::new("target"))
                && dir
                    .parent()
                    .map_or(false, |parent| parent.join("Cargo.toml").exists())
        {
            if let Some(str) = dir.to_str() {
                return Some(str.to_owned());
            } else {
                return None;
            }
        }
        if dir.pop() {
            continue;
        }
        return None;
    }
}

/// Utility for composition of paths to build artifacts
pub fn out_path(filename: &str) -> String {
    let dir = std::env::var("OUT_DIR").unwrap();
    format!("{dir}/{filename}")
}

