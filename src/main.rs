use std::collections::{HashMap, BTreeSet};


#[derive(Debug,Clone)]
struct TspGraph {
    edges :  HashMap::<(usize,usize),i64>,
//    sets:   BTreeMap<BTreeSet((usize,usize))>

}

impl TspGraph {

    pub fn new() -> TspGraph {
        TspGraph {  edges :  HashMap::<(usize,usize),i64>::new() }
    }

    pub fn define_edge(&mut self, v1: usize, v2: usize, weight: i64) {

        let new_edge_id = {  
            if v1 <= v2 {
                (v1, v2)
            }
            else {
                (v2, v1)
            }
        };
        self.edges.insert(new_edge_id, weight);
    }
}




fn main() {
    
    // let mut map = BTreeMap::<Vec<u32>,String>::new();
    let mut map = HashMap::<BTreeSet<(usize,usize)>,String>::new();
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


    println!("{:#?}",g);

    use itertools::Itertools;

    let range : Vec<usize> = (2..=5).into_iter().collect();
    println!("Range {:?}",range);
    let vertex : BTreeSet<usize> = BTreeSet::<usize>::from_iter(range.iter().cloned());
    println!("Vertex {:?}",vertex);
    let mut vertex_set = Vec::<BTreeSet<usize>>::new();
    for size in 0..vertex.len() {
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

    for set in vertex_set {
        println!("Set {:?} ", set);
        let mut reduced_set = set.clone();
        for v in &set {
            reduced_set.remove(v);
            println!(" {:?} -> v:{} Min of:", reduced_set, v);
            if reduced_set.is_empty() {
                println!(" Edge (1,{}) ", v);
            }
            else {
                for source in &reduced_set {
                    println!( "   Set {:?} + edge ({},{})",reduced_set, source, v );

                }
            }
            reduced_set.insert(*v);
        }
        println!();
    }


}
