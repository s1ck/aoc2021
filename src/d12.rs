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

    dfs(
        cave_system,
        start,
        end,
        &mut path,
        &|cave_system, path, cave| {
            matches!(cave_system.size(cave), CaveSize::Small) && path.contains(&cave)
        },
    )
}

fn part2(cave_system: &CaveSystem) -> u32 {
    let start = cave_system.cave_id("start");
    let end = cave_system.cave_id("end");

    let mut path = vec![];

    dfs(
        cave_system,
        start,
        end,
        &mut path,
        &|cave_system, path, cave| {
            if cave_system.label(cave) == "end" {
                return false;
            }

            if matches!(cave_system.size(cave), CaveSize::Small) {
                let contains = path.contains(&cave);

                if cave_system.label(cave) == "start" && contains {
                    return true;
                }

                return path
                    .iter()
                    .filter(|c| matches!(cave_system.size(**c), &CaveSize::Small))
                    .fold(HashMap::new(), |mut map, cave| {
                        *map.entry(cave).or_insert(0) += 1;
                        map
                    })
                    .values()
                    .any(|count| *count > 1 && contains);
            }

            false
        },
    )
}

fn dfs<F>(
    cave_system: &CaveSystem,
    current_cave: usize,
    target_cave: usize,
    path: &mut Vec<usize>,
    halt_fn: &F,
) -> u32
where
    F: Fn(&CaveSystem, &mut Vec<usize>, usize) -> bool,
{
    if halt_fn(cave_system, path, current_cave) {
        return 0;
    }

    if current_cave == target_cave {
        return 1;
    }

    path.push(current_cave);

    let mut count = 0;

    for n in cave_system.edges(current_cave) {
        count += dfs(cave_system, *n, target_cave, path, halt_fn);
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
