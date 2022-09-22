mod tsp;
mod unidirgraph;
use crate::tsp::TSP;
use log::{  info ,/* error ,*/ debug, /* warn ,*/ trace };
/*
use std::collections::{BTreeMap, BTreeSet};
use itertools::Itertools;

use crate::unidirgraph::UnidirectionalGraph;


#[derive(Debug,Clone)]
struct PathInfo {
    distance: i64,
    prev: usize,
}

struct TSP {
    pub vertex: BTreeSet<usize>,
    pub  vertex_sets :  Vec<BTreeSet<usize>>,
    path_calcs : BTreeMap<(BTreeSet<usize>,usize),PathInfo>,
    pub graph: UnidirectionalGraph,
}

impl TSP {

    pub fn new() -> TSP {
        TSP {
            vertex:    BTreeSet::<usize>::new(),
            vertex_sets: Vec::<BTreeSet<usize>>::new(),
            path_calcs : BTreeMap::<(BTreeSet::<usize>,usize),PathInfo>::new(),
            graph: UnidirectionalGraph::new(),
        }

    }

    pub fn add_set(&mut self, vertex_set : BTreeSet<usize>) {
        trace!("adding Set {:?}",vertex_set);
        self.vertex_sets.push(vertex_set);
    }


    pub fn initialize(&mut self, starting_vertex: usize) {
        trace!("Starting Initialize");
        // creat a set with all the vertexes in it
        let mut vertex : BTreeSet<usize> = BTreeSet::<usize>::from_iter(self.graph.vertex_iter().cloned());
        
        //remove the starting vertex from set before generating all the combinations
        vertex.remove(&starting_vertex);

        // generation the list of vertex sets that we will need
        for size in 0..vertex.len()+1 {
            let vset = vertex.iter().combinations(size);
            for combo in vset {
                // for each set of combination of len 'size'
                // create 
                let set = BTreeSet::<usize>::from_iter(
                    combo.into_iter().cloned().collect::<Vec<usize>>());
                trace!("set {:?}", set );
                self.add_set(set);
            }
        }
        self.vertex = vertex;

    }

    pub fn find_path(&self, vertex_set: &BTreeSet<usize>, last_vertex: usize) {
        let mut path = Vec::<usize>::new();
        let mut reduced_set = vertex_set.clone();

        let mut cur_vertex = last_vertex;
        trace!("Paths are: {:?}",self.path_calcs);
        path.push(cur_vertex);
        while !reduced_set.is_empty() {
            trace!("Added  {}  to path, new set now {:?}",cur_vertex,reduced_set);
            let previous = self.path_calcs.get(&(reduced_set.clone(),cur_vertex)).unwrap().prev;
            reduced_set.remove(&cur_vertex);
            trace!("Added  {}  to path, new set now {:?}, previous {}",cur_vertex,reduced_set,previous);
            path.push(previous);
            cur_vertex = previous;
        }
        trace!("TSP Path is {:?}",path);

    }


    pub fn calculate_tsp_path(&mut self) {

        for set in &self.vertex_sets {
            info!("Set {:?} ", set);
            let mut reduced_set = set.clone();
            for v in set {
                reduced_set.remove(v);
                trace!(" {:?} -> v:{} Min of:", reduced_set, v);
                if reduced_set.is_empty() {
        //            println!("Edges i{:#?}",self.graph);
                    let edge = UnidirectionalGraph::edge_name(1,*v);
                    let edge_distance = self.graph.get_distance(1,*v);
                    trace!(" Edge (1,{}) i.e {:?} {}", v,edge, edge_distance);
                    self.path_calcs.insert(
                        (set.clone(),*v),
                        PathInfo { 
                            distance: edge_distance, 
                            prev: 1, 
                        }
                    );
                }
                else {
                    for source in &reduced_set {
                        let mut min_distance = i64::MAX;
                        let edge = UnidirectionalGraph::edge_name(*source,*v);
                        let edge_distance = self.graph.get_distance(*source,*v);
                        let set_weight = self.path_calcs.get(&(reduced_set.clone(),*source)).unwrap().distance;
                        let new_dist = set_weight + edge_distance;
                        let trace_str = format!( "      Set {:?} to {} via {} : sd {}+ ({},{}) d {} = {} cur {})",
                                    reduced_set, v, source, set_weight, source, v, edge_distance, new_dist, min_distance);
                        if new_dist < min_distance {
                            min_distance = new_dist;
                            self.path_calcs.insert(
                                (set.clone(),*v),
                                PathInfo {
                                    distance: new_dist, 
                                    prev: *source,
                                    }
                            );
                            trace!("{} - Updating {:?},{} to {}",trace_str, set,v, new_dist);
                        }
                        else {
                            trace!("{} - Skipping",trace_str, );
                        }

                    }
                }
                reduced_set.insert(*v);
            }
        }
        let mut min_distance = i64::MAX;
        let mut final_vertex : usize = 0;
        for last_vertex in &self.vertex {
            let set_weight = self.path_calcs.get(&(self.vertex.clone(),*last_vertex)).unwrap().distance;
            let edge_distance = self.graph.get_distance(1,*last_vertex);
            let new_weight = set_weight + edge_distance;
            if new_weight < min_distance {
                min_distance = new_weight;
                final_vertex = *last_vertex;
            }
        }
        println!("TSP Distance {} last vertex is {}", min_distance,final_vertex);use std::process::exit;
        self.find_path(&self.vertex,final_vertex);
    }

    pub fn define_vertex(&mut self, vertex: usize) {
        self.graph.define_vertex(vertex);
    }

    pub fn define_edge(&mut self, v1: usize, v2: usize, distance: i64) {
        self.graph.define_edge(v1,v2,distance);
    }

}
*/

