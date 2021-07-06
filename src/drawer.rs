
use crate::decode::{Context};
use crate::image::{DrawCommand};
use crate::utils::load;

crate::entry_point!("drawer", drawer_main);

fn drawer_main() {
    let commands: Vec<DrawCommand> = load("commands.ron");
    println!("Commands:\n{:#?}", commands);
}