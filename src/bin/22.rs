use advent_of_code_2023::read_lines_as_vec;
use std::{vec, collections::{HashSet, HashMap, VecDeque}};

#[derive(Debug)]
struct Point3D(i32, i32, i32);

#[derive(PartialEq, Clone, Copy, Eq, Hash)]
struct Point2D(i32, i32);

#[derive(Debug)]
struct Block {
    start: Point3D,
    end: Point3D,
}

impl Block {
    fn project_set(&self) -> HashSet<Point2D> {
        let mut set = HashSet::new();
        for x in self.start.0..=self.end.0 {
            for y in self.start.1..=self.end.1 {
                set.insert(Point2D(x, y));
            }
        }
        set
    }

    fn set_height(&mut self, z: i32) {
        let diff = self.end.2 - self.start.2;
        self.start.2 = z;
        self.end.2 = z + diff;
    }
}

fn intersect(a: &Block, b: &Block) -> bool {
    // Two projections intersect if they share a point
    a.project_set().intersection(&b.project_set()).count() > 0
}

fn calculate_chain(block: usize, supports: &[Vec<usize>], supported_by: &[Vec<usize>]) -> u32 {
    // Stores blocks marked for removal
    let mut to_remove = HashSet::new();
    to_remove.insert(block);

    let mut blocks_to_analyze = VecDeque::new();  // BFS
    blocks_to_analyze.push_back(block);

    while !blocks_to_analyze.is_empty() {
        let current = blocks_to_analyze.pop_front().unwrap();
        // For all blocks that this block supports, check if they're supported by more than 1 block
        for &touched in &supports[current] {
            if supported_by[touched].iter().all(|&support| {
                to_remove.contains(&support) // all supports of the supported block need to have been marked for removal
            }) {
                to_remove.insert(touched);  // this block will fall since all of it's supports will fall
                blocks_to_analyze.push_back(touched);  // analyse the blocks this block supports
            }
        }
    }

    to_remove.len() as u32 - 1  // -1 because we don't count the block itself
}

fn solve(mut input: Vec<Block>) -> (u32, u32) {
    // Sort by z
    input.sort_by(|a, b| a.start.2.cmp(&b.start.2));

    let mut supported_by: Vec<Vec<usize>> = vec![Vec::new(); input.len()];
    let mut supports: Vec<Vec<usize>> = vec![Vec::new(); input.len()];

    // Since 1st block is the one with the lowest z, it's always supported by the ground
    input[0].set_height(1);

    // starting from the 2nd block, check if it projects (x, y) into any of the previous blocks
    // if it does, add a reference to this block into the previous block to mark that they touch
    for i in 1..input.len() {
        let mut height_to_blocks = HashMap::new();

        for j in 1..=i {
            let o = i - j; // from i-1 to 0

            if intersect(&input[i], &input[o]) {
                height_to_blocks.entry(input[o].end.2)
                    .and_modify(|e: &mut Vec<usize>| e.push(o))
                    .or_insert(vec![o]);  // store the top height of the block that intersects
            }
        }

        // The blocks that support us are the ones with the highest height
        let intersection_height = height_to_blocks.keys().max().cloned().unwrap_or_default();
        if intersection_height != 0 {
            supported_by[i] = height_to_blocks.remove(&intersection_height).unwrap();
            for o in &supported_by[i] {
                supports[*o].push(i);
            }
        }

        input[i].set_height(intersection_height + 1);
    }

    // Count number of removable blocks. A block can be removed if the blocks it touches are supported by more than 1 block
    let removable_blocks = (0..input.len())
        .filter(|i| {
            // For all blocks this block supports, check if they're supported by more than 1 block
            supports[*i]
                .iter()
                .all(|&touched| supported_by[touched].len() > 1)
        })
        .count() as u32;

    let mut chain_size = Vec::new();  // chain size for each block if it was removed
    for i in 0..input.len() {
        chain_size.push(calculate_chain(i, &supports, &supported_by));
    }

    // Find the sum of blocks that would fall if each block was removed
    let sum = chain_size.iter().sum();

    (removable_blocks, sum)
}

fn parse(input: &[String]) -> Vec<Block> {
    let mut blocks = Vec::new();
    for line in input {
        let mut parts = line.split('~');
        let start = parts.next().unwrap();
        let end = parts.next().unwrap();

        let mut start = start.split(',').map(|s| s.parse::<i32>().unwrap());
        let mut end = end.split(',').map(|s| s.parse::<i32>().unwrap());

        let block = Block {
            start: Point3D(
                start.next().unwrap(),
                start.next().unwrap(),
                start.next().unwrap(),
            ),
            end: Point3D(
                end.next().unwrap(),
                end.next().unwrap(),
                end.next().unwrap(),
            )};

        assert!(block.start.0 <= block.end.0);
        assert!(block.start.1 <= block.end.1);
        assert!(block.start.2 <= block.end.2);

        blocks.push(block);
    }

    blocks
}

