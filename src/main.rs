use std::collections::{BTreeMap, BTreeSet};


#[derive(Debug,Clone)]
struct TspGraph {
    pub edges :  BTreeMap::<(usize,usize),i64>,
//    sets:   BTreeMap<BTreeSet((usize,usize))>

}

impl TspGraph {

    pub fn new() -> TspGraph {
        TspGraph {  edges :  BTreeMap::<(usize,usize),i64>::new() }
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




fn main() {
    
    // let mut map = BTreeMap::<Vec<u32>,String>::new();
    let mut map = BTreeMap::<BTreeSet<(usize,usize)>,String>::new();
    let vec1 = vec![(1,2),(2,3)];
    let bset1 = BTreeSet::from_iter(vec1.iter().cloned());
    let vec2 = vec![(3,4)];
    let bset2 = BTreeSet::from_iter(vec2.iter().cloned());
    let vec3 = vec![(10,11),(12,13)];
    let bset3 = BTreeSet::from_iter(vec3.iter().cloned());
    let vec4 = vec![(1,2),(10,11),(12,13)];
    let bset4 = BTreeSet::from_iter(vec4.iter().cloned());
    let vec5 = vec![(3,4),(1,2),(1,4)];
    let bset5 = BTreeSet::from_iter(vec5.iter().cloned());
    println!("bset1 {:?} bset5 {:?}",bset1,bset5);
    let mut bset6 = bset4.clone();
    bset6.remove(&(10,11));
    map.insert(bset1,"Vector 1: 1-2-3".to_string());
    map.insert(bset2,"Vector 2: 3".to_string());
    map.insert(bset3,"Vector 3: 10-12".to_string());
    map.insert(bset4,"Vector 4: 1-10-12".to_string());
    map.insert(bset5,"Vector 5: 1-2-3".to_string());
    map.insert(bset6,"Vector 6: 10-12".to_string());
    for (key, value) in map {
        println!("{:?} {}",key, value);
        for x in key {
            println!("item {:?}", x);
        }
    }


    let mut g = TspGraph::new();
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

    let range : Vec<usize> = (2..=5).into_iter().collect();
    println!("Range {:?}",range);
    let vertex : BTreeSet<usize> = BTreeSet::<usize>::from_iter(range.iter().cloned());
    println!("Vertex {:?}",vertex);
    let mut vertex_set = Vec::<BTreeSet<usize>>::new();
    for size in 0..vertex.len()+1 {
        let vset = vertex.iter().combinations(size);
        for combo in vset {
            // for each set of combination of len 'size'
            // create 
            let set = BTreeSet::<usize>::from_iter(
                combo.into_iter().cloned().collect::<Vec<usize>>());
            println!("set {:?}", set );
            vertex_set.push(set);
        }
    }
    println!("Vertex Set {:?}", vertex_set);

    let mut tsp_calc = BTreeMap::<BTreeSet::<usize>,i64>::new();
    for set in vertex_set {
        println!("Set {:?} ", set);
        let mut reduced_set = set.clone();
        for v in &set {
            reduced_set.remove(v);
            println!(" {:?} -> v:{} Min of:", reduced_set, v);
            if reduced_set.is_empty() {
                let edge = TspGraph::edge_name(1,*v);
                let edge_distance = g.get_distance(1,*v);
                println!(" Edge (1,{}) i.e {:?} {}", v,edge, edge_distance);
                tsp_calc.insert(set.clone(),edge_distance);
            }
            else {
                let mut min_distance = i64::MAX;
                for source in &reduced_set {
                    let edge = TspGraph::edge_name(*source,*v);
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
