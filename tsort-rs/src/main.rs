// refered
// - https://crates.io/crates/topological-sort
// - https://hcpc-hokudai.github.io/archive/graph_topological_sort_001.pdf
// - https://ja.wikipedia.org/wiki/トポロジカルソート

use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
    io::{self, BufRead, BufReader},
};

#[derive(Default)]
struct Dependency<T> {
    indegree: usize,
    outedges: HashSet<T>,
}

#[derive(Default)]
struct TopologicalSort<T> {
    graph: HashMap<T, Dependency<T>>,
}

impl<T> TopologicalSort<T>
where
    T: Hash + Eq + Clone + Default,
{
    fn add_dependency(&mut self, from: T, to: T) {
        self.graph
            .entry(from)
            .or_insert_with(Default::default)
            .outedges
            .insert(to.clone());
        self.graph
            .entry(to)
            .or_insert_with(Default::default)
            .indegree += 1;
    }

    fn sort(&mut self) -> Option<Vec<T>> {
        let mut result = Vec::new();

        let mut queue: VecDeque<T> = self
            .graph
            .iter()
            .filter(|(_, dep)| dep.indegree == 0)
            .map(|(from, _)| from.clone())
            .collect();

        while let Some(from) = queue.pop_front() {
            result.push(from.clone());

            let outedges = if let Some(dep) = self.graph.get(&from) {
                dep.outedges.clone()
            } else {
                continue;
            };

            for to in &outedges {
                let mut node = self.graph.get_mut(to).unwrap();
                node.indegree -= 1;

                if node.indegree == 0 {
                    queue.push_back(to.clone());
                }
            }
        }

        if result.len() == self.graph.len() {
            Some(result)
        } else {
            None
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut tsort = TopologicalSort::default();
    let reader = BufReader::new(io::stdin());

    for line in reader.lines() {
        let line = line?;
        let (from, to) = line.split_once(' ').unwrap();
        tsort.add_dependency(from.to_string(), to.to_string());
    }

    for e in tsort.sort().unwrap() {
        println!("{e}");
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let mut tsort = TopologicalSort::default();
        tsort.add_dependency(1, 2);
        tsort.add_dependency(2, 3);
        assert_eq!(tsort.sort().unwrap(), vec![1, 2, 3]);

        let mut tsort = TopologicalSort::default();
        tsort.add_dependency(1, 2);
        tsort.add_dependency(1, 3);
        tsort.add_dependency(2, 3);
        tsort.add_dependency(2, 4);
        tsort.add_dependency(4, 3);
        assert_eq!(tsort.sort().unwrap(), vec![1, 2, 4, 3]);

        // cycle
        let mut tsort = TopologicalSort::default();
        tsort.add_dependency(1, 2);
        tsort.add_dependency(2, 1);
        assert_eq!(tsort.sort(), None);
    }
}
