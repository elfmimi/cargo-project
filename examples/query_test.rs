extern crate cargo_project;
#[macro_use]
extern crate log;
extern crate simple_logger as logger;

fn main() {
    println!("cargo-project query() test");

    logger::init_with_level(log::Level::Debug).unwrap();

    debug!("This is debug");
    trace!("This is trace");

    // assert!(env::set_current_dir(&root).is_ok());
    let project = cargo_project::Project::query("../../Work/SAMD21/atsamd-rs/boards/xiao_m0").expect("Couldn't parse the Cargo.toml");

    if let Some(target) = project.target() {
        println!("target = {}", target)
    }
}
