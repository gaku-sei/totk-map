#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

use clap::Parser;

#[derive(Parser, Debug, bevy::prelude::Resource)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, action)]
    debug_display: bool,
}

impl From<Args> for totk_map::resources::Options {
    fn from(args: Args) -> Self {
        Self {
            debug_display: args.debug_display,
            canvas: None,
        }
    }
}

#[allow(clippy::unnecessary_wraps)]
fn main() -> anyhow::Result<()> {
    totk_map::run(Args::parse().into());
    Ok(())
}
