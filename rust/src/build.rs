use std::path::Path;

extern crate rustsourcebundler;
use rustsourcebundler::Bundler;

fn main() {
    // region bundle files
    let mut bundler: Bundler = Bundler::new(Path::new("src/bin/csbk.rs"),
                                            Path::new("src/bin/singlefile.rs"));
    bundler.crate_name("<crate name>");
    bundler.run();
    // endregion
}