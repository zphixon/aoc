use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

pub fn run(example: bool) {
    let data = if example {
        include_str!("../data/example/day5.1.txt")
    } else {
        include_str!("../data/day5.1.txt")
    };

    tracing::info!("day 5 part 1{}", if example { " example" } else { "" });
    tracing::info!(
        "day 5 part 1{} result: {}",
        if example { " example" } else { "" },
        part1(data),
    );

    if example {
        tracing::warn!("used example data");
    }

    tracing::info!("day 5 part 2{}", if example { " example" } else { "" });
    tracing::info!(
        "day 5 part 2{} result: {}",
        if example { " example" } else { "" },
        part2(data), // same data
    );
}

#[derive(Debug)]
struct Rule {
    before: usize,
    after: usize,
}

#[derive(Debug)]
struct OrderRules {
    updates: Vec<Vec<usize>>,

    /// things that can come after a usize
    prev_nexts: HashMap<usize, HashSet<usize>>,

    /// things that can come before a usize
    next_prevs: HashMap<usize, HashSet<usize>>,
}

impl FromStr for OrderRules {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut prev_nexts = HashMap::<usize, HashSet<usize>>::new();
        let mut next_prevs = HashMap::<usize, HashSet<usize>>::new();

        let mut split = s.split("\n\n");
        let Some(rules_str) = split.next() else {
            return Err(String::from("no rules"));
        };
        let Some(updates_str) = split.next() else {
            return Err(String::from("no rules"));
        };

        let mut rules = Vec::new();
        for rule_line in rules_str.lines() {
            let mut split = rule_line.split("|");
            let Some(before_str) = split.next() else {
                return Err(String::from("no before in rule"));
            };
            let Some(after_str) = split.next() else {
                return Err(String::from("no after in rule"));
            };
            let Ok(before) = before_str.parse() else {
                return Err(String::from("before not a number"));
            };
            let Ok(after) = after_str.parse() else {
                return Err(String::from("after not a number"));
            };
            rules.push(Rule { before, after });
        }

        for rule in rules.iter() {
            prev_nexts
                .entry(rule.before)
                .or_default()
                .insert(rule.after);
            next_prevs
                .entry(rule.after)
                .or_default()
                .insert(rule.before);
        }

        //let mut real_prev_nexts = prev_nexts.clone();
        //for (prev, nexts) in prev_nexts.iter() {
        //    let mut graphed = HashSet::new();
        //    let mut queue = nexts.clone();
        //    tracing::trace!("prev={}", prev);

        //    while !queue.is_empty() {
        //        tracing::trace!("queue={:?}", queue);
        //        for next in queue.clone() {
        //            tracing::trace!("remove {}", next);
        //            queue.remove(&next);
        //            if graphed.contains(&next) {
        //                continue;
        //            }
        //            graphed.insert(next);
        //            if let Some(more_nexts) = prev_nexts.get(&next) {
        //                tracing::trace!("add {:?}", more_nexts);
        //                queue.extend(more_nexts);
        //            }
        //            real_prev_nexts.entry(*prev).or_default().insert(next);
        //        }
        //    }
        //}

        for (&next, prevs) in next_prevs.iter() {
            for &prev in prevs {
                prev_nexts.entry(prev).or_default().insert(next);
            }
        }

        let mut updates = Vec::new();
        for update_line in updates_str.lines() {
            updates.push(
                update_line
                    .split(",")
                    .map(|page| page.parse())
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|_| String::from("page not a number"))?,
            );
        }

        Ok(OrderRules {
            //prev_nexts: real_prev_nexts,
            prev_nexts,
            next_prevs,
            updates,
        })
    }
}

impl OrderRules {
    fn is_allowed(&self, prev: &usize, next: &usize) -> bool {
        self.prev_nexts.get(prev).map(|after| after.contains(next)) == Some(true)
            || self
                .next_prevs
                .get(next)
                .map(|before| before.contains(prev))
                == Some(true)
    }

    fn is_in_order(&self, update: &[usize]) -> bool {
        tracing::debug!("checking update {:?}", update);
        let mut prev = update.first().unwrap();
        for next in update.iter().skip(1) {
            if self.is_allowed(prev, next) {
                tracing::trace!("{} -> {} ok", prev, next);
            } else {
                tracing::debug!("{} -> {} not allowed", prev, next);
                return false;
            }
            prev = next;
        }
        tracing::debug!("update seems ok");
        true
    }

    fn updates_in_order(&self) -> Vec<&Vec<usize>> {
        self.updates
            .iter()
            .filter(|update| self.is_in_order(update.as_slice()))
            .collect()
    }

    fn cloned_updates_out_of_order(&self) -> Vec<Vec<usize>> {
        self.updates
            .iter()
            .filter(|update| !self.is_in_order(update.as_slice()))
            .cloned()
            .collect()
    }
}

fn part1(data: &str) -> usize {
    let Ok(rules) = data.parse::<OrderRules>() else {
        tracing::error!("bad data");
        return 0;
    };
    tracing::debug!("{:?}", rules);
    rules
        .updates_in_order()
        .into_iter()
        .map(|update| update[update.len() / 2])
        .sum()
}

fn extract_allowed_subset(rules: &OrderRules, mut update: Vec<usize>) -> (Vec<usize>, Vec<usize>) {
    let mut removed = Vec::new();

    tracing::debug!("extracting from {:?}", update);

    let mut prev = update[0];
    let mut ni = 1;
    while ni < update.len() {
        let next = update[ni];
        if !rules.is_allowed(&prev, &next) {
            tracing::trace!("{} -> {} not allowed, removing {}", prev, next, next);
            update.remove(ni);
            removed.push(next);
        } else {
            prev = next;
            ni += 1;
        }
    }

    assert!(rules.is_in_order(update.as_slice()), "fundamental flaw");
    (update, removed)
}

fn part2(data: &str) -> usize {
    let Ok(rules) = data.parse::<OrderRules>() else {
        tracing::error!("bad data");
        return 0;
    };
    tracing::debug!("{:?}", rules);

    let mut center_sum = 0;

    for update in rules.cloned_updates_out_of_order() {
        tracing::debug!("fixing update {:?}", update);

        let (mut update, removed) = extract_allowed_subset(&rules, update);

        tracing::debug!("left over: {:?}", update);
        tracing::debug!("removed: {:?}", removed);

        for item in removed {
            let mut i = update.len() - 1;
            while i > 0 {
                if rules.is_allowed(&update[i - 1], &item) {
                    tracing::trace!("{} -> {} ok", update[i - 1], item);
                    break;
                }
                tracing::trace!("{} -> {} not allowed", update[i - 1], item);
                i -= 1;
            }
            update.insert(i, item);
            tracing::trace!("inserted at {}: {:?}", i, update);
        }

        tracing::debug!("fixed: {:?}", update);
        assert!(rules.is_in_order(update.as_slice()), "nooooooo 😭😭😭😭😭😿😿😿😿");
        center_sum += update[update.len() / 2];
    }

    center_sum
}

#[test]
fn test1() {
    assert_eq!(part1(include_str!("../data/day5.1.txt")), 4462);
}

#[test]
fn test2() {
    assert_eq!(part2(include_str!("../data/day5.1.txt")), 1952);
}
