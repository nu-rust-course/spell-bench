use rustc_version::{version_meta, Channel::Nightly};

fn main() {
    if version_meta().ok().map(|m| m.channel) == Some(Nightly) {
        println!("cargo:rustc-cfg=feature=\"nightly\"");
    }
}
