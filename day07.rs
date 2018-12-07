use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[macro_use]
extern crate scan_fmt;

struct Step {
    name: char,
    requirements: Vec<char>,
    duration: i32
}

struct Worker {
    step: char,
    work_remaining: i32
}

struct Graph {
    steps: Vec<Step>,
    idle_workers: i32,
    active_workers: Vec<Worker>
}

impl Graph {
    pub fn add_requirement(&mut self, requirement:&(char, char)) {
        match self.steps.iter().position(|x| x.name == requirement.1) {
            Some(pos) => self.steps[pos].requirements.push(requirement.0),
            None => self.steps.push(Step {name: requirement.1, requirements: vec![requirement.0], duration: 0})
        }

        if self.steps.iter().find(|x| x.name == requirement.0).is_none() {
            self.steps.push(Step {name: requirement.0, requirements:Vec::new(), duration: 0})
        }
    }

    pub fn is_done(&self) -> bool {
        self.steps.is_empty() && self.active_workers.is_empty()
    }

    fn execute_step(&mut self) -> Option<String> {
        if !self.is_done() {
            let mut result: String = String::from("");
            for active in self.active_workers.iter_mut() {
                active.work_remaining -= 1;
                if active.work_remaining <= 0 {
                    result.push(active.step);
                    for step in self.steps.iter_mut() {
                        step.requirements.retain(|x| *x != active.step);
                    }
                    self.idle_workers += 1;
                }
            }

            self.active_workers.retain(|worker| worker.work_remaining > 0);

            if self.idle_workers > 0 {
                self.steps.sort_by(|a, b| {
                    let al = a.requirements.len();
                    let bl = b.requirements.len();
                    if al == bl {
                        a.name.cmp(&b.name)
                    } else {
                        al.cmp(&bl)
                    }
                });

                let mut remove_count = 0;
                for step in &self.steps {
                    if step.requirements.is_empty() {
                        remove_count += 1;
                        self.idle_workers -= 1;
                        self.active_workers.push(Worker {step:step.name, work_remaining:step.duration});
                        if self.idle_workers <= 0 {
                            break;
                        }
                    } else {
                        break;
                    }
                }

                if remove_count > 0 {
                    self.steps = self.steps.split_off(remove_count);
                }
            }

            if !result.is_empty() {
                return Some(result);
            }
        }
        None
    }

    pub fn execute(&mut self) -> String {
        let mut result = String::from("");
        loop {
            let step = self.execute_step();
            if step.is_some() {
                result += &step.unwrap();
            }
            if self.is_done() {
                break;
            }
        }
        result
    }

    pub fn execute_time(&mut self) -> i32 {
        let mut result = 0;
        loop {
            self.execute_step();
            if self.is_done() {
                break;
            }
            result += 1;
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
fn part1(path:&str, num_workers:i32) -> String {
    let reqs = parse_requirements(path);
    let mut graph = Graph {steps: Vec::new(), idle_workers: num_workers, active_workers: Vec::new()};
    for req in &reqs {
        graph.add_requirement(req);
    }
    graph.execute()
}

#[allow(dead_code)]
fn part2(path:&str, base_duration:i32, num_workers:i32) -> i32 {
    let reqs = parse_requirements(path);
    let mut graph = Graph {steps: Vec::new(), idle_workers: num_workers, active_workers: Vec::new()};
    for req in &reqs {
        graph.add_requirement(req);
    }
    for step in graph.steps.iter_mut() {
        step.duration = base_duration + step.name as i32 - 'A' as i32 + 1;
    }
    graph.execute_time()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part1_example() {
        use part1;
        assert_eq!(part1("C:\\Users\\lgascoigne\\IdeaProjects\\advent\\test.txt", 1), "CABDFE");
    }

    #[test]
    fn test_part1_input() {
        use part1;
        assert_eq!(part1("C:\\Users\\lgascoigne\\IdeaProjects\\advent\\input.txt", 1), "GDHOSUXACIMRTPWNYJLEQFVZBK");
    }

    #[test]
    fn test_part2_example() {
        use part2;
        assert_eq!(part2("C:\\Users\\lgascoigne\\IdeaProjects\\advent\\test.txt", 0, 2), 15);
    }

    #[test]
    fn test_part2_input() {
        use part2;
        assert_eq!(part2("C:\\Users\\lgascoigne\\IdeaProjects\\advent\\input.txt", 60, 5), 1024);
    }
}

fn main() {
}
