mod cliargs;
mod display;
mod ip;

use crate::cliargs::CliArgs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cliargs = CliArgs::from_args()?;

    println!("{}", cliargs.get_target());

    if cliargs.get_network() {
        println!("Network: {}", cliargs.get_target().network());
    }
    if cliargs.get_broadcast() {
        println!("Broadcast: {}", cliargs.get_target().broadcast());
    }
    if cliargs.get_mask() {
        println!("Mask: {}", cliargs.get_target().mask());
    }
    if cliargs.get_hostmin() {
        println!("Hostmin: {}", cliargs.get_target().hostmin());
    }
    if cliargs.get_hostmax() {
        println!("Hostmax: {}", cliargs.get_target().hostmax());
    }
    if cliargs.get_count() {
        println!("Count: {}", cliargs.get_target().count()?);
    }
    Ok(())
}
