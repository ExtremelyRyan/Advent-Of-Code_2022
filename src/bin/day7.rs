// --- Day 7: No Space Left On Device ---
use std::{cell::RefCell, collections::HashMap, fs::read_to_string, path::Path, rc::Rc};

#[derive(Default)]
pub struct Aoc2022_07 {
    root: Rc<Dir>,
}

#[derive(Default, Debug)]
struct Dir {
    _name: String,
    size: RefCell<usize>,
    parent: Option<Rc<Dir>>,
    subdir: RefCell<HashMap<String, Rc<Dir>>>,
}

impl Dir {
    fn get_size(&self) -> usize {
        *self.size.borrow()
            + self
                .subdir
                .borrow()
                .values()
                .fold(0, |a, b| a + b.get_size())
    }
}

fn read_lines<T: AsRef<Path>>(pathname: T) -> Vec<String> {
    read_to_string(pathname)
        .expect("unable to open file")
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

fn main() {
    println!("DAY 7 NOT COMPLETE");
    // let mut aoc = Aoc2022_07{root : Rc::new(Dir::default())};
    // let (p1, p2) = aoc.p1_p2();
}

pub trait Run { 
    fn name(&self) -> (usize, usize);
    fn parse(&mut self);
    fn part1(&mut self) -> usize;
    fn part2(&mut self) -> usize;
    fn p1_p2 (&mut self) -> (usize, usize);
}

impl Run for Aoc2022_07 { 
    fn name(&self) -> (usize, usize) {
        (2022, 7)
    }

    fn parse(&mut self) {
        let mut cwd = Rc::clone(&self.root);
        for line in read_lines("./input/day7_input.txt") {
            let words = line.split(' ').collect::<Vec<&str>>();
            //println!("words: {words:?}");
            match (words[0], words[1]) {
                ("$", "ls") => {}
                ("$", "cd") => match words[2] {
                    "/" => cwd = Rc::clone(&self.root),
                    ".." => cwd = Rc::clone(cwd.parent.as_ref().unwrap()),
                    dirname => {
                       // println!("cwd: {:?}", cwd.subdir);
                        let newdir: Rc<Dir> = cwd.subdir.borrow().get(dirname).unwrap().clone();
                        cwd = newdir;
                    }
                },
                ("dir", dirname) => {
                    cwd.subdir.borrow_mut().insert(
                        dirname.to_string(),
                        Rc::new(Dir {
                            _name: dirname.to_string(),
                            size: RefCell::new(0),
                            parent: Some(Rc::clone(&cwd)),
                            subdir: RefCell::new(HashMap::new()),
                        }),
                    );
                }
                (size, _name) => {
                    *cwd.size.borrow_mut() += size.parse::<usize>().unwrap();
                }
            }
        }
    }

    fn part1(&mut self) -> usize {
        let mut to_visit = vec![Rc::clone(&self.root)];
        let mut total = 0;

        while let Some(dir) = to_visit.pop() {
            to_visit.extend(dir.subdir.borrow().values().map(Rc::clone));

            let size = dir.get_size();
            if size <= 100000 {
                total += size;
            }
        }
       // println!("total {}", total);
        let p2 = self.part2();
       // println!("part two {}", p2);
        total

    }

    fn part2(&mut self) -> usize {
        //println!("debug: root {0:?}", self.root);
        let total_size = self.root.get_size();
        let free_space = 70000000 - total_size;
        let space_needed = 30000000 - free_space;

        let mut to_visit = vec![Rc::clone(&self.root)];
        let mut best = usize::MAX;

        while let Some(dir) = to_visit.pop() {
            to_visit.extend(dir.subdir.borrow().values().map(Rc::clone));

            let size = dir.get_size();
            if size >= space_needed {
                best = best.min(size);
            }
        }
        //println!("best {}", best);
        best
    }

    fn p1_p2 (&mut self) -> (usize, usize) {

        let mut to_visit = vec![Rc::clone(&self.root)];
        let mut total = 0;

        while let Some(dir) = to_visit.pop() {
            to_visit.extend(dir.subdir.borrow().values().map(Rc::clone));

            let size = dir.get_size();
            if size <= 100000 {
                total += size;
            }
        }  
        

        //println!("debug: root {0:?}", self.root);
        let total_size = self.root.get_size();
        let free_space = 70000000 - total_size;
        let space_needed = 30000000 - free_space;

        let mut to_visit = vec![Rc::clone(&self.root)];
        let mut best = usize::MAX;

        while let Some(dir) = to_visit.pop() {
            to_visit.extend(dir.subdir.borrow().values().map(Rc::clone));

            let size = dir.get_size();
            if size >= space_needed {
                best = best.min(size);
            }
        }
       // println!("total: {0}, best {1}",total, best);
        (total ,best)
    }
}
