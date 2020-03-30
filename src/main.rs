#![feature(proc_macro_hygiene, decl_macro)]
use structopt::StructOpt;

mod web;
mod cli;

#[derive(StructOpt)]
#[structopt(name = "chessmate")]
struct Opt {
    /// Should app use web server or not
    #[structopt(short, long)]
    server: bool,
}


fn main() {
    let opt = Opt::from_args();
    if opt.server {
        web::run_server();
    } else {
        cli::run_cli();
    }
}
