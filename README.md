# vpsdk-rs

A safe, Rust wrapper around vpsdk.

## Requirements

* vpsdk = 4 (r1779)

You should make sure that you install the VPSDK into a directory and create an environment variable, VPSDK_DIR, that points to the library.

## Installing

At the moment, I've not put it on cargo.io. In order to use it you can follow this:

    $ cargo new --new greeter-bot

Then you will need to modify your Cargo.toml file and add the git repo as a dependency:

    $ nano Cargo.toml
    
    [dependencies]
    vpsdk = { git = "https://github.com/raygoe/vpsdk-rs" }
 
 After that, you can use the sdk:
 
     $ nano src/main.rs
     
```rust
extern crate vpsdk;

use vpsdk::Sdk;

fn main() {
    let mut sdk = Sdk::create("universe.virtualparadise.org", 57000).expect("Could not connect!");
    sdk.login("<USERNAME>", "<PASSWORD>", "Rust Bot").expect("Could not login!");
    sdk.enter("Blizzard").expect("Could enter world!");
    sdk.update().expect("Could not update!");
}
```

Then, you can just start the bot!

    $ cargo run
