use std::collections::{BTreeMap, BTreeSet};

mod unidirgraph;
use crate::unidirgraph::UnidirectionalGraph;


struct TSP {
    pub  vertex_sets :  Vec<BTreeSet<usize>>,
    path_calcs : BTreeMap<BTreeSet<usize>,i64>,
}

impl TSP {

    pub fn new() -> TSP {
        TSP {
            vertex_sets: Vec::<BTreeSet<usize>>::new(),
            path_calcs : BTreeMap::<BTreeSet::<usize>,i64>::new(),
        }

    }

    pub fn add_set(&mut self, vertex_set : BTreeSet<usize>) {
        self.vertex_sets.push(vertex_set);
    }
}

fn main() {
    
    env_logger::init();

    let mut g = UnidirectionalGraph::new();
    let mut tsp = TSP::new();

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


    println!("Edges");
    for edge in &g.edges {
        println!("{:?}",edge);
    }
    println!("---------------");

    use itertools::Itertools;

 //   let range : Vec<usize> = (2..=5).into_iter().collect();
 //   println!("Range {:?}",range);
//    let mut vertex : BTreeSet<usize> = BTreeSet::<usize>::from_iter(range.iter().cloned());
    let mut vertex : BTreeSet<usize> = BTreeSet::<usize>::from_iter(g.vertex_iter().cloned());
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
            tsp.add_set(set);
        }
    }
    println!("Vertex Set {:?}", vertex_set);
    println!("G Vertex sets {:?}", tsp.vertex_sets);

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
                    print!( "   Set {:?} dist: {}  + edge {:?} dist {} = total {}  cur_min {})",reduced_set, set_weight, edge, edge_distance, new_dist, min_distance);
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


}
