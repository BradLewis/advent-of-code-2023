#![allow(dead_code)]

use std::{collections::HashMap, fs};

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
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Rating {
    fn sum(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

impl From<&str> for Rating {
    fn from(value: &str) -> Self {
        let mut s = value.split(",");
        let x = extract_number(s.next().expect("failed to extract x"));
        let m = extract_number(s.next().expect("failed to extract m"));
        let a = extract_number(s.next().expect("failed to extract a"));
        let s = extract_number(s.next().expect("failed to extract s"));
        Self { x, m, a, s }
    }
}

fn extract_number(s: &str) -> usize {
    let (_, n) = s.split_once("=").expect("failed to split rating");
    n.parse::<usize>().expect("failed to parse rating")
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

impl WorkflowPart {
    fn evaluate(&self, rating: &Rating) -> bool {
        if let Some(rule) = self.rule {
            let part = match rule.category {
                Category::X => rating.x,
                Category::M => rating.m,
                Category::A => rating.a,
                Category::S => rating.s,
            };
            return match rule.op {
                Operator::LessThan => part < rule.cutoff,
                Operator::GreaterThan => part > rule.cutoff,
            };
        }
        true
    }
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

impl Workflow {
    fn find_target(&self, rating: &Rating) -> String {
        for part in self.parts.iter() {
            match part.rule {
                Some(_) => {
                    if part.evaluate(rating) {
                        return part.target.to_string();
                    }
                }
                None => return part.target.to_string(),
            }
        }
        panic!("failed to find target");
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("failed to read input file");
    let result = part1(&input);
    println!("part 1: {result}");
}

fn part1(input: &str) -> usize {
    let (workflows, ratings) = input.split_once("\n\n").expect("failed to split input");
    let workflows = parse_workflows(workflows);
    let ratings = parse_ratings(ratings);

    let start = workflows.get("in").expect("failed to get starting entry");
    ratings.iter().map(|r| solve(r, start, &workflows)).sum()
}

fn solve(rating: &Rating, workflow: &Workflow, workflows: &HashMap<String, Workflow>) -> usize {
    let target = workflow.find_target(rating);
    match target.as_str() {
        "A" => rating.sum(),
        "R" => 0,
        _ => {
            let w = workflows.get(&target).expect("failed to get next target");
            solve(rating, w, workflows)
        }
    }
}

fn parse_workflows(workflows: &str) -> HashMap<String, Workflow> {
    workflows
        .lines()
        .map(Workflow::from)
        .map(|w| (w.name.to_string(), w))
        .collect()
}

fn parse_ratings(ratings: &str) -> Vec<Rating> {
    ratings
        .replace("{", "")
        .replace("}", "")
        .lines()
        .map(Rating::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = fs::read_to_string("test_input.txt").expect("failed to read test input file");
        let result = part1(&input);
        assert_eq!(result, 19114);
    }
}
