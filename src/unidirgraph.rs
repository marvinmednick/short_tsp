use std::collections::{HashMap, BTreeMap, BTreeSet};
use log::{ /* info ,*/ error, debug, warn, trace };
use crate::graphbuilder::GraphBuilder;

#[derive(Debug,Clone)]
pub struct Vertex {
    pub vertex_id: usize,
    pub xpos:  f64,
    pub ypos: f64,
}

#[derive(Debug,Clone)]
pub struct UnidirectionalGraph<T> {
    vertex : BTreeSet<usize>,
    vertex_info: BTreeMap<usize,Vertex>,
    edges :  BTreeMap::<(usize,usize),T>,

}


impl<T: PartialOrd+ std::fmt::Debug+Copy+std::fmt::Display> GraphBuilder for &mut UnidirectionalGraph<T>{
    fn add_vertex(&mut self, id:  usize, xpos: f64, ypos: f64) {
        self.define_vertex(id, xpos, ypos);
    }
}


impl <T: std::cmp::PartialOrd+std::fmt::Debug+Copy+std::fmt::Display> UnidirectionalGraph<T> {

    pub fn new() -> UnidirectionalGraph<T> {
        UnidirectionalGraph {  
            edges :  BTreeMap::<(usize,usize),T>::new(),
            vertex_info : BTreeMap::<usize, Vertex>::new(),
            vertex : BTreeSet::<usize>::new(),
        }
    }

    pub fn define_vertex(&mut self,vertex_id: usize, xpos: f64, ypos: f64 ) {
        self.vertex_info.insert(vertex_id, Vertex {  vertex_id, xpos, ypos });
        self.vertex.insert(vertex_id);
        debug!("Adding Vertex {} ({},{})", vertex_id, xpos, ypos);

    }

    pub fn define_edge(&mut self, v1: usize, v2: usize, distance: T) {

        if self.vertex.contains(&v1) && self.vertex.contains(&v2) {
            self.edges.insert(Self::edge_name(v1,v2), distance);
            debug!("Adding Edge ({},{}) distance={}",v1,v2,distance);
        }
        else {
            error!("Bad Vertex info -- either vertex {} or {} is not yet defind",v1,v2);
        }
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

    pub fn get_distance(&self, v1: usize, v2: usize) -> T {
        *self.edges.get(&Self::edge_name(v1,v2)).unwrap()
    }

    pub fn get_info(&self, vertex_id: usize) -> Option<&Vertex>{
        self.vertex_info.get(&vertex_id)
    }

    pub fn num_vertex(&self) -> usize {
        self.vertex.len()
    }

    pub fn vertex_iter(&self) -> std::collections::btree_set::Iter<usize> {
        self.vertex.iter()
    }

    pub fn edge_iter(&self) -> std::collections::btree_map::Iter<(usize,usize),T> {
        self.edges.iter()
    }


    pub fn vertex_info_iter(&self) -> std::collections::btree_map::Iter<usize,Vertex> {
        self.vertex_info.iter()
    }
}


