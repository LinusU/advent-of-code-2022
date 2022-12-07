use std::{num::ParseIntError, str::FromStr};

use aoc_runner_derive::aoc;

struct Dir {
    children: Vec<Dir>,
    files_size: u64,
}

impl Dir {
    fn visit_all<T: FnMut(&Dir)>(&self, fun: &mut T) {
        fun(self);

        for child in self.children.iter() {
            child.visit_all(fun);
        }
    }
}

impl FromStr for Dir {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stack = Vec::<Dir>::new();

        for line in s.lines() {
            if line == "$ cd /" {
                assert_eq!(stack.len(), 0);

                stack.push(Dir {
                    children: Vec::new(),
                    files_size: 0,
                });

                continue;
            }

            if line == "$ ls" {
                continue;
            }

            if line == "$ cd .." {
                let dir = stack.pop().unwrap();
                stack.last_mut().unwrap().children.push(dir);
                continue;
            }

            if let Some(_dir) = line.strip_prefix("$ cd ") {
                stack.push(Dir {
                    children: Vec::new(),
                    files_size: 0,
                });
                continue;
            }

            if let Some(_dir) = line.strip_prefix("dir ") {
                continue;
            }

            let split_pos = line.chars().position(|c| c == ' ').unwrap();
            let (size, _) = line.split_at(split_pos);

            let size = size.parse::<u64>()?;

            for mut dir in stack.iter_mut() {
                dir.files_size += size;
            }
        }

        while stack.len() > 1 {
            let dir = stack.pop().unwrap();
            stack.last_mut().unwrap().children.push(dir);
        }

        Ok(stack.pop().unwrap())
    }
}

#[aoc(day7, part1)]
pub fn part1(input: &str) -> Result<u64, ParseIntError> {
    let root = input.parse::<Dir>()?;

    let mut result = 0;

    root.visit_all(&mut |dir| {
        if dir.files_size <= 100000 {
            result += dir.files_size;
        }
    });

    Ok(result)
}

#[aoc(day7, part2)]
pub fn part2(input: &str) -> Result<u64, ParseIntError> {
    let root = input.parse::<Dir>()?;

    let disk_size = 70_000_000u64;
    let free_space = disk_size - root.files_size;

    let target_space = 30_000_000u64;
    let need_to_free = target_space - free_space;

    let mut result = target_space;

    root.visit_all(&mut |dir| {
        if dir.files_size >= need_to_free && result > dir.files_size {
            result = dir.files_size;
        }
    });

    Ok(result)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_case_1() {
        let result = super::part1("$ cd /\n$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d\n$ cd a\n$ ls\ndir e\n29116 f\n2557 g\n62596 h.lst\n$ cd e\n$ ls\n584 i\n$ cd ..\n$ cd ..\n$ cd d\n$ ls\n4060174 j\n8033020 d.log\n5626152 d.ext\n7214296 k");
        assert_eq!(result, Ok(95437));
    }

    #[test]
    fn test_case_2() {
        let result = super::part2("$ cd /\n$ ls\ndir a\n14848514 b.txt\n8504156 c.dat\ndir d\n$ cd a\n$ ls\ndir e\n29116 f\n2557 g\n62596 h.lst\n$ cd e\n$ ls\n584 i\n$ cd ..\n$ cd ..\n$ cd d\n$ ls\n4060174 j\n8033020 d.log\n5626152 d.ext\n7214296 k");
        assert_eq!(result, Ok(24933642));
    }
}
