use clap::Parser;
use slog::Drain;

#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;

#[derive(Parser, Debug)]
#[command(author,version,about,long_about=None)]
struct Args {
    #[arg(short, long)]
    address: Option<String>,

    #[arg(short, long)]
    port: Option<u32>,

    #[arg(short, long)]
    target: Option<String>,

    #[arg(short, long)]
    dag: Option<String>,
}

impl Args {
    fn set_flag(&mut self) {
        self.address = Some(self.address.as_deref().unwrap_or("0.0.0.0").to_string());
        self.port = self.port.or(Some(8080));
        self.target = Some(self.target.as_deref().unwrap_or("/").to_string());
        self.dag = Some(self.dag.as_deref().unwrap_or("{}").to_string());
    }
}

fn main() {
    let setup_logger = set_logger("run", "setup");

    let mut args = Args::parse();
    args.set_flag();

    info!(setup_logger, "{}", args.address.unwrap());
}

fn set_logger(key: &'static str, value: &'static str) -> slog::Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    return slog::Logger::root(drain, o!(key => value));
}
