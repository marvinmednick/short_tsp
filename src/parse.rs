use std::fs::File;
use std::io::{BufReader,BufRead};
use regex::Regex;
use log::{  info , error, debug, /*warn,*/ trace };
use crate::graphbuilder::GraphBuilder;


// First line is number of vertexes and number of edges
// e.g.    
//
// 1   2,8   3,6
// 2   1,8  3, 4
// 3   1,6, 2, 4
pub fn read_vertex_location<F> ( file: & mut File,  mut graph_functions: F)
where F: GraphBuilder,
{

    //open the file
    let mut reader = BufReader::new(file);

	let mut _line_count = 0;
    let mut line_data = String::new();
    if let Err(error) = reader.read_line(&mut line_data) {
        error!("Error reading first line {}",error);
    }
    let re_first_line = Regex::new(r"^\s*(?P<num_vertex>\d+)([^\d]*$|$)").unwrap();
    if let Some(caps) = re_first_line.captures(&line_data) {
        let num_vertex_text = caps.name("num_vertex").map_or("", |m| m.as_str());
        let num_vertex = num_vertex_text.parse::<usize>().unwrap();
        trace!("line {} expecting {} vertex",_line_count, num_vertex);
    }
    else {
        error!("Not able to read line {} correctly {}",_line_count,line_data);
    }
    _line_count += 1;	

    let mut vertex_count = 0;
    for line in reader.lines() {
		_line_count += 1;	
		vertex_count += 1;	
		let line_data = line.unwrap();
        trace!("Proccesing Line {} - ({})",_line_count,line_data);
        if _line_count % 10000 == 0 {
            info!("Proccesing Line {} - ({})",_line_count,line_data);
        }

        let re_float_vertex = Regex::new(r"^\s*(?P<xpos>(-*)(\d+\.\d+|\d+))\s+(?P<ypos>(-*)(\d+\.\d+)|\d+).*$").unwrap();
        if let Some(caps) = re_float_vertex.captures(&line_data) {

            let text_xpos = caps.name("xpos").map_or("", |m| m.as_str());
            trace!("Text_xpos  = {} caps {:?}",text_xpos,caps);
            let xpos = text_xpos.parse::<f64>().unwrap();

            let text_ypos = caps.name("ypos").map_or("", |m| m.as_str());
            trace!("Text_ypos  = {} caps {:?}",text_ypos,caps);
            let ypos = text_ypos.parse::<f64>().unwrap();

            graph_functions.add_vertex(vertex_count,xpos, ypos);

        }
        else {
            error!("Line {} - No vertex found ({})",_line_count,line_data);
        }
    }
}

