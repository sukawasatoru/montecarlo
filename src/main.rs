use clap::{Parser, Subcommand};
use montecarlo_pi::montecarlo_pi::parallel::parallel;
use montecarlo_pi::montecarlo_pi::serial::serial;
use montecarlo_pi::prelude::*;

#[derive(Parser)]
#[structopt(name = "montecarlo-pi")]
struct Opt {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Subcommand)]
enum Command {
    #[command()]
    Serial {
        /// Plot number.
        #[arg(short, long)]
        num: usize,
    },
    #[command()]
    Parallel {
        /// Plot number.
        #[arg(short, long)]
        num: usize,

        /// Number of jobs to run simultaneously.
        #[arg(
            short = 'j',
            long = "jobs",
            value_parser = clap::value_parser!(u16).range(1..),
        )]
        thread: Option<u16>,

        /// Window size.
        #[arg(short, long, default_value = "0")]
        window: usize,
    },
}

fn main() -> Fallible<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    info!("Hello");

    let opt = Opt::parse();

    match opt.cmd {
        Command::Serial { num } => serial(num)?,
        Command::Parallel {
            num,
            thread,
            window,
        } => {
            tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .worker_threads(
                    thread
                        .map(|data| data as usize)
                        .unwrap_or_else(num_cpus::get),
                )
                .build()?
                .block_on(async { parallel(num, window).await })?;
        }
    }

    info!("Bye");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn verify_cli() {
        Opt::command().debug_assert();
    }
}