fn main() {
    
    env_logger::init();

//    let mut g = UnidirectionalGraph::new();
    let mut tsp = TSP::new();
/*
    let mut i = 1;
    g.define_edge(1,2,i);   i+=1;
    g.define_edge(3,2,i);   i+=1;
    g.define_edge(3,4,i);   i+=1;
    g.define_edge(4,5,i);   i+=1;
    g.define_edge(1,3,i);   i+=1;
    g.define_edge(1,4,i);   i+=1;
    g.define_edge(1,5,i);   i+=1;
    g.define_edge(2,4,i);   i+=1;
    g.define_edge(2,5,i);   i+=1;
    g.define_edge(3,5,i);   
*/
    let mut i = 1;
    tsp.define_edge(1,2,i);   i+=1;
    tsp.define_edge(3,2,i);   i+=1;
    tsp.define_edge(3,4,i);   i+=1;
    tsp.define_edge(4,5,i);   i+=1;   
    tsp.define_edge(1,5,i);   i+=1;
    tsp.define_edge(1,4,i);   i+=1;
    tsp.define_edge(1,3,i);   i+=1;
    tsp.define_edge(2,4,i);   i+=1;
    tsp.define_edge(2,5,i);   i+=1;
    tsp.define_edge(3,5,i);   // i+=1;

    info!("Edges");
    for edge in &tsp.graph.edges {
        info!("{:?}",edge);
    }
    info!("---------------");
/*
 //   let range : Vec<usize> = (2..=5).into_iter().collect();
 //   println!("Range {:?}",range);
//    let mut vertex : BTreeSet<usize> = BTreeSet::<usize>::from_iter(range.iter().cloned());
    //let mut vertex : BTreeSet<usize> = BTreeSet::<usize>::from_iter(g.vertex_iter().cloned());
    println!("Vertex {:?}",vertex);
//    println!("Vertex1 {:?}",vertex);
    let mut vertex_set = Vec::<BTreeSet<usize>>::new();

    //remove the starting vertex
    let starting_vertex : usize = 1;
    vertex.remove(&starting_vertex);
    
    for size in 0..vertex.len()+1 {
        let vset = vertex.iter().combinations(size);
        for combo in vset {
            // for each set of combination of len 'size'
            // create 
            let set = BTreeSet::<usize>::from_iter(
                combo.into_iter().cloned().collect::<Vec<usize>>());
            println!("set {:?}", set );
            vertex_set.push(set.clone());
    //        tsp.add_set(set);
        }
    }
    println!("Vertex Set {:?}", vertex_set);
    println!("G Vertex sets {:?}", tsp.vertex_sets);
*/
    trace!("Calling Calcuate");
    tsp.calculate(1);
    let (distance, path) = tsp.solution();
    println!("TSP Distance {}   Path is {:?}", distance, path);

/*
    exit(0);
    println!("Old ..  ");

    let mut tsp_calc = BTreeMap::<BTreeSet::<usize>,i64>::new();
    for set in vertex_set {
        println!("Set {:?} ", set);
        let mut reduced_set = set.clone();
        for v in &set {
            reduced_set.remove(v);
            println!(" {:?} -> v:{} Min of:", reduced_set, v);
            if reduced_set.is_empty() {
                let edge = UnidirectionalGraph::edge_name(1,*v);
                let edge_distance = g.get_distance(1,*v);
                println!(" Edge (1,{}) i.e {:?} {}", v,edge, edge_distance);
                tsp_calc.insert(set.clone(),edge_distance);
            }
            else {
                let mut min_distance = i64::MAX;
                for source in &reduced_set {
                    let edge = UnidirectionalGraph::edge_name(*source,*v);
                    let edge_distance = g.get_distance(*source,*v);
                    let set_weight = tsp_calc.get(&reduced_set).unwrap();
                    let new_dist = set_weight + edge_distance;
                    print!( "   Set {:?} to {} via {}: set dist: {} + edge ({},{}) dist {} = total {}  cur_min {})",reduced_set, v, source, set_weight, source, v, edge_distance, new_dist, min_distance);
                    if new_dist < min_distance {
                        min_distance = new_dist;
                        tsp_calc.insert(set.clone(),new_dist);
                        println!(" - Updating {:?} to {}",set,new_dist);
                    }
                    else {
                        println!(" - Skipping");
                    }

                }
            }
            reduced_set.insert(*v);
        }
        println!();
    }
*/

}
