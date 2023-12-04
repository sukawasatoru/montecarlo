use montecarlo_pi::montecarlo_pi::parallel::parallel;
use montecarlo_pi::montecarlo_pi::serial::serial;
use montecarlo_pi::prelude::*;
use structopt::StructOpt;

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
            default_value = "0",
            help = "Number of jobs to run simultaneously"
        )]
        thread: usize,

        #[structopt(
            short = "w",
            long = "window",
            default_value = "0",
            help = "Window size"
        )]
        window: usize,
    },
}

#[tokio::main]
async fn main() -> Fallible<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    info!("Hello");

    let opt = Opt::from_args() as Opt;

    match opt.cmd {
        Command::Serial { num } => serial(num)?,
        Command::Parallel {
            num,
            thread,
            window,
        } => {
            let thread = if thread == 0 {
                num_cpus::get().max(1)
            } else {
                thread
            };
            parallel(num, thread, window).await?
        }
    }

    info!("Bye");

    Ok(())
}
