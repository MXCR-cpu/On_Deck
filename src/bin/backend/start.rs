use std::path::PathBuf;
use std::process::Command;

pub fn build(frontend_pages: Vec<&str>) {
    for page in frontend_pages.into_iter() {
        build_frontend(PathBuf::from(page));
    }
    set_rocket_database();
}

fn build_frontend(frontend: PathBuf) {
    println!("cargo running:{}/src", frontend.display());
    println!("cargo running:{}/index.html", frontend.display());
    let expect_string: String = format!("build.rs: Failed to build {}", frontend.display());
    Command::new("trunk")
        .args(&["build", "--release"])
        .current_dir(frontend)
        .status()
        .expect(expect_string.as_str());
}

fn set_rocket_database() {
    // Command::new("docker")
    //     .args(&["run", "-d", "--name", "redis-stack-server", "-p", "6379:6379", "redis/redis-stack-server:latest"])
    //     .status()
    //     .expect("build.rs: Failed to set ROCKET_DATABASES");
    Command::new("docker")
        .args(&["start", "redis-stack-server"])
        .status()
        .expect("build.rs: Failed to set ROCKET_DATABASES");
}
