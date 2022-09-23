mod tsp;
mod unidirgraph;
mod minmax;
mod graphbuilder;
mod cmd_line;
mod parse;
use crate::cmd_line::CommandArgs;

use clap::Parser;
use crate::tsp::TSP;
use log::{  info ,/* error ,*/ debug, /* warn ,*/ trace };
use std::path::Path;
use std::fs::File;
use crate::parse::read_vertex_location;
use crate::minmax::{MinMax,MinMax::Value};

fn main() {
    
    env_logger::init();

    let cmd_line = CommandArgs::parse();
    debug!("The Command Line, {:?}!",cmd_line);

    // Create a path to the desired file
    let path = Path::new(&cmd_line.filename);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };


    let mut tsp = TSP::new();

    read_vertex_location(&mut file, &mut tsp);

    tsp.generate_edges_by_dist();
    tsp.calculate(1);
    let (distance, path) = tsp.solution();
    let mut int_distance : MinMax<i64> = MinMax::NA;
    if let Value(dist) = distance {
        int_distance  = Value (dist as i64)
    }
    if cmd_line.verbose {
        println!("TSP Distance {}   Path is {:?}  int distances {}", 
                 distance, path, int_distance);
    }
    else if cmd_line.path {
        println!("{:?}", path );
    }
    else {
        println!("{}",int_distance);
    }
}
