extern crate clap;
//use log::{ info , error /* ,debug, warn,trace */ };

//use clap::{Arg, Command,arg, Parser, Subcommand};
use clap::{Parser};


#[derive(Parser, Debug)]
#[clap(name = "short")]
#[clap(author = "Marvin Mednick")]
#[clap(version = "1.0")]
#[clap(about = "Traveling Salesman Path", long_about = "Dynamic Programming approach to TSP")]
pub struct CommandArgs  {

   #[clap(value_parser)]
   pub filename: String,

    #[clap(short, long )]
    input_format: Option<bool>,

    #[clap(short, long, takes_value=false)]
    /// Skips the first line of the file (e.g. first line has number of edges, vertexes)
    pub skip_first: bool,
    
    #[clap(short, long, takes_value=false)]
    /// displays the path instead of distance
    pub path: bool,

    #[clap(short, long, takes_value=false)]
    /// displays the path instead of distance
    pub verbose: bool,
}

