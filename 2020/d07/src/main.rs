use std::collections::HashSet;
use std::collections::HashMap;
use regex::Regex;
use std::io::BufRead;

fn main() {
  let head_re = Regex::new(
    r"^([\w ]+) bags contain").unwrap();
  let anc_re = Regex::new(
    r"(\d+|no) ([\w ]+) bags?(?:, )?").unwrap();
  
  let mut containers = HashMap::<String, Vec<(i32, String)>>::new();
  std::io::stdin().lock().lines()
    .map(|line_res| line_res.unwrap())
    .for_each(|line| {
      let key = head_re.captures(&line).unwrap()[1].to_string();
      let mut contents = Vec::new();
      anc_re.captures_iter(&line).for_each(|c| {
        if c[1].ne("no") {
          contents.push((c[1].parse::<i32>().unwrap(), c[2].to_string()));
        }
      });
      if !contents.is_empty() {
        containers.insert(key, contents);
      }
    });

  let mut child_to_parents = HashMap::<&str, Vec<&str>>::new();
  containers.iter().for_each(|(parent, children)| {
    children.iter().for_each(|(_, child)| {
      let parents = child_to_parents.entry(&child).or_insert(Vec::new());
      if !parents.contains(&parent.as_str()) {
        parents.push(&parent);
      }
    });
  });

  let mut sg_ancestors = HashSet::<&str>::new();
  let mut to_visit = vec!["shiny gold"];
  while let Some(v) = to_visit.pop() {
    if let Some(parents) = child_to_parents.get(v) {
      parents.iter().for_each(|parent| {
        if sg_ancestors.insert(parent) {
          to_visit.push(parent);
        }
      });
    };
  }
  
  println!("part 1: {}", sg_ancestors.len());

  fn find_bag_count(bag: &str, containers: &HashMap<String, Vec<(i32, String)>>) -> i32 {
    if let Some(contents) = containers.get(bag) {
      contents.iter().map(|(count, child)| {
        count * (1 + find_bag_count(child, containers))
      }).sum()
    } else {
      0
    }
  };

  println!("part 2: {}", find_bag_count("shiny gold", &containers));
}

