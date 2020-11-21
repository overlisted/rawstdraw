extern crate cc;

fn main() {
    cc::Build::new()
        .file("rawdraw/CNFG.c")
        .warnings(false)
        .compile("librawdraw.a");

    let libs = vec!["X11", "m", "pthread", "Xinerama", "Xext", "GL"];
        
    for name in libs {
        println!("cargo:rustc-link-lib={}", name)        
    }
}
