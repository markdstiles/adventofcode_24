//https://adventofcode.com/2024/day/15

use crate::misc_types::{Point, Rect};
use std::{fs::File, io::{BufRead, BufReader}};

#[derive(Copy, Clone, Debug, PartialEq)]
enum EntityType {
    Wall,
    Box,
    Robot
}

#[derive(Copy, Clone, Debug)]
struct Entity {
    position: Point<i32>,
    entity_type: EntityType
}

impl Entity {
    fn new(position: Point<i32>, entity_type: EntityType) -> Entity {
        Entity {
            position,
            entity_type,
        }
    }
}

/*
fn get_quadrant(position: Point<i32>, quadrants: &[Rect]) -> Option<usize> {
    quadrants.iter().enumerate().filter(|(_, &q)| q.is_inside(position)).map(|(idx, _)| idx).next()
}
*/

pub fn do_part1() -> anyhow::Result<i64> {
    println!("Day 15 - Part 1:");
    
    let mut input_file = std::env::current_dir()?;
    input_file.push("input\\day15.txt");

    println!("Reading input from {}", input_file.display());

    let file = File::open(input_file.clone())?;
    let reader = BufReader::new(file);

    let mut entities: Vec<Entity> = Vec::new();  
    let mut boxes_to_push: Vec<(usize, Point<i32>)> = Vec::new();  
    let mut robot: usize = 0;

    /* Quad map version
    //Map is split into quarters to improve search/collision detection
    let mut quad_map: Vec<Vec<Entity>> = vec![Vec::new(); 4];
    let mut robot: (usize, usize) = (0, 0);
    let quadrants: Vec<Rect> = vec![
        Rect::new(0, 0, 25, 25),
        Rect::new(25, 0, 50, 25),
        Rect::new(0, 25, 25, 50),
        Rect::new(25, 25, 50, 50),
    ];
    */

    //Parse the map and create a list of entities with their locations
    for (y, line) in reader.lines().enumerate() {
        let line = line?;

        if line.starts_with("#") {
            //Line is part of the map - add the entities
            for (x, c) in line.chars().enumerate() {
                let entity_type = match c {
                    '#' => EntityType::Wall,
                    '@' => EntityType::Robot,
                    'O' => EntityType::Box,
                    _ => continue
                };

                let entity = Entity::new(Point::new(x as i32, y as i32), entity_type);
                entities.push(entity);

                if entity.entity_type == EntityType::Robot {
                    robot = entities.len() - 1;
                }
            }
        } else if !line.trim().is_empty() {
            //Apply the robot's movements to the entities
            for c in line.chars() {
                let direction: Option<Point<i32>> = match c {
                    '<' => Some(Point::new(-1, 0)),
                    '^' => Some(Point::new(0, -1)),
                    '>' => Some(Point::new(1, 0)),
                    'v' => Some(Point::new(0, 1)),
                    _ => None,
                };

                if let Some(direction) = direction {
                    let robot_new_pos = entities[robot].position + direction;
                    let mut new_pos = robot_new_pos;
                    let mut can_move = false;
                    //Look for objects that the robot would collide with once its new position is calculated
                    //If its a box, apply the same logic to the box and any other boxes it would also collide with
                    //If its a wall, we stop - nothing can move
                    loop {
                        //Should only be one entity occupying an x,y location - and none if it is unoccupied
                        if let Some((idx, collided_entity)) = entities.iter()
                            .enumerate()
                            .find(|(idx, e)| e.position == new_pos) {
                            match collided_entity.entity_type {
                                EntityType::Box => {
                                    new_pos = collided_entity.position + direction; //Update new pos - we'll need to check this location too
                                    boxes_to_push.push((idx, new_pos))              //Box can potentially be moved
                                },
                                EntityType::Wall => break,                          //We hit a wall, nothing can move
                                EntityType::Robot => panic!("New position shouldn't be the same as the robots current position."),
                            }
                        } else {
                            //We found a space with nothing in so we can move and also move any boxes we found on the way
                            can_move = true;
                            break;
                        }
                    }

                    if can_move {
                        //Move the robot
                        entities[robot].position = robot_new_pos;

                        //If there are any boxes to push, move them
                        while let Some((idx, new_pos)) = boxes_to_push.pop() {
                            entities[idx].position = new_pos;
                        }
                    } else {
                        //Nothing moves
                        boxes_to_push.clear();
                    }
                }
            }
        }
    }

    //Calculate GPS (Goods Positioning System) coordinates of the boxes = (y * 100) + x
    let total = entities.iter()
        .filter(|&e| e.entity_type == EntityType::Box)
        .map(|e| (e.position.y * 100) + e.position.x)
        .sum::<i32>();

    Ok(total as i64)
}

pub fn do_part2() -> anyhow::Result<i64> {
    println!("Day 15 - Part 2:");
    
    let mut input_file = std::env::current_dir()?;
    input_file.push("input\\day15.txt");

    println!("Reading input from {}", input_file.display());

    let file = File::open(input_file.clone())?;
    let reader = BufReader::new(file);

    Ok(0)
}