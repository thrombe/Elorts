#![allow(non_snake_case)]
#![allow(dead_code)]

mod remElorts;
mod dweet;
mod discord;
mod webElorts;
mod search_and_chop;

use remElorts::{RemElorts, AddReminder};
use webElorts::{WebElorts, UpdateWebElorts};

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum FuncOpt {
    //#[structopt(flatten)]
    RemElorts(RemElorts),
    WebElorts(WebElorts),
    AddReminder(AddReminder),
    UpdateWebElorts(UpdateWebElorts),
}

impl FuncOpt {
    fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            Self::RemElorts(func) => func.run(),
            Self::WebElorts(func) => func.run(),
            Self::AddReminder(func) => func.run(),
            Self::UpdateWebElorts(func) => func.run(),
        }
    }
}

#[derive(Debug, StructOpt)]
struct Opt {
    /// turns on debug prints
    #[structopt(short = "d", long = "debug")]
    debug: bool,
    
    #[structopt(subcommand)]
    sub: FuncOpt,
}

static mut DEBUG: bool = false; // using this is unsafe
/// so that i can pass -d and just have debug prints enabled
#[macro_export]
macro_rules! printdebug {
    //( $debug:expr, ( $x:expr ),* ) => {
    ( $( $x:expr ),* ) => {
        //if $debug {
        let debug: bool;
        unsafe {debug = super::DEBUG}
        if debug {
            $(
                print!("{:?}, ", $x);
            )*
            println!("\n");
        }
    };
}

fn main() {
    let mut opt = Opt::from_args();
    // println!("{:?}", opt);
    unsafe {DEBUG = opt.debug}
    // printdebug!(2, 4, 5);
    
    opt.sub.run().unwrap();
}
