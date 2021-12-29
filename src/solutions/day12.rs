use std::collections::HashMap;
use std::io;
use std::io::BufRead;
use itertools::Itertools;

fn read_input(input: impl BufRead) -> Vec<(String, String)> {
    input.lines().map(|line| {
        line.unwrap().split('-').map(String::from).collect_tuple().unwrap()
    }).collect()
}

pub fn main() {
    let edges = read_input(io::stdin().lock());
    let graph = Graph::new(edges.into_iter());

    println!("{}", graph.part_1());
    println!("{}", graph.part_2());
}

#[derive(Debug, Default)]
struct Graph {
    edges: HashMap<String, Vec<String>>,
}

impl Graph {
    fn new(edges: impl Iterator<Item=(String, String)>) -> Self {
        let mut graph = Graph::default();

        for (a, b) in edges {
            assert!(is_small(&a) || is_small(&b), "Adjacent large caves, our code won't work.");

            graph.edges.entry(a.clone()).or_default().push(b.clone());
            graph.edges.entry(b).or_default().push(a);
        }

        graph
    }

    /// Return the number of paths from "start" to "end".
    ///
    /// Paths are allowed to visit small caves at most once each.
    fn part_1(&self) -> u32 {
        let mut curr_path = HashMap::new();
        self.dfs("start", &mut curr_path)
    }

    /// Helper function for `part_1`.
    ///
    /// How many paths from here to end?
    fn dfs<'a>(&'a self, cave: &'a str, curr_path: &mut HashMap<&'a str, u32>) -> u32 {
        if cave == "end" {
            return 1;
        }

        *curr_path.entry(cave).or_default() += 1;

        let ret = self.edges[cave].iter().filter_map(|other| {
            // Don't re-visit small caves.
            if is_small(other) && curr_path.contains_key(other.as_str()) {
                None
            } else {
                Some(self.dfs(other, curr_path))
            }
        }).sum();

        let freq = curr_path.get_mut(cave).unwrap();
        *freq -= 1;
        if *freq == 0 {
            curr_path.remove(cave);
        }

        ret
    }

    /// Return the number of paths from "start" to "end".
    ///
    /// Same rules as part 1, except a path may re-visit a single small cave once.
    fn part_2(&self) -> u32 {
        let mut curr_path = HashMap::new();
        self.dfs_part_2("start", &mut curr_path, false)
    }

    /// Helper function for `part_2`.
    ///
    /// How many paths from here to end?
    fn dfs_part_2<'a>(&'a self, cave: &'a str, curr_path: &mut HashMap<&'a str, u32>, revisited_small: bool) -> u32 {
        if cave == "end" {
            return 1;
        }

        *curr_path.entry(cave).or_default() += 1;

        let ret = self.edges[cave].iter().filter_map(|other| {
            let small_repeat = is_small(other) && curr_path.contains_key(other.as_str());

            // Never re-visit start.
            if other == "start" {
                None
            }
            // Don't re-visit a small cave if we've already used up our quota.
            else if small_repeat && revisited_small {
                None
            }
            else {
                Some(self.dfs_part_2(other, curr_path, revisited_small || small_repeat))
            }
        }).sum();

        let freq = curr_path.get_mut(cave).unwrap();
        *freq -= 1;
        if *freq == 0 {
            curr_path.remove(cave);
        }

        ret
    }
}

fn is_small(cave: &str) -> bool {
    cave.chars().all(|c| c.is_lowercase())
}
