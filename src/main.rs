use env_logger::Env;
use structopt::StructOpt;

mod server;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "cut-optimizer-2d-server",
    about = "A cut optimizer server for optimizing rectangular cut pieces from sheet goods.",
    author = "Jason Hansen <jasonrodneyhansen@gmail.com>"
)]
pub(crate) struct Opt {
    /// IP address to listen on
    #[structopt(
        short = "i",
        long = "ip",
        default_value = "127.0.0.1",
        env = "CUT_OPTIMIZER_2D_IP"
    )]
    ip: String,

    /// Port to listen on
    #[structopt(
        short = "p",
        long = "port",
        default_value = "3030",
        env = "CUT_OPTIMIZER_2D_PORT"
    )]
    port: u16,

    /// Maximum length of request body
    #[structopt(long = "max-content-length", default_value = "32896")]
    max_content_length: u64,

    /// Silence all log output
    #[structopt(short = "q", long = "quiet")]
    quiet: bool,

    /// Verbose logging mode (-v, -vv, -vvv)
    #[structopt(short = "v", long = "verbose", parse(from_occurrences))]
    verbose: usize,
}

#[tokio::main]
async fn main() {
    let opt = Opt::from_args();

    init_logger(&opt);

    server::serve(&opt).await;
}

fn init_logger(opt: &Opt) {
    if !opt.quiet {
        env_logger::Builder::from_env(Env::default().default_filter_or(match opt.verbose {
            0 => "warn",
            1 => "info",
            2 => "debug",
            _ => "trace",
        }))
        .init();
    }
}
