use std::collections::{HashMap, VecDeque};

fn find_regions(map: &Vec<Vec<char>>) -> HashMap<char, Vec<Vec<(usize, usize)>>> {
    let rows = map.len();
    let cols = map[0].len();
    let mut visited = vec![vec![false; cols]; rows];
    let mut regions: HashMap<char, Vec<Vec<(usize, usize)>>> = HashMap::new();

    for r in 0..rows {
        for c in 0..cols {
            if !visited[r][c] {
                let plant_type = map[r][c];
                if plant_type == ' ' {
                    continue;
                }

                let (region, region_visited) = bfs(map, &mut visited, r, c, plant_type);

                // Update the larger visited set
                for (vr, vc) in region_visited.iter() {
                    visited[*vr][*vc] = true;
                }

                regions.entry(plant_type).or_default().push(region);
            }
        }
    }

    regions
}

fn bfs(
    map: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    start_r: usize,
    start_c: usize,
    plant_type: char
) -> (Vec<(usize, usize)>, Vec<(usize, usize)>) {
    let mut region = Vec::new();
    let mut region_visited = Vec::new();
    let mut queue = VecDeque::new();

    queue.push_back((start_r, start_c));

    while let Some((r, c)) = queue.pop_front() {
        if r >= map.len() || c >= map[0].len() ||
            visited[r][c] || map[r][c] != plant_type {
            continue;
        }

        visited[r][c] = true;
        region.push((r, c));
        region_visited.push((r, c));

        let directions = [(0,1), (1,0), (0,-1), (-1,0)];
        for (dr, dc) in directions {
            let nr = r as i32 + dr;
            let nc = c as i32 + dc;

            if nr >= 0 && nc >= 0 &&
                nr < map.len() as i32 && nc < map[0].len() as i32 &&
                map[nr as usize][nc as usize] == plant_type &&
                !visited[nr as usize][nc as usize] {
                queue.push_back((nr as usize, nc as usize));
            }
        }
    }

    (region, region_visited)
}

fn calculate_region_price(region_plots: &Vec<(usize, usize)>, map: &Vec<Vec<char>>) -> usize {
    let rows = map.len();
    let cols = map[0].len();
    let plant_type = map[region_plots[0].0][region_plots[0].1];

    let area = region_plots.len();
    let mut perimeter = 0;

    // Check each plot in the region
    for (r, c) in region_plots {
        let directions = [(0,1), (1,0), (0,-1), (-1,0)];

        for (dr, dc) in directions {
            let nr = *r as i32 + dr;
            let nc = *c as i32 + dc;

            // Check if this side is on the map boundary or touches a different plant type
            if nr < 0 || nc < 0 ||
                nr >= rows as i32 || nc >= cols as i32 ||
                map[nr as usize][nc as usize] != plant_type {
                perimeter += 1;
            }
        }
    }

    area * perimeter
}

fn solve_garden_plot_problem(input: &str) -> usize {
    let map: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let regions = find_regions(&map);

    regions.iter()
        .flat_map(|(_, plant_regions)|
            plant_regions.iter().map(|region_plots|
                calculate_region_price(region_plots, &map)
            )
        )
        .sum()
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn test_small_example() {
        let input = "AAAA
BBCD
BBCC
EEEC";

        let map: Vec<Vec<char>> = input
            .lines()
            .map(|line| line.chars().collect())
            .collect();

        let regions = find_regions(&map);

        // Check number of distinct regions
        assert_eq!(regions.len(), 5);

        // Check regions of different plant types
        assert!(regions.contains_key(&'A'));
        assert!(regions.contains_key(&'B'));
        assert!(regions.contains_key(&'C'));
        assert!(regions.contains_key(&'D'));
        assert!(regions.contains_key(&'E'));

        // Verify region sizes
        assert_eq!(regions[&'A'][0].len(), 4);
        assert_eq!(regions[&'B'][0].len(), 4);
        assert_eq!(regions[&'C'][0].len(), 4);
        assert_eq!(regions[&'D'][0].len(), 1);
        assert_eq!(regions[&'E'][0].len(), 3);
    }

    #[test]
    fn test_nested_regions() {
        let input = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO";

        let total_price = solve_garden_plot_problem(input);
        assert_eq!(total_price, 772);
    }

    #[test]
    fn test_large_example() {
        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

        let total_price = solve_garden_plot_problem(input);
        assert_eq!(total_price, 1930);
    }

    #[test]
    fn test_region_price_calculation() {
        let input = "AAAA
BBCD
BBCC
EEEC";

        let map: Vec<Vec<char>> = input
            .lines()
            .map(|line| line.chars().collect())
            .collect();

        let regions = find_regions(&map);

        // Manually check prices for specific regions
        let a_price = calculate_region_price(&regions[&'A'][0], &map);
        let b_price = calculate_region_price(&regions[&'B'][0], &map);
        let d_price = calculate_region_price(&regions[&'D'][0], &map);

        assert_eq!(a_price, 40);  // 4 * 10
        assert_eq!(b_price, 32);  // 4 * 8
        assert_eq!(d_price, 4);   // 1 * 4
    }

    #[test]
    fn test_single_region() {
        let input = "XXXXX
XXXXX
XXXXX
XXXXX
XXXXX";

        let total_price = solve_garden_plot_problem(input);
        assert_eq!(total_price, 500);  // 25 * 20
    }

    #[test]
    fn final_solve() {
        let input: String = fs::read_to_string("input/12.txt").unwrap();
        let total_price = solve_garden_plot_problem(input.as_str());
        assert_eq!(total_price, 0);
    }
}