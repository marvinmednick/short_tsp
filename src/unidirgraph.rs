use std::collections::BTreeMap;

#[derive(Debug,Clone)]
pub struct UnidirectionalGraph {
    pub edges :  BTreeMap::<(usize,usize),i64>,
//    sets:   BTreeMap<BTreeSet((usize,usize))>

}

impl UnidirectionalGraph {

    pub fn new() -> UnidirectionalGraph {
        UnidirectionalGraph {  edges :  BTreeMap::<(usize,usize),i64>::new() }
    }

    pub fn define_edge(&mut self, v1: usize, v2: usize, distance: i64) {

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
}


