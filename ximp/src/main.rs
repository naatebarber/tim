mod http_svc;
mod tcp_prox;

extern crate getopts;
use getopts::Options;
use std::env::args;
use std::error::Error;

use http_svc::http_svc as transcribe;
use tcp_prox::prox as proxy;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    opts.usage(&brief);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();

    opts.optopt(
        "m",
        "mode",
        "Mode in which to run ximp",
        "[proxy, transcribe]",
    );
    opts.optflag("h", "help", "Show options");

    let m = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!("{}", f.to_string()),
    };

    if m.opt_present("h") {
        print_usage(&program, opts);
        return Ok(());
    }

    match m
        .opt_str("m")
        .unwrap_or(String::from("transcribe"))
        .as_str()
    {
        "transcribe" => {
            transcribe().await;
        }
        "proxy" => {
            proxy().await?;
        }
        m => {
            panic!("Unknown ximp mode {}", m);
        }
    }

    Ok(())
}
