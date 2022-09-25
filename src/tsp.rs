use std::collections::{BTreeMap, BTreeSet};
use itertools::Itertools; use crate::unidirgraph::UnidirectionalGraph;
use crate::minmax::{MinMax,MinMax::NA,MinMax::Value};
use crate::unidirgraph::Vertex;
use crate::graphbuilder::GraphBuilder;
use crate::memtrack::MemTrack;
use crate::bitset::{BitSet32,u32_to_vec};

use log::{  info ,/* error ,*/ debug, /* warn ,*/ trace };

#[derive(Debug,Clone)]
struct PathCalc<T> {
    path_calcs : BTreeMap<(BTreeSet<usize>,usize),PathInfo<T>>,
}


impl <T: std::cmp::PartialOrd+std::fmt::Debug+Copy+std::ops::Add+std::fmt::Display+std::ops::Add<Output = T>> PathCalc<T>{

    pub fn new() -> PathCalc<T> {
            PathCalc::<T> { path_calcs : BTreeMap::<(BTreeSet::<usize>,usize),PathInfo<T>>::new() }
    }

    pub fn insert(&mut self, location: (BTreeSet::<usize>,usize), data: PathInfo<T>) {
            self.path_calcs.insert(location,data);
    }

    pub fn get(&self, location: &(BTreeSet::<usize>,usize)) ->  Option<&PathInfo<T>> {
        self.path_calcs.get(location)
    }
}

#[derive(Debug,Clone)]
pub struct PathInfo<T> {
    distance: T,
    prev: usize,
}

pub struct TSP<T> {
    pub vertex: BTreeSet<usize>,
    pub  vertex_sets :  Vec<BTreeSet<usize>>,
    pc : PathCalc<T>,
    pub graph: UnidirectionalGraph<T>,
    tsp_path:  Vec<usize>,
    tsp_distance: MinMax<T>,
    mc: MemTrack,
}


impl<T: PartialOrd+ std::fmt::Debug+Copy+std::fmt::Display> GraphBuilder for &mut TSP<T>{
    fn add_vertex(&mut self, id:  usize, xpos: f32, ypos: f32) {
        self.graph.define_vertex(id, xpos, ypos);
    }
}



impl TSP<f32> {
    pub fn generate_edges_by_dist(&mut self) {

        let vertex : Vec<usize> = self.graph.vertex_iter().cloned().collect();
        let vset = vertex.iter().combinations(2) ;
        for combo in vset {
            let vertex1 = self.graph.get_info(*combo[0]).unwrap();
            let vertex2 = self.graph.get_info(*combo[1]).unwrap();

            let dist = ( 
                        (vertex1.xpos - vertex2.xpos).powf(2.0) + 
                        (vertex1.ypos - vertex2.ypos).powf(2.0)
                    ).sqrt();
            trace!("Distance for v1 {} to v2 {} is {}",combo[0],combo[1],dist);
            self.define_edge(*combo[0],*combo[1],dist);
            

        }

    }


}
impl <T: std::cmp::PartialOrd+std::fmt::Debug+Copy+std::ops::Add+std::fmt::Display+std::ops::Add<Output = T>> TSP<T> {

    pub fn new() -> TSP<T> {
        TSP {
            vertex:    BTreeSet::<usize>::new(),
            vertex_sets: Vec::<BTreeSet<usize>>::new(),
            pc : PathCalc::<T>::new(),
            graph: UnidirectionalGraph::<T>::new(),
            tsp_path:  Vec::<usize>::new(),
            tsp_distance: MinMax::NA,
            mc:  MemTrack::new(),
        }

    }

    pub fn add_set(&mut self, vertex_set : BTreeSet<usize>) {
        trace!("adding Set {:?}",vertex_set);
        self.vertex_sets.push(vertex_set);
    }


    fn initialize(&mut self, starting_vertex: usize) {
        debug!("Starting Initialize");
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
        self.mc.debug_mem_info(&"After TSP Init".to_string());
        self.vertex = vertex;

    }

