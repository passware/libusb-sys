extern crate conan;

use std::{env, path};

fn main() {
    let recipe = if cfg!(windows) {
        "conanfile-win.txt"
    } else {
        "conanfile-unix.txt"
    };

    let mut install_command = conan::InstallCommandBuilder::new()
        .build_policy(conan::BuildPolicy::Never)
        .recipe_path(path::Path::new(recipe))
        .with_remote("conancenter");

    let conan_profile = env::var("CONAN_PROFILE").ok();
    if let Some(conan_profile) = conan_profile.as_ref() {
        install_command = install_command.with_profile(conan_profile.as_str());
    }

    let install_command = install_command.build();
    if let Some(build_info) = install_command.generate() {
        build_info.cargo_emit();
        return;
    }

    panic!("conan install failed");
}
