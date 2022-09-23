mod tsp;
mod unidirgraph;
mod minmax;
mod graphbuilder;
use crate::tsp::TSP;
use log::{  info ,/* error ,*/ debug, /* warn ,*/ trace };

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

    for x in 1..=5 {
        tsp.define_vertex(x,x as f64,0.0);
    }
    tsp.generate_edges_by_dist();
    let mut i = 1.5;
    tsp.define_edge(1,2,i);   i+=1.1;
    tsp.define_edge(3,2,i);   i+=1.1;
    tsp.define_edge(3,4,i);   i+=1.1;
    tsp.define_edge(4,5,i);   i+=1.1;   
    tsp.define_edge(1,5,i);   i+=1.1;
    tsp.define_edge(1,4,i);   i+=1.1;
    tsp.define_edge(1,3,i);   i+=1.1;
    tsp.define_edge(2,4,i);   i+=1.1;
    tsp.define_edge(2,5,i);   i+=1.1;
    tsp.define_edge(3,5,i);   // i+=1;

    info!("Edges");
    for edge in tsp.graph.edge_iter() {
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


}