    pub fn find_path(&self, vertex_set: &BTreeSet<usize>, last_vertex: usize) -> Vec<usize> {
        let mut path = Vec::<usize>::new();
        let mut reduced_set = vertex_set.clone();

        let mut cur_vertex = last_vertex;
        trace!("Paths are: {:?}",self.pc);
        path.push(cur_vertex);
        while !reduced_set.is_empty() {
            trace!("Added  {}  to path, new set now {:?}",cur_vertex,reduced_set);
            let previous = self.pc.get(&(reduced_set.clone(),cur_vertex)).unwrap().prev;
            reduced_set.remove(&cur_vertex);
            trace!("Added  {}  to path, new set now {:?}, previous {}",cur_vertex,reduced_set,previous);
            path.push(previous);
            cur_vertex = previous;
        }
        info!("TSP Path is {:?}",path);
        path

    }


    pub fn calculate(&mut self, starting_vertex: usize) {

        self.initialize(starting_vertex);
        let mut size_of_set = 0;
        let mut set_count = self.vertex_sets.len();
        let mut _count = 0;
        info!("Processing {} sets",set_count);
        for set in &self.vertex_sets {
            _count += 1;
            trace!("Starting Set {:?} size: {}", set,set.len());
            self.mc.debug_mem_change(&format!("In set #{}  {:?}",_count,set));
            let mut reduced_set = set.clone();
            if size_of_set != reduced_set.len() {
                size_of_set = reduced_set.len();
                info!("Processing sets of size {}",size_of_set);
            }
            for v in set {
                reduced_set.remove(v);
                trace!(" {:?} -> v:{} Min of:", reduced_set, v);
                if reduced_set.is_empty() {
        //            println!("Edges i{:#?}",self.graph);
                    let edge = UnidirectionalGraph::<T>::edge_name(1,*v);
                    let edge_distance = self.graph.get_distance(1,*v);
                    trace!(" Edge (1,{}) i.e {:?} {}", v,edge, edge_distance);
                    let pi = PathInfo { 
                            distance: edge_distance, 
                            prev: 1, 
                        };
                    self.pc.insert((set.clone(),*v),pi);
                }
                else {
                    let mut min_distance = MinMax::NA;
                    for source in &reduced_set {
                        let edge = UnidirectionalGraph::<T>::edge_name(*source,*v);
                        let edge_distance = self.graph.get_distance(*source,*v);
                        let set_weight = self.pc.get(&(reduced_set.clone(),*source)).unwrap().distance;
                        let new_dist = set_weight + edge_distance;
                        let trace_str = format!("Set {:?} to {} via {} : sd {}+ ({},{}) d {} = {} cur {})",
                                    reduced_set, v, source, set_weight, source, v, edge_distance, new_dist, min_distance);
                        trace!("{:?},{}:  {:?},{} = {} + {} ({})",set,v,reduced_set,source,set_weight,edge_distance,new_dist);
                        let old_min_dist = min_distance;
                        if Value(new_dist) < min_distance {
                            min_distance = Value(new_dist);
                            let pi = PathInfo {
                                    distance: new_dist, 
                                    prev: *source,
                                    };
                            self.pc.insert( (set.clone(),*v),pi);
//                            trace!("{} - Updating {:?},{} to {}",trace_str, set,v, new_dist);
                            debug!("{:?},{} now {} (was {})",set,v,new_dist,old_min_dist);
                        }
                        else {
                            debug!("{} - Skipping",trace_str, );
                        }

                    }
                }
                reduced_set.insert(*v);
            }
        }

        let mut min_distance = MinMax::NA;
        let mut final_vertex : usize = 0;
        for last_vertex in &self.vertex {
            let set_weight = self.pc.get(&(self.vertex.clone(),*last_vertex)).unwrap().distance;
            let edge_distance = self.graph.get_distance(1,*last_vertex);
            let new_weight = set_weight + edge_distance;
            if Value(new_weight) < min_distance {
                min_distance = Value(new_weight);
                final_vertex = *last_vertex;
            }
        }
        info!("TSP Distance {} last vertex is {}", min_distance,final_vertex);
        self.tsp_path = self.find_path(&self.vertex,final_vertex);
        self.tsp_distance = min_distance;
    }

