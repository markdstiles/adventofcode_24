//https://adventofcode.com/2024/day/12

use std::{collections::HashSet, fs::File, io::{BufRead, BufReader}, ops::{Add, AddAssign}};

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
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

#[derive(Copy, Clone, Debug, PartialEq)]
struct Edge {
    plot: Point<usize>,
    outside_dir: Point<i32>,
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

    fn cost(&self) -> u32 {
        self.area() * self.perimeter_len()
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
        .fold(0, |total, r| total + r.cost());

    /* For debugging...
    farm.regions.iter().for_each(|r| println!("A region of {} plants with price {} * {} = {}", r.plant, r.area(), r.perimeter_len(), r.cost()));

    for region_idx in 0..farm.regions.len() {
        let plant = farm.regions[region_idx].plant;
        farm.fill_region(plant, region_idx);
    }

    println!("Map:")
    farm.map.iter().for_each(|line| println!("{}", line.iter().collect::<String>()));

    println!("Edges:")
    farm.regions.iter().for_each(|r| {
        println!("Region {}", r.plant);
        r.perimeter.iter().for_each(|p| {
            println!("{:?}", p);
        });
    });
    */

    Ok(total_cost as i64)
}

pub fn do_part2() -> anyhow::Result<i64> {
    println!("Day 12 - Part 2:");
    
    let mut input_file = std::env::current_dir()?;
    input_file.push("input\\day12.txt");

    println!("Reading input from {}", input_file.display());

    let file = File::open(input_file.clone())?;
    let reader = BufReader::new(file);
   
    Ok(0)
}