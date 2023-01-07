#![feature(async_fn_in_trait)]
#![feature(associated_type_defaults)]
#![warn(incomplete_features)]
mod client;
mod plugin;

fn main() {
    let mut plugin = plugin::build_plugin();
    plugin.start();
}
