use clap::Parser;
use crate::subcommands::MathCommands;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct MathArgs {

    #[command(subcommand)]
    pub(crate) command: Option<MathCommands>

}