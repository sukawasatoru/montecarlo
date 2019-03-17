use log::info;
use structopt::StructOpt;

use montecarlo_pi::montecarlo_pi::{
    parallel::parallel,
    prelude::*,
    serial::serial,
};

#[derive(StructOpt, Debug)]
#[structopt(name = "montecarlo-pi")]
struct Opt {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    #[structopt(name = "serial")]
    Serial {
        #[structopt(short = "n", long = "num", help = "Plot number")]
        num: usize,
    },
    #[structopt(name = "parallel")]
    Parallel {
        #[structopt(short = "n", long = "num", help = "Plot number")]
        num: usize,

        #[structopt(
        short = "j",
        long = "jobs",
        default_value = "1",
        help = "Number of jobs to run simultaneously"
        )]
        thread: usize,
    },
}

fn main() -> Fallible<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    info!("Hello");

    let opt = Opt::from_args() as Opt;

    match opt.cmd {
        Command::Serial { num } => serial(num)?,
        Command::Parallel { num, thread } => parallel(num, thread)?,
    }

    info!("Bye");

    Ok(())
}
