use std::collections::{HashMap, BTreeSet};
fn main() {
    
    // let mut map = BTreeMap::<Vec<u32>,String>::new();
    let mut map = HashMap::<BTreeSet<u32>,String>::new();
    let vec1 = vec![1,2,3];
    let bset1 = BTreeSet::from_iter(vec1.iter().cloned());
    let vec2 = vec![3];
    let bset2 = BTreeSet::from_iter(vec2.iter().cloned());
    let vec3 = vec![10,12];
    let bset3 = BTreeSet::from_iter(vec3.iter().cloned());
    let vec4 = vec![1,10,12];
    let bset4 = BTreeSet::from_iter(vec4.iter().cloned());
    let vec5 = vec![3,2,1];
    let bset5 = BTreeSet::from_iter(vec5.iter().cloned());
    println!("bset1 {:?} bset5 {:?}",bset1,bset5);
    let mut bset6 = bset4.clone();
    bset6.remove(&1);
    map.insert(bset1,"Vector 1: 1-2-3".to_string());
    map.insert(bset2,"Vector 2: 3".to_string());
    map.insert(bset3,"Vector 3: 10-12".to_string());
    map.insert(bset4,"Vector 4: 1-10-12".to_string());
    map.insert(bset5,"Vector 5: 1-2-3".to_string());
    map.insert(bset6,"Vector 6: 10-12".to_string());
    for (key, value) in map {
        println!("{:?} {}",key, value);
        for x in key {
            println!("item {}", x);
        }
    }
}
