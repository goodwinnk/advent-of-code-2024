use std::collections::{HashMap, HashSet};
use advent_of_code2024_rust::{day, run_on_day_input};
use anyhow::*;
use std::io::{BufRead};
use itertools::Itertools;

fn parse_input<R: BufRead>(reader: R) -> HashMap<String, HashSet<String>> {
    let mut adjacency_list: HashMap<String, HashSet<String>> = HashMap::new();

    for line in reader.lines().flatten().filter(|line| !line.is_empty()) {
        let (a, b) = line
            .split_once('-')
            .map(|(a, b)| (a.to_string(), b.to_string())).unwrap();

        adjacency_list.entry(a.clone())
            .or_insert_with(HashSet::new)
            .insert(b.clone());

        adjacency_list.entry(b.clone())
            .or_insert_with(HashSet::new)
            .insert(a.clone());
    }

    adjacency_list
}

fn find_triplets(adjacency_list: &HashMap<String, HashSet<String>>) -> HashSet<Vec<String>> {
    let mut triplets = HashSet::new();

    for node1 in adjacency_list.keys() {
        for node2 in &adjacency_list[node1.as_str()] {
            for node3 in &adjacency_list[node2.as_str()] {
                if (node1 == node2) || (node2 == node3) || (node1 == node3) {
                    continue;
                }

                if adjacency_list[node3.as_str()].contains(node1) {
                    let mut nodes = vec![node1.clone(), node2.clone(), node3.clone()];
                    nodes.sort();
                    triplets.insert(nodes);
                }
            }
        }
    }

    triplets
}

//noinspection DuplicatedCode
fn part1<R: BufRead>(reader: R) -> Result<i64> {
    let adjacency_list = parse_input(reader);
    let triplets = find_triplets(&adjacency_list);

    Ok(
        triplets.iter()
            .filter(|triplet| triplet.iter().any(|node| node.starts_with('t')))
            .count() as i64
    )
}

fn build_biggest_cliques<'a>(
    current_clique: &'a HashSet<String>,
    candidates: &HashSet<String>,
    adjacency_list: &HashMap<String, HashSet<String>>) -> HashSet<String> {
    if candidates.is_empty() {
        return current_clique.clone();
    }

    // println!(
    //     "current_clique: {:?}, candidates: {:?}",
    //     current_clique,
    //     candidates
    // );

    let mut visited = HashSet::new();
    let mut biggest_clique = current_clique.clone();
    
    for candidate in candidates {
        let mut new_clique = current_clique.clone();
        new_clique.insert(candidate.clone());
        let new_candidates: &HashSet<String> =
            &adjacency_list[candidate.as_str()]
                .difference(&visited)
                .map(|x| x.clone())
                .collect::<HashSet<String>>()
                .intersection(candidates)
                .map(|x| x.clone())
                .collect();

        let new_clique = build_biggest_cliques(&new_clique, new_candidates, adjacency_list);
        if new_clique.len() > biggest_clique.len() {
            biggest_clique = new_clique;
        }

        visited.insert(candidate.clone());
    }

    biggest_clique
}

fn find_biggest_clique(adjacency_list: &HashMap<String, HashSet<String>>) -> HashSet<String> {
    build_biggest_cliques(&HashSet::new(), &adjacency_list.keys().map(|x| x.clone()).collect(), adjacency_list)
}

//noinspection DuplicatedCode
fn part2<R: BufRead>(reader: R) -> Result<String> {
    let adjacency_list = parse_input(reader);
    let biggest_clique = find_biggest_clique(&adjacency_list);
    let password = biggest_clique.iter().sorted().join(",").to_string();
    Ok(password)
}

//#region

fn part1_result() -> Result<()> {
    run_on_day_input(day!(), part1)?;
    Ok(())
}

fn part2_result() -> Result<()> {
    run_on_day_input(day!(), part2)?;
    Ok(())
}

fn main() {
    part1_result().unwrap();
    part2_result().unwrap();
}

//#endregion

#[cfg(test)]
mod tests {
    use std::io::BufReader;
    use indoc::indoc;
    use super::*;

    //noinspection SpellCheckingInspection
    #[cfg(test)]
    mod part1_tests {
        use super::*;

        fn test_part1(expect: i64, input: &str) {
            assert_eq!(expect, part1(BufReader::new(input.as_bytes())).unwrap());
        }

        #[test]
        fn test1() {
            test_part1(
                7,
                indoc! {"
                    kh-tc
                    qp-kh
                    de-cg
                    ka-co
                    yn-aq
                    qp-ub
                    cg-tb
                    vc-aq
                    tb-ka
                    wh-tc
                    yn-cg
                    kh-ub
                    ta-co
                    de-co
                    tc-td
                    tb-wq
                    wh-td
                    ta-ka
                    td-qp
                    aq-cg
                    wq-ub
                    ub-vc
                    de-ta
                    wq-aq
                    wq-vc
                    wh-yn
                    ka-de
                    kh-ta
                    co-tc
                    wh-qp
                    tb-vc
                    td-yn
                "},
            );
        }

        #[test]
        fn part1_final() {
            assert_eq!(1194, run_on_day_input(day!(), part1).unwrap());
        }
    }

    //noinspection SpellCheckingInspection
    #[cfg(test)]
    mod part2_tests {
        use super::*;

        fn test_part2(expect: &str, input: &str) {
            assert_eq!(expect, part2(BufReader::new(input.as_bytes())).unwrap());
        }

        #[test]
        fn test1() {
            test_part2(
                "co,de,ka,ta",
                indoc! {"
                    kh-tc
                    qp-kh
                    de-cg
                    ka-co
                    yn-aq
                    qp-ub
                    cg-tb
                    vc-aq
                    tb-ka
                    wh-tc
                    yn-cg
                    kh-ub
                    ta-co
                    de-co
                    tc-td
                    tb-wq
                    wh-td
                    ta-ka
                    td-qp
                    aq-cg
                    wq-ub
                    ub-vc
                    de-ta
                    wq-aq
                    wq-vc
                    wh-yn
                    ka-de
                    kh-ta
                    co-tc
                    wh-qp
                    tb-vc
                    td-yn
                "}
            );
        }

        #[test]
        fn part2_final() {
            assert_eq!("bd,bu,dv,gl,qc,rn,so,tm,wf,yl,ys,ze,zr", run_on_day_input(day!(), part2).unwrap());
        }
    }
}
