use std::collections::HashMap;

#[derive(Debug)]
enum CaveSize {
    Big,
    Small,
}

struct Cave {
    id: usize,
    label: String,
    size: CaveSize,
}

struct CaveSystem {
    l_idx: HashMap<String, usize>,
    caves: HashMap<usize, Cave>,
    edges: HashMap<usize, Vec<usize>>,
}

impl CaveSystem {
    fn new() -> Self {
        Self {
            l_idx: HashMap::new(),
            caves: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    fn cave_count(&self) -> usize {
        self.caves.len()
    }

    fn add_cave(&mut self, label: &str) -> usize {
        *self.l_idx.entry(label.to_string()).or_insert_with(|| {
            let size = if label.chars().any(|c| c.is_lowercase()) {
                CaveSize::Small
            } else {
                CaveSize::Big
            };
            let id = self.caves.len();
            let cave = Cave {
                id,
                label: label.to_string(),
                size,
            };
            self.caves.insert(id, cave);

            id
        })
    }

    fn add_edge(&mut self, source: usize, target: usize) {
        self.edges.entry(source).or_insert(Vec::new()).push(target);
        self.edges.entry(target).or_insert(Vec::new()).push(source);
    }

    fn cave_id(&self, label: &str) -> usize {
        *self.l_idx.get(label).unwrap()
    }

    fn label(&self, id: usize) -> &str {
        &self.caves.get(&id).unwrap().label
    }

    fn size(&self, id: usize) -> &CaveSize {
        &self.caves.get(&id).unwrap().size
    }

    fn edges(&self, id: usize) -> &[usize] {
        &self.edges.get(&id).unwrap()
    }
}

impl From<&[&str]> for CaveSystem {
    fn from(lines: &[&str]) -> Self {
        let mut cave_system = CaveSystem::new();

        lines
            .iter()
            .map(|line| line.trim())
            .map(|line| line.split_once('-').unwrap())
            .for_each(|(source, target)| {
                let source = cave_system.add_cave(source);
                let target = cave_system.add_cave(target);
                cave_system.add_edge(source, target);
            });

        cave_system
    }
}

pub fn run(lines: &[&str]) -> (usize, usize) {
    let cave_system = CaveSystem::from(lines);

    (part1(&cave_system) as usize, part2(&cave_system) as usize)
}

fn part1(cave_system: &CaveSystem) -> u32 {
    let start = cave_system.cave_id("start");
    let end = cave_system.cave_id("end");

    let mut path = vec![];

    dfs(cave_system, start, start, end, &mut path, false)
}

fn part2(cave_system: &CaveSystem) -> u32 {
    let start = cave_system.cave_id("start");
    let end = cave_system.cave_id("end");

    let mut path = vec![];

    dfs(cave_system, start, start, end, &mut path, true)
}

fn dfs(
    cave_system: &CaveSystem,
    current_cave: usize,
    source: usize,
    target: usize,
    path: &mut Vec<usize>,
    can_revisit: bool,
) -> u32 {
    if current_cave == target {
        return 1;
    }

    let can_revisit = match (
        matches!(cave_system.size(current_cave), CaveSize::Small),
        path.contains(&current_cave),
        can_revisit,
        current_cave == source,
    ) {
        (true, true, _, true) | (true, true, false, _) => return 0,
        (true, true, true, _) => false,
        _ => can_revisit,
    };

    path.push(current_cave);
    let mut count = 0;
    for neighbor in cave_system.edges(current_cave) {
        count += dfs(cave_system, *neighbor, source, target, path, can_revisit);
    }
    path.pop();

    return count;
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT_1: &str = r#"start-A
                             start-b
                             A-c
                             A-b
                             b-d
                             A-end
                             b-end"#;

    const INPUT_2: &str = r#"dc-end
                             HN-start
                             start-kj
                             dc-start
                             dc-HN
                             LN-dc
                             HN-end
                             kj-sa
                             kj-HN
                             kj-dc"#;

    #[test]
    fn test_part1_input_1() {
        let input = INPUT_1.split('\n').collect::<Vec<_>>();

        let cave_system = CaveSystem::from(input.as_slice());

        assert_eq!(part1(&cave_system), 10);
    }

    #[test]
    fn test_part1_input_2() {
        let input = INPUT_2.split('\n').collect::<Vec<_>>();

        let cave_system = CaveSystem::from(input.as_slice());

        assert_eq!(part1(&cave_system), 19);
    }

    #[test]
    fn test_part2_input_1() {
        let input = INPUT_1.split('\n').collect::<Vec<_>>();

        let cave_system = CaveSystem::from(input.as_slice());

        assert_eq!(part2(&cave_system), 36);
    }
}