    pub fn define_vertex(&mut self, vertex: usize, xpos: f32, ypos: f32) {
        self.graph.define_vertex(vertex,xpos,ypos);
    }

    pub fn define_edge(&mut self, v1: usize, v2: usize, distance: T) {
        self.graph.define_edge(v1,v2,distance);
    }


    pub fn solution(&self) -> (MinMax<T>, &Vec<usize>) {
        (self.tsp_distance,&self.tsp_path)
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    fn init_log() {
       let _ = env_logger::builder().is_test(true).try_init();
       info!("Init {}",module_path!());

    }

    #[test]
    fn test_simple_5() {
        init_log();

        let mut tsp = TSP::new();

        // set position to all 0.0
        for x in 1..=5  {
            tsp.define_vertex(x,0.0,0.0);
        }

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
        tsp.calculate(1);
        let (distance, path) = tsp.solution();
        assert_eq!(distance,Value(15));
        assert_eq!(path,&vec![5,4,3,2,1]);

    }

    #[test]
    fn test_simple_4() {
        init_log();

        let mut tsp = TSP::new();

        // set position to all 0.0
        for x in 1..=4  {
            tsp.define_vertex(x,0.0,0.0);
        }

        let mut i = 1;
        tsp.define_edge(1,2,i);   i+=1;
        tsp.define_edge(3,2,i);   i+=1;
        tsp.define_edge(3,4,i);   i+=1;
        tsp.define_edge(1,4,i);   i+=1;
        tsp.define_edge(1,3,i);   i+=1;
        tsp.define_edge(2,4,i);   //i+=1;
        tsp.calculate(1);
        let (distance, path) = tsp.solution();
        assert_eq!(distance,Value(10));
        assert_eq!(path,&vec![4,3,2,1]);

    }

    #[test]
    fn test_float_4() {
        init_log();
        let mut tsp = TSP::<f32>::new();

        // set position to all 0.0
        for x in 1..=4  {
            tsp.define_vertex(x,0.0,0.0);
        }
        let mut i = 1.0;
        tsp.define_edge(1,2,i);   i+=1.0;
        tsp.define_edge(3,2,i);   i+=1.0;
        tsp.define_edge(3,4,i);   i+=1.0;
        tsp.define_edge(1,4,i);   i+=1.0;
        tsp.define_edge(1,3,i);   i+=1.0;
        tsp.define_edge(2,4,i);   //i+=1;
        tsp.calculate(1);
        let (distance, path) = tsp.solution();
        assert_eq!(distance,Value(10.0));
        assert_eq!(path,&vec![4,3,2,1]);

    }

    #[test]
    fn test_float_10_4() {
        init_log();
        let mut tsp = TSP::<f32>::new();
        tsp.define_vertex(1, 3.433752748235324,2.9215164273513206);
        tsp.define_vertex(2, 0.266027289402357, 3.367553812393056);
        tsp.define_vertex(3, 3.107592426409198, 3.091359997997841);
        tsp.define_vertex(4, 1.2770174634306963, 1.4543288785259425);
        tsp.generate_edges_by_dist();
        tsp.calculate(1);
        let (distance, path) = tsp.solution();
        debug!("Distance {} , path {:?}",distance, path);
        let mut int_distance : MinMax<i64> = MinMax::NA;
        if let Value(dist) = distance {
            int_distance  = Value (dist as i64)
        }
        assert_eq!(int_distance,Value(7));

    }

    #[test]
    fn test_float_1_2() {
        init_log();
        let mut tsp = TSP::<f32>::new();
        tsp.define_vertex(1,1.185111439847509,1.1487624635211768);
        tsp.define_vertex(2,1.4444704252469853,1.9471010355780376);
        tsp.generate_edges_by_dist();
        tsp.calculate(1);
        let (distance, path) = tsp.solution();
        debug!("Distance {} , path {:?}",distance, path);
        let mut int_distance : MinMax<i64> = MinMax::NA;
        if let Value(dist) = distance {
            int_distance  = Value (dist as i64)
        }
        assert_eq!(int_distance,Value(1));

    }

}