fn main() {
    let _lines = read_lines_as_vec("inputs/22.txt").unwrap();

    let _example = r#"1,0,1~1,2,1
    0,0,2~2,0,2
    0,2,3~2,2,3
    0,0,4~0,2,4
    2,0,5~2,2,5
    0,1,6~2,1,6
    1,1,8~1,1,9"#
        .lines()
        .map(|s| s.trim().to_string())
        .collect::<Vec<String>>();

    let blocks = parse(&_lines);
    println!("There are {} blocks", blocks.len());
    let (r1, r2) = solve(blocks);
    println!("Part 1: {}", r1);
    println!("Part 2: {}", r2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intersect() {
        // a is point, b is not
        // a lies in edges of b
        assert!(intersect(
            &Block {
                start: Point3D(0, 0, 0),
                end: Point3D(0, 0, 0),
            },
            &Block {
                start: Point3D(0, 0, 0),
                end: Point3D(2, 0, 0),
            }
        ));

        // a lies somewhere in the middle of b
        assert!(intersect(
            &Block {
                start: Point3D(1, 0, 0),
                end: Point3D(1, 0, 0),
            },
            &Block {
                start: Point3D(0, 0, 0),
                end: Point3D(2, 0, 0),
            }
        ));

        // b is a point, a is not. same as previous
        assert!(intersect(
            &Block {
                start: Point3D(0, 0, 0),
                end: Point3D(2, 0, 0),
            },
            &Block {
                start: Point3D(0, 0, 0),
                end: Point3D(0, 0, 0),
            }
        ));

        assert!(intersect(
            &Block {
                start: Point3D(0, 0, 0),
                end: Point3D(2, 0, 0),
            },
            &Block {
                start: Point3D(1, 0, 0),
                end: Point3D(1, 0, 0),
            }
        ));

        // both a and b are points
        assert!(intersect(
            &Block {
                start: Point3D(1, 0, 0),
                end: Point3D(1, 0, 0),
            },
            &Block {
                start: Point3D(1, 0, 0),
                end: Point3D(1, 0, 0),
            }
        ));

        // neither a and b are points
        // parallel, non collinear
        assert!(!intersect(
            &Block {
                start: Point3D(1, 0, 0),
                end: Point3D(1, 2, 0),
            },
            &Block {
                start: Point3D(2, 0, 0),
                end: Point3D(2, 2, 0),
            }
        ));

        // collinear intersecting horizontal, at edges and in the middle
        assert!(intersect(
            &Block {
                start: Point3D(0, 0, 0),
                end: Point3D(2, 0, 0),
            },
            &Block {
                start: Point3D(2, 0, 0),
                end: Point3D(3, 0, 0),
            }
        ));
        assert!(intersect(
            &Block {
                start: Point3D(0, 0, 0),
                end: Point3D(2, 0, 0),
            },
            &Block {
                start: Point3D(1, 0, 0),
                end: Point3D(3, 0, 0),
            }
        ));

        // collinear intersecting vertical, at edges and in the middle
        assert!(intersect(
            &Block {
                start: Point3D(0, 0, 0),
                end: Point3D(0, 2, 0),
            },
            &Block {
                start: Point3D(0, 2, 0),
                end: Point3D(0, 3, 0),
            }
        ));
        assert!(intersect(
            &Block {
                start: Point3D(0, 0, 0),
                end: Point3D(0, 2, 0),
            },
            &Block {
                start: Point3D(0, 1, 0),
                end: Point3D(0, 3, 0),
            }
        ));

        // collinear non intersecting horizontal and vertical
        assert!(!intersect(
            &Block {
                start: Point3D(0, 0, 0),
                end: Point3D(2, 0, 0),
            },
            &Block {
                start: Point3D(3, 0, 0),
                end: Point3D(5, 0, 0),
            }
        ));
        assert!(!intersect(
            &Block {
                start: Point3D(0, 0, 0),
                end: Point3D(0, 2, 0),
            },
            &Block {
                start: Point3D(0, 3, 0),
                end: Point3D(0, 5, 0),
            }
        ));


        // perpendicular intersecting, both a vertical and b horizontal, and b horizontal and a vertical
        // edges touching (shared vertex)
        assert!(intersect(
            &Block {
                start: Point3D(0, 0, 0),
                end: Point3D(0, 2, 0),
            },
            &Block {
                start: Point3D(0, 2, 0),
                end: Point3D(2, 2, 0),
            }
        ));
        assert!(intersect(
            &Block {
                start: Point3D(0, 0, 0),
                end: Point3D(0, 2, 0),
            },
            &Block {
                start: Point3D(0, 0, 0),
                end: Point3D(0, 2, 0),
            }
        ));

        // edge touching middle
        assert!(intersect(
            &Block {
                start: Point3D(0, 0, 0),
                end: Point3D(0, 2, 0),
            },
            &Block {
                start: Point3D(0, 1, 0),
                end: Point3D(2, 1, 0),
            }
        ));
        assert!(intersect(
            &Block {
                start: Point3D(2, 0, 0),
                end: Point3D(2, 2, 0),
            },
            &Block {
                start: Point3D(0, 1, 0),
                end: Point3D(2, 1, 0),
            }
        ));

        // middle with middle
        assert!(intersect(
            &Block {
                start: Point3D(0, 1, 0),
                end: Point3D(2, 1, 0),
            },
            &Block {
                start: Point3D(1, 0, 0),
                end: Point3D(1, 2, 0),
            }
        ));
    }
}
