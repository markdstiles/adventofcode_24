//https://adventofcode.com/2024/day/12

use std::{collections::HashSet, fs::File, hash::Hash, io::{BufRead, BufReader}, ops::{Add, AddAssign}};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Point<T> {
        Point {
            x,
            y,
        }
    }
}

impl From<Point<i32>> for Point<usize> {
    fn from(value: Point<i32>) -> Self {
        Point {
            x: value.x.try_into().unwrap(),
            y: value.y.try_into().unwrap(),
        }
    }
}

impl From<Point<usize>> for Point<i32> {
    fn from(value: Point<usize>) -> Self {
        Point {
            x: value.x.try_into().unwrap(),
            y: value.y.try_into().unwrap(),
        }
    }
}

impl<T: Add<Output = T>> Add for Point<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> AddAssign for Point<T>
    where T: AddAssign, {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Edge {
    outside_dir: Point<i32>,
    plot: Point<usize>,
}

#[derive(Clone, Debug, PartialEq)]
struct Region {
    plant: char,
    plots: Vec<Point<usize>>,
    perimeter: Vec<Edge>,
}

impl Region {
    fn new(plant: char) -> Region {
        Region {
            plant,
            plots: vec![],
            perimeter: vec![],
        }
    }
    fn area(&self) -> u32 {
        self.plots.len() as u32
    }

    fn perimeter_len(&self) -> u32 {
        self.perimeter.len() as u32
    }

    fn number_of_sides(&mut self) -> u32 {
        let directions = vec![
            Point::new(0, -1),  //Up
            Point::new(0, 1),   //down
            Point::new(-1, 0),  //left
            Point::new(1, 0),   //right
        ];

        let mut total_sides = 0;
        //Prevent sorting when not required
        let mut sorted_by_y = false;
        self.perimeter.sort_by_key(|e| e.plot.x);

        for direction in directions {

            let mut previous_edge: Option<Point<usize>> = None;
            let mut is_side_x = false;
            let mut is_side_y = false;

            //Sort by x or y depending on direction
            if direction.x == 0 && !sorted_by_y {
                self.perimeter.sort_by_key(|e| e.plot.y);
                sorted_by_y = true;
            } else if direction.y == 0 && sorted_by_y {
                self.perimeter.sort_by_key(|e| e.plot.x);
                sorted_by_y = false;
            }

            let mut sides = 0;
            self.perimeter.iter()
                .filter(|p| p.outside_dir == direction)
                .for_each(|p| {

                    if let Some(previous_edge) = previous_edge {
                        //If we have a previous edge to compare...
                        //Is the previous edge on the same side as the current edge?
                        if p.plot.x == previous_edge.x && usize::abs_diff(p.plot.y,previous_edge.y) == 1 {
                            if is_side_y {
                                //We've changed sides
                                sides += 1;
                                is_side_y = false;
                            }
                            is_side_x = true;
                        } else if p.plot.y == previous_edge.y && usize::abs_diff(p.plot.x,previous_edge.x) == 1 {
                            if is_side_x {
                                //We've changed sides
                                sides += 1;
                                is_side_x = false;
                            }
                            is_side_y = true;
                        } else if is_side_x || is_side_y {
                            //We were processing a side but came to the end of it
                            sides += 1;
                            is_side_x = false;
                            is_side_y = false;
                        } else {
                            //Previous edge was a single side
                            sides += 1;
                        }
                    }
                    previous_edge = Some(p.plot);
                });
                //Add the last side
                sides += 1;
                total_sides += sides;
        };

        total_sides
    }

    fn cost_by_permeter_len(&self) -> u32 {
        self.area() * self.perimeter_len()
    }

    fn cost_by_side(&mut self) -> u32 {
        self.area() * self.number_of_sides()
    }
}

struct Farm {
    map: Vec<Vec<char>>,
    width: usize,
    height: usize,
    regions: Vec<Region>,
    directions: Vec<Point<i32>>,
}

impl Farm {
    fn new(map: Vec<Vec<char>>) -> Farm {
        let width = map[0].len();
        let height = map.len();

        Farm {
            map,
            width,
            height,
            regions: Vec::new(),
            directions: vec![
                Point::new(0, -1),  //Up
                Point::new(0, 1),   //down
                Point::new(-1, 0),  //left
                Point::new(1, 0),   //right
            ],
        }
    }

    fn find_regions(&mut self) {
        //Map will be filled in as we discover regions
        let mut y = 0;
        while y < self.height{
            if let Some((x, _)) = self.map[y].iter().enumerate().find(|(_, &c)| c != '.') {
                let region = self.scan_region(Point::new(x, y));
                self.regions.push(region);
                self.fill_region('.', self.regions.len()-1);
            } else {
                y += 1;
            }
        }
    }

    fn scan_region(&mut self, start_point: Point<usize>) -> Region {
        let mut region = Region::new(self.map[start_point.y][start_point.x]);

        let mut visited: HashSet<Point<i32>> = HashSet::new();
        self.probe_point(start_point.into(), &mut region, &mut visited);

        region
    }

    fn probe_point(&self, point: Point<i32>, region: &mut Region, visited: &mut HashSet<Point<i32>>) {
        if visited.contains(&point) {
            return
        }

        visited.insert(point);

        if !self.inside(region.plant, point) {
            return
        } else {
            region.plots.push(point.into());
        }

        //Look for connected plots, ignoring those that have already been visited
        for &direction in &self.directions {
            let target = point + direction; 

            if !self.inside(region.plant, target) {
                //Add to edges
                region.perimeter.push(Edge { plot: point.into(), outside_dir: direction });
                visited.insert(target);
            } else {
                self.probe_point(target, region, visited);
            }
        }
    }
    
    fn inside(&self, plant: char, location: Point<i32>) -> bool {
        if !(location.x >= 0 && location.x < self.width.try_into().unwrap() && location.y >= 0 && location.y < self.height.try_into().unwrap()) {
            //Outside map bounds
            return false
        } else if self.map[location.y as usize][location.x as usize] != plant {
            return false
        }

        true
    }

    fn fill_region(&mut self, new_plant: char, region_idx: usize) {
        for plot in &self.regions[region_idx].plots {
            self.map[plot.y][plot.x] = new_plant;
        }
    }
}

pub fn do_part1() -> anyhow::Result<i64> {
    println!("Day 12 - Part 1:");
    
    let mut input_file = std::env::current_dir()?;
    input_file.push("input\\day12.txt");

    println!("Reading input from {}", input_file.display());

    let file = File::open(input_file.clone())?;
    let reader = BufReader::new(file);

    let mut farm = Farm::new(reader.lines()
        .map(|line| 
            line.unwrap()
                .chars()
                .collect::<Vec<char>>()
            )
        .collect()
    );

    farm.find_regions();

    let total_cost = farm.regions.iter()
        .fold(0, |total, r| total + r.cost_by_permeter_len());

    Ok(total_cost as i64)
}

pub fn do_part2() -> anyhow::Result<i64> {
    println!("Day 12 - Part 2:");
    
    let mut input_file = std::env::current_dir()?;
    input_file.push("input\\day12.txt");

    println!("Reading input from {}", input_file.display());

    let file = File::open(input_file.clone())?;
    let reader = BufReader::new(file);
   
    let mut farm = Farm::new(reader.lines()
    .map(|line| 
        line.unwrap()
            .chars()
            .collect::<Vec<char>>()
        )
        .collect()
    );

    farm.find_regions();

    let total_cost = farm.regions.iter_mut()
        .fold(0, |total, r| total + r.cost_by_side());

    Ok(total_cost as i64)
}