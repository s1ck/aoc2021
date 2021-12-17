use std::{cmp::Ordering, collections::BinaryHeap};

use graph::prelude::*;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: u32,
    position: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn run(lines: &str) -> (usize, usize) {
    let g1 = parse(lines, 1);
    let g2 = parse(lines, 5);

    (
        dijkstra(&g1, 0, g1.node_count() - 1) as usize,
        dijkstra(&g2, 0, g2.node_count() - 1) as usize,
    )
}

fn dijkstra(g: &UndirectedCsrGraph<usize, (), u32>, start: usize, end: usize) -> u32 {
    let mut dist = (0..g.node_count()).map(|_| u32::MAX).collect::<Vec<_>>();
    let mut heap = BinaryHeap::new();

    dist[start] = 0;

    heap.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = heap.pop() {
        // found shortest path
        if position == end {
            return cost;
        }

        // already found a shorter path
        if cost > dist[position] {
            continue;
        }

        for Target { target, value } in g.neighbors_with_values(position) {
            let next = State {
                cost: cost + value,
                position: *target,
            };

            if next.cost < dist[next.position] {
                heap.push(next);
                dist[next.position] = next.cost;
            }
        }
    }

    u32::MAX
}

fn parse(lines: &str, scale: usize) -> UndirectedCsrGraph<usize, (), u32> {
    let field = lines
        .split('\n')
        .map(|line| line.trim())
        .map(|line| line.as_bytes().iter().map(|b| b - b'0').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let height = field.len();
    let width = field[0].len();
    let new_height = height * scale;
    let new_width = width * scale;

    let mut edges = Vec::new();

    (0..height).for_each(|row| {
        (0..width).for_each(|col| {
            (0..scale).for_each(|s_row| {
                (0..scale).for_each(|s_col| {
                    let n = field[row][col] as usize;
                    let n = (n - 1 + s_row + s_col) % 9 + 1;

                    let s_row = row + (s_row * height);
                    let s_col = col + (s_col * width);

                    let source = s_row * new_width + s_col;

                    for (n_row, n_col) in [
                        (s_row.wrapping_sub(1), s_col),
                        (s_row, s_col.wrapping_sub(1)),
                        (s_row, s_col + 1),
                        (s_row + 1, s_col),
                    ] {
                        if n_row < new_height && n_col < new_width {
                            let target = n_row * new_width + n_col;
                            edges.push((target, source, n as u32));
                        }
                    }
                })
            })
        })
    });

    GraphBuilder::new()
        .csr_layout(CsrLayout::Deduplicated)
        .edges_with_values(edges)
        .build()
}

#[cfg(test)]
mod tests {
    use test::Bencher;

    use super::*;

    const INPUT: &str = r#"1163751742
                           1381373672
                           2136511328
                           3694931569
                           7463417111
                           1319128137
                           1359912421
                           3125421639
                           1293138521
                           2311944581"#;
    #[test]
    fn test_part1() {
        let g = parse(INPUT, 1);

        assert_eq!(g.node_count(), 100);
        assert_eq!(g.edge_count(), 180);

        assert_eq!(dijkstra(&g, 0, g.node_count() - 1), 40);
    }

    #[test]
    fn test_part2() {
        let g = parse(INPUT, 5);

        assert_eq!(g.node_count(), 2500);
        assert_eq!(g.edge_count(), 4900);

        assert_eq!(dijkstra(&g, 0, g.node_count() - 1), 315);
    }

    #[bench]
    fn bench_part1(b: &mut Bencher) {
        let input = std::fs::read_to_string("input/d15.txt").expect("file not found");

        b.iter(|| {
            let g = parse(input.as_str(), 1);
            dijkstra(&g, 0, g.node_count() - 1)
        });
    }

    #[bench]
    fn bench_part2(b: &mut Bencher) {
        let input = std::fs::read_to_string("input/d15.txt").expect("file not found");

        b.iter(|| {
            let g = parse(input.as_str(), 5);
            dijkstra(&g, 0, g.node_count() - 1)
        });
    }
}
