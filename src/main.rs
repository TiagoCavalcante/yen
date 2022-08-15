use graphs::Graph;
use std::{env, time::Instant};

mod path;

fn main() {
  let size = 1_000;
  let density = 0.1;

  let length =
    env::args().nth(1).unwrap().parse().ok().unwrap();

  let start = 0;
  let end = 10;

  let mut graph = Graph::new(size);

  graph.fill_undirected(density);

  let now = Instant::now();
  let path = path::yen(&mut graph, start, end, length);
  println!("{:.4}", now.elapsed().as_secs_f32());

  // Test if the path is valid.
  if let Some(path) = path {
    assert_eq!(path.len(), length);
    assert_eq!(*path.first().unwrap(), start);
    assert_eq!(*path.last().unwrap(), end);

    // Check if the path is made only by real edges.
    for index in 0..path.len() - 1 {
      assert!(graph.has_edge(path[index], path[index + 1]));
    }

    // Ensure that the path contain no loops.
    let mut unique = path.clone();
    // We need a sorted vector to use dedup.
    unique.sort();
    unique.dedup();
    // If the path had loops then the length of the unique
    // vector would be smaller than the length of the path.
    assert_eq!(path.len(), unique.len());
  } else {
    panic!("Couldn't find a valid path")
  }
}
