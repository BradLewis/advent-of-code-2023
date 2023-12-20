#![allow(dead_code)]

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Category {
    X,
    M,
    A,
    S,
}

impl From<char> for Category {
    fn from(value: char) -> Self {
        match value {
            'x' => Category::X,
            'm' => Category::M,
            'a' => Category::A,
            's' => Category::S,
            _ => panic!("invalid category"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Rating {
    max_x: usize,
    min_x: usize,
    max_m: usize,
    min_m: usize,
    max_a: usize,
    min_a: usize,
    max_s: usize,
    min_s: usize,
}

impl Rating {
    fn contains(&self, n: usize, category: Category) -> bool {
        match category {
            Category::X => n >= self.min_x && n <= self.max_x,
            Category::M => n >= self.min_m && n <= self.max_m,
            Category::A => n >= self.min_a && n <= self.max_a,
            Category::S => n >= self.min_s && n <= self.max_s,
        }
    }

    fn set_min(&mut self, min: usize, category: Category) {
        match category {
            Category::X => self.min_x = min,
            Category::M => self.min_m = min,
            Category::A => self.min_a = min,
            Category::S => self.min_s = min,
        }
    }

    fn set_max(&mut self, max: usize, category: Category) {
        match category {
            Category::X => self.max_x = max,
            Category::M => self.max_m = max,
            Category::A => self.max_a = max,
            Category::S => self.max_s = max,
        }
    }

    fn min(&self, category: Category) -> usize {
        match category {
            Category::X => self.min_x,
            Category::M => self.min_m,
            Category::A => self.min_a,
            Category::S => self.min_s,
        }
    }

    fn max(&self, category: Category) -> usize {
        match category {
            Category::X => self.max_x,
            Category::M => self.max_m,
            Category::A => self.max_a,
            Category::S => self.max_s,
        }
    }

    fn sum(&self) -> usize {
        let mut total = 1;
        total *= self.max_x - self.min_x + 1;
        total *= self.max_m - self.min_m + 1;
        total *= self.max_a - self.min_a + 1;
        total *= self.max_s - self.min_s + 1;
        total
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Operator {
    LessThan,
    GreaterThan,
}

impl From<char> for Operator {
    fn from(value: char) -> Self {
        match value {
            '<' => Operator::LessThan,
            '>' => Operator::GreaterThan,
            _ => panic!("invalid operator"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Rule {
    op: Operator,
    cutoff: usize,
    category: Category,
}

#[derive(Debug)]
struct WorkflowPart {
    rule: Option<Rule>,
    target: String,
}

impl From<&str> for WorkflowPart {
    fn from(value: &str) -> Self {
        if value.contains(":") {
            let (comparison, target) = value
                .split_once(":")
                .expect("failed to split workflow part");

            let mut comparison = comparison.chars();

            let category = Category::from(comparison.next().expect("failed to get category"));
            let op = Operator::from(comparison.next().expect("failed to get operator"));
            let cutoff = comparison
                .as_str()
                .parse::<usize>()
                .expect("failed to get cutoff");
            Self {
                rule: Some(Rule {
                    category,
                    op,
                    cutoff,
                }),
                target: target.to_string(),
            }
        } else {
            Self {
                rule: None,
                target: value.to_string(),
            }
        }
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    parts: Vec<WorkflowPart>,
}

impl From<&str> for Workflow {
    fn from(value: &str) -> Self {
        let (name, rest) = value.split_once("{").expect("failed to split workflow");
        let rest = &rest[0..rest.len() - 1];
        let parts = rest.split(",").map(WorkflowPart::from).collect();

        Self {
            name: name.to_owned(),
            parts,
        }
    }
}

fn next(rating: Rating, workflow_name: &str, workflows: &HashMap<String, Workflow>) -> usize {
    match workflow_name {
        "A" => rating.sum(),
        "R" => 0,
        _ => {
            let w = workflows
                .get(workflow_name)
                .expect("failed to get next workflow");
            solve(rating, w, workflows)
        }
    }
}

fn solve(rating: Rating, workflow: &Workflow, workflows: &HashMap<String, Workflow>) -> usize {
    let mut rating = rating;
    let mut total = 0;
    for part in workflow.parts.iter() {
        if let Some(rule) = part.rule {
            if rating.contains(rule.cutoff, rule.category) {
                match rule.op {
                    Operator::LessThan => {
                        let mut r1 = rating.clone();
                        let mut r2 = rating.clone();
                        r1.set_max(rule.cutoff - 1, rule.category);
                        r2.set_min(rule.cutoff, rule.category);
                        rating = r2;
                        total += next(r1, &part.target, workflows);
                    }
                    Operator::GreaterThan => {
                        let mut r1 = rating.clone();
                        let mut r2 = rating.clone();
                        r1.set_max(rule.cutoff, rule.category);
                        r2.set_min(rule.cutoff + 1, rule.category);
                        rating = r1;
                        total += next(r2, &part.target, workflows);
                    }
                }
            } else {
                match rule.op {
                    Operator::LessThan => {
                        let max = rating.max(rule.category);
                        if max < rule.cutoff {
                            total += next(rating, &part.target, workflows);
                        }
                    }
                    Operator::GreaterThan => {
                        let min = rating.max(rule.category);
                        if min > rule.cutoff {
                            total += next(rating, &part.target, workflows);
                        }
                    }
                }
            }
        } else {
            total += next(rating, &part.target, workflows);
        }
    }
    total
}

fn parse_workflows(workflows: &str) -> HashMap<String, Workflow> {
    workflows
        .lines()
        .map(Workflow::from)
        .map(|w| (w.name.to_string(), w))
        .collect()
}

pub fn part2(input: &str) -> usize {
    let (workflows, _) = input.split_once("\n\n").expect("failed to split input");
    let workflows = parse_workflows(workflows);

    let start = workflows.get("in").expect("failed to get starting entry");
    let rating = Rating {
        min_x: 1,
        max_x: 4000,
        min_m: 1,
        max_m: 4000,
        min_a: 1,
        max_a: 4000,
        min_s: 1,
        max_s: 4000,
    };
    solve(rating, start, &workflows)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_part2() {
        let input = fs::read_to_string("test_input.txt").expect("failed to read test input file");
        let result = part2(&input);
        assert_eq!(result, 167409079868000)
    }
}
