mod tsp;
mod unidirgraph;
mod minmax;
mod graphbuilder;
mod cmd_line;
mod parse;
mod memtrack;
mod bitset;

use crate::cmd_line::CommandArgs;
use crate::memtrack::MemTrack;

use clap::Parser;
use crate::tsp::TSP;
use log::{  debug, };
use std::path::Path;
use std::fs::File;
use crate::parse::read_vertex_location;
use crate::minmax::{MinMax,MinMax::Value};

fn main() {
    
    env_logger::init();
    let mut mc = MemTrack::new();

    mc.debug_mem_info(&"Start".to_string());

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
    mc.debug_mem_info(&"After Read".to_string());

    tsp.generate_edges_by_dist();
    mc.debug_mem_info(&"After Gen Edges".to_string());
    tsp.calculate(1);
    mc.debug_mem_info(&"After Calculate".to_string());
    let (distance, path) = tsp.solution();
    let mut int_distance : MinMax<i32> = MinMax::NA;
    if let Value(dist) = distance {
        int_distance  = Value (dist as i32)
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
