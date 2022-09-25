use itertools::Itertools;
use bitmaps::Bitmap;


pub struct BitSet32 {
    bitmap: Bitmap<32>, 
}


impl BitSet32 {

    pub fn new() -> BitSet32 {
        BitSet32 { bitmap: Bitmap::new()}
    }

    pub fn add(&mut self, member: usize) {
        if member < 32 {
            self.bitmap.set(member, true);
        }
        else { 
            panic!("member {} out of range (max 32)",member);
        }
    }
    
    pub fn remove(&mut self, member: usize) {
        if member < 32 {
            self.bitmap.set(member, false);
        }
        else { 
            panic!("member {} out of range (max 32)",member);
        }
    }

    pub fn add_from_vec(&mut self, list: Vec<usize>) {
        for item in list {
            self.add(item);
        }
    }

    pub fn get_set_id(&self) -> u32 {
        self.bitmap.into_value()
    }

    pub fn get_vec(&self) -> Vec<usize> {
        let indices: Vec<usize> = self.bitmap.into_iter().collect();
        indices
    }


}

pub fn u32_to_vec(value: u32) -> Vec<usize> {

    let mut remaining = value;
    let mut result = Vec::<usize>::new();


    let mut index  : usize = 0;
    while remaining > 0 {
        if remaining & 1 == 1 {
            result.push(index)
        }
        remaining = remaining >> 1;
        index += 1;
    }
    result

}

#[cfg(test)]
mod tests {


    use super::*;

    #[test]
    fn test_basic_set () {
        let mut bs = BitSet32::new();
        let vertex = vec!(0,1,2,3);
        for i in vertex.iter() {
            bs.add(*i);
        }
        assert_eq!(bs.get_set_id(),0xF);
        assert_eq!(bs.get_vec(),vec!(0,1,2,3));
    }

    #[test]
    fn test_add_vec() {
        let mut bs = BitSet32::new();
        let data = vec!(4,5,6,7);
        bs.add_from_vec(data);
        assert_eq!(bs.get_set_id(),0xF0);
        assert_eq!(bs.get_vec(),vec!(4,5,6,7));
    }


}
