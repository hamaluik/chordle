fn main() {
    println!("cargo:rerun-if-changed=migrations");

    // set a compile-time environment variable that can be used in the
    // Last-Modified header, by executing the `date` command at build time
    let last_modified = std::process::Command::new("date")
        .arg("-u")
        .arg("+%a, %d %b %Y %T GMT")
        .output()
        .expect("Can run date command");
    println!(
        "cargo:rustc-env=BUILD_TIME_LAST_MODIFIED={}",
        String::from_utf8_lossy(&last_modified.stdout)
    );
}
