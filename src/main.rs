mod bundle;
mod cli;

use anyhow::{Result, bail};
use clap::Parser;

use bundle::Bundle;
use cli::{Cli, Command};

fn main() {
    let cli = Cli::parse();
    if let Err(err) = run(cli) {
        eprintln!("rrun: {err:#}");
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Command::Create { bundle, .. } => {
            let bundle = Bundle::load(&bundle)?;
            bail!("create: not implemented yet (bundle `{}`)", bundle.path.display())
        }
        Command::Start { .. } => bail!("start: not implemented yet"),
        Command::Run { .. } => bail!("run: not implemented yet"),
        Command::State { .. } => bail!("state: not implemented yet"),
        Command::Kill { .. } => bail!("kill: not implemented yet"),
        Command::Delete { .. } => bail!("delete: not implemented yet"),
        Command::List { .. } => bail!("list: not implemented yet"),
        Command::Spec { .. } => bail!("spec: not implemented yet"),
    }
}
