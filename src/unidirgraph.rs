use std::collections::{BTreeMap, BTreeSet};
use log::{ /* info ,*/ error, debug, warn, trace };

#[derive(Debug,Clone)]
pub struct UnidirectionalGraph {
    pub vertex : BTreeSet<usize>,
    pub edges :  BTreeMap::<(usize,usize),i64>,

}

impl UnidirectionalGraph {

    pub fn new() -> UnidirectionalGraph {
        UnidirectionalGraph {  
            edges :  BTreeMap::<(usize,usize),i64>::new(),
            vertex : BTreeSet::<usize>::new(),
        }
    }

    pub fn define_vertex(&mut self,vertex: usize) {
        self.vertex.insert(vertex);
        debug!("Adding Vertex {}", vertex)

    }

    pub fn define_edge(&mut self, v1: usize, v2: usize, distance: i64) {

        self.define_vertex(v1);
        self.define_vertex(v2);
        self.edges.insert(Self::edge_name(v1,v2), distance);
    }

    // creates a properly order edge name  tuple
    // edge names between two vertex are defined to be (lower id, higher id)  in order to
    // ensure that are consistent independent of how v1 and v2 are set  
    // i.e  both v1=3, v2=2 and v1=2 and v2=3 both with result in (2,3) (the edge between 2 and 3)
    pub fn edge_name(v1: usize, v2: usize) -> (usize, usize) {

        if v1 <= v2 {
            (v1, v2)
        }
        else {
            (v2, v1)
        }
    }

    pub fn get_distance(&self, v1: usize, v2: usize) -> i64 {
        *self.edges.get(&Self::edge_name(v1,v2)).unwrap()
    }

    pub fn num_vertex(&self) -> usize {
        self.vertex.len()
    }

    pub fn vertex_iter(&self) -> std::collections::btree_set::Iter<usize> {
        self.vertex.iter()
    }
}


