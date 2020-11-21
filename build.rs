extern crate cc;
extern crate target_lexicon;

use target_lexicon::OperatingSystem;
use std::str::FromStr;

fn main() {
    cc::Build::new()
        .file("rawdraw/CNFG.c")
        .warnings(false)
        .compile("librawdraw.a");

    let target_env = std::env::var("TARGET").unwrap();
    let triple = target_lexicon::Triple::from_str(&target_env).unwrap();

    let libs = match triple.operating_system {
        OperatingSystem::Linux => vec!["X11", "m", "pthread", "Xinerama", "Xext", "GL"],
        OperatingSystem::Windows => vec!["gdi32"],
        _ => panic!("Unsupported target operating system!")
    };
        
    for name in libs {
        println!("cargo:rustc-link-lib={}", name)        
    }
}
