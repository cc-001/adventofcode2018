use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[macro_use]
extern crate scan_fmt;

struct Step {
    name: char,
    requirements: Vec<char>
}

struct Graph {
    steps: Vec<Step>
}

impl Graph {
    pub fn add_requirement(&mut self, requirement:&(char, char)) {
        match self.steps.iter().position(|x| x.name == requirement.1) {
            Some(pos) => self.steps[pos].requirements.push(requirement.0),
            None => self.steps.push(Step {name: requirement.1, requirements: vec![requirement.0]})
        }

        if self.steps.iter().find(|x| x.name == requirement.0).is_none() {
            self.steps.push(Step {name: requirement.0, requirements:Vec::new()})
        }
    }

    fn execute_step(&mut self) -> Option<char> {
        if self.steps.is_empty() {
            None
        }
        else {
            self.steps.sort_by(|a, b| {
                let al = a.requirements.len();
                let bl = b.requirements.len();
                if al == bl {
                    a.name.cmp(&b.name)
                } else {
                    al.cmp(&bl)
                }
            });

            let cur = self.steps.remove(0).name;
            for step in self.steps.iter_mut() {
                step.requirements.retain(|x| *x != cur);
            }
            Some(cur)
        }
    }

    pub fn execute(&mut self) -> String {
        let mut result = String::from("");
        loop {
            let step = self.execute_step();
            if step.is_some() {
                result.push(step.unwrap());
            } else {
                break;
            }
        }
        result
    }
}

fn parse_requirements(path:&str) -> Vec<(char, char)> {
    let file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", path, Error::description(&why)),
        Ok(file) => file,
    };

    let mut requirements = Vec::new();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let (a, b) = scan_fmt!(&line, "Step {} must be finished before step {} can begin.", char, char);
                requirements.push((a.unwrap(), b.unwrap()));
            }
            Err(e) => println!("err: {}", e)
        }
    }
    return requirements;
}

#[allow(dead_code)]
fn part1(path:&str) -> String {
    let reqs = parse_requirements(path);
    let mut graph = Graph {steps: Vec::new()};
    for req in &reqs {
        graph.add_requirement(req);
    }
    graph.execute()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1_example() {
        use part1;
        assert_eq!(part1("C:\\Users\\lgascoigne\\IdeaProjects\\advent\\test.txt"), "CABDFE");
    }

    #[test]
    fn test_part1_input() {
        use part1;
        assert_eq!(part1("C:\\Users\\lgascoigne\\IdeaProjects\\advent\\input.txt"), "GDHOSUXACIMRTPWNYJLEQFVZBK");
    }
}

fn main() {
    println!("{}", part1("C:\\Users\\lgascoigne\\IdeaProjects\\advent\\input.txt"));
}
