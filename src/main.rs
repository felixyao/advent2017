use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

fn read_input() -> Vec<i32> {
    let mut senquence: Vec<i32> = Vec::new();
    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);
    for w in reader.lines() {
        let v : i32 = w.unwrap().parse().unwrap();
        senquence.push(v);
    }
    senquence 
}


#[derive(Debug)]
struct Cursor<'a> {
    input: &'a mut [i32],
    index: i32,
}

impl<'a> Cursor<'a> {

    fn new(v: &'a mut [i32]) -> Cursor<'a> {
        Cursor {
            input: v,
            index: 0,
        }
    }

    fn update(&mut self) -> i32 {
        if self.index < 0 || (self.index as usize) >= self.input.len() {
            panic!("balabalabala");
        }
        let v = self.input[self.index as usize];
        if v < 3 {
            self.input[self.index as usize] = v + 1;
        } else {
            self.input[self.index as usize] = v - 1;
        }
        v
    }

    fn jump(&mut self, steps:i32) -> bool {
        let newindex = self.index + steps;
        if newindex < 0  || (newindex as usize) >= self.input.len() {
           return true ;
        } 
        self.index = newindex;
        false
    } 

    fn next( &mut self) -> bool {
        let j = self.update();
        self.jump(j)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_test() {
        let mut input = [0,3,0,1,-3];
        let mut steps:u32 = 1;
        let mut v = Cursor::new(&mut input);
        while !v.next() {
            steps = steps + 1;
        }
        assert_eq!(10, steps);
    }
}

fn main() {
    let mut input = read_input();
    let mut steps:u32 = 1;
    {
    let mut v = Cursor::new(&mut input);
    while !v.next() {
        steps = steps + 1;
    }
    }
    println!("{}", steps);
}
