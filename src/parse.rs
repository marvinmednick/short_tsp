use std::fs::File;
use std::io::{BufReader,BufRead};
use regex::Regex;
use log::{  info , error, debug, /*warn,*/ trace };
use crate::graphbuilder::GraphBuilder;


#[derive(PartialEq)]
pub enum VertexOrder {
    SourceFirst,
    DestFirst,
}

    
// Format is 1 line per vertex with a tuple consistenting of destination vertex and weight
// First line is number of vertexes and number of edges
// e.g.    
//
// 1   2,8   3,6
// 2   1,8  3, 4
// 3   1,6, 2, 4
pub fn read_adjacency_multi<F> ( file: & mut File,  mut graph_functions: F, skip_first_line: bool)
where F: GraphBuilder,
{

    //open the file
    let mut reader = BufReader::new(file);

	let mut _line_count = 0;
    if skip_first_line {
        let mut line_data = String::new();
        if let Err(error) = reader.read_line(&mut line_data) {
            error!("Error reading first line {}",error);
        }
        trace!("First line skipped {}",line_data);
        _line_count += 1;	
    }

    for line in reader.lines() {
		_line_count += 1;	
		let line_data = line.unwrap();
        trace!("Proccesing Line {} - ({})",_line_count,line_data);
        if _line_count % 10000 == 0 {
            info!("Proccesing Line {} - ({})",_line_count,line_data);
        }

        // split the line into the vertex and the list of adjacent vertexes/weight pairs
        //let re_vertex = Regex::new(r"\s*(?P<vertex>\d+)\s+(?P<adjacent_list>.*$)").unwrap();
        let re_vertex = Regex::new(r"^\s*(?P<vertex>\d+)(?P<rest_of_line>.*)$").unwrap();
        // adjacent vertexes are in the format vertex,weight   - and regex below allows for
        // whitespace
        if let Some(caps) = re_vertex.captures(&line_data) {

            let text1 = caps.get(1).map_or("", |m| m.as_str());
            trace!("Text1  = {} caps {:?}",text1,caps);
            let vertex = text1.parse::<usize>().unwrap();
            debug!("Reading connectsion for vertex {}",vertex);

            graph_functions.add_vertex(vertex);
            let re_adjacent = Regex::new(r"\s*(?P<vertex>\d+)\s*(,|\s)\s*(?P<weight>-?\d*)").unwrap();
            let text2 = caps.get(2).map_or("", |m| m.as_str());
            trace!("Adjacency info: {}",text2);


            let mut _count =0;
            for caps in re_adjacent.captures_iter(text2) {
                let dest_vertex = caps["vertex"].parse::<usize>().unwrap(); 
                let weight = caps["weight"].parse::<i64>().unwrap(); 
                debug!("Adding connection from {} to {} with weight {}",vertex,dest_vertex,weight);
                if None == graph_functions.add_edge(vertex,dest_vertex,weight) {
                    error!("Cound not Add..");
                }
                _count += 1;

            }
        }
        else {
            error!("Line {} - No vertex found ({})",_line_count,line_data)
        }
    }
}

/*  NOT USED -- above can read either one or moer per line and with or without ,
 *
pub fn read_adjacency_single<F: GraphBuilder, >
    ( file: & mut File, order: VertexOrder, mut graph_functions: F ) {

    //open the file
    let mut reader = BufReader::new(file);

    // read the first line
    let mut line = String::new();
    let _len = reader.read_line(&mut line).unwrap();
    info!("First Input Line is \'{}\'",line);

    // parse the first line which includes number of vertexes and number of edges
    let first_line_regex = Regex::new(r"\s*(?P<num_vertex>\d+)\s+(?P<num_edges>\d+)\s+.*$").unwrap();
    let caps = first_line_regex.captures(&line).unwrap();
    let _num_vertex = caps["num_vertex"].parse::<usize>().unwrap(); 
    let _num_edges = caps["num_edges"].parse::<usize>().unwrap(); 

	let mut count = 0;
    for line in reader.lines() {
		count += 1;	
		let line_data = line.unwrap();
        debug!("Processing {} {}",count, line_data);
        if count % 50 == 0 {
            info!("Proccesed {}", count);
        }
        let line_regex = Regex::new(r"\s*(?P<v1>\d+)\s+(?P<v2>\d+)\s+(?P<weight>-?\d+)*$").unwrap();
        trace!("Line is {}",line_data);
        let caps = line_regex.captures(&line_data).unwrap();
        trace!("Caps is {:#?}",caps);
        let vertex1 = caps["v1"].parse::<usize>().unwrap(); 
        let vertex2 = caps["v1"].parse::<usize>().unwrap(); 
        let weight = caps["weight"].parse::<i64>().unwrap(); 
        if order == VertexOrder::SourceFirst {
            graph_functions.add_edge(vertex1,vertex2, weight);
        }
        else {
            graph_functions.add_edge(vertex2,vertex1, weight);
        }
    }

}
*/

