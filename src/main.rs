#![feature(proc_macro_hygiene, decl_macro)]
use structopt::StructOpt;

mod web;

#[derive(StructOpt)]
#[structopt(name="chessmate")]
struct Opt {
    /// Should app use web server or not
    #[structopt(short, long)]
    server: bool,
}

fn run_cli() {
    println!("Command line use")
}

fn main() {
    let opt = Opt::from_args();
    if opt.server {
        web::run_server()
    } else {
        run_cli()
    }
}
