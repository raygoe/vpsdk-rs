use std::env;

fn main() {
    let vpsdk_dir = match env::var("VPSDK_DIR") {
        Ok(dir) => dir,
        Err(_) => {
            println!("You need to specify the VPSDK directory as an environment variable.");
            println!("Example: export VPSDK_DIR=/path/to/vpsdk_dir");
            panic!("Build Failed.")
        }
    };

    println!("cargo:rustc-flags=-l VPSDK -L {}", vpsdk_dir);
}