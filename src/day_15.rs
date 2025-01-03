//https://adventofcode.com/2024/day/15

use crate::misc_types::Point;
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

    let mut entities: Vec<Entity> = Vec::new();  
    let mut boxes_to_push: Vec<(usize, Point<i32>)> = Vec::new();
    let mut boxes_to_collision_test: Vec<(usize, Point<i32>)> = Vec::new();
    let mut robot: usize = 0;

    //Robot is still one cell wide, however walls and boxes are now double the width
    //Boxes move a cell at a time and now can overlap other boxes
    //When the robot pushes an overlapped box, both need to move!
    //Entities origin cell will be on its left side

    let mut debug_iteration = 0;

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

                //Double the spacing on the x axis
                let entity = Entity::new(Point::new((x * 2) as i32, y as i32), entity_type);
                entities.push(entity);

                if entity.entity_type == EntityType::Robot {
                    robot = entities.len() - 1;
                }
            }
        } else if !line.trim().is_empty() {
            //Apply the robot's movements to the entities (speed is still one cell at a time)
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
                    let mut can_move = true;
                    //Look for objects that the robot would collide with once its new position is calculated
                    //If its a box, apply the same logic to the box and any other boxes it would also collide with
                    //If its a wall, we stop - nothing can move
                    
                    //Unlike part 1, we now need to cater for multiple collisions
                    //The robot is still 1 cell in width, it can only collide with one entity - find if it will collide with a wall or box
                    //If we have a box to collision test, find any boxes it collides with, also perform collision tests on those until we hit a wall
                    //or we have free space in to which all the boxes can move

                    if let Some((idx, collided_entity)) = entities.iter()
                        .enumerate()
                        .find(|(idx, e)| *idx != robot 
                        && (
                            //Does the cell the robot is moving into collide with any part of an entity covering that cell
                            robot_new_pos.x >= e.position.x && robot_new_pos.x <= e.position.x + 1
                            && robot_new_pos.y == e.position.y    //Height is still only 1 cell so they have to be in the same row
                        )) {
                        match collided_entity.entity_type {
                            EntityType::Box => {
                                let new_pos = collided_entity.position + direction;
                                boxes_to_collision_test.push((idx, new_pos));
                                boxes_to_push.push((idx, new_pos));
                            },
                            EntityType::Wall => can_move = false,
                            EntityType::Robot => panic!("New position shouldn't be the same as the robots current position."),
                        }
                    }

                    if can_move && !boxes_to_collision_test.is_empty() {
                        'outer: while let Some((box_idx, box_new_pos)) = boxes_to_collision_test.pop() {
                            for (idx, collided_entity) in entities.iter()
                                .enumerate()
                                .filter(|(idx, e)| *idx != box_idx //Excluding itself
                                && *idx != robot    //Or the robot
                                && (
                                    //Check for overlapping boxes/walls, if the origin of the box being moved falls with the bounds of another box or wall...
                                    box_new_pos.x + 1 >= e.position.x && box_new_pos.x <= e.position.x + 1
                                    && box_new_pos.y == e.position.y    //Height is still only 1 cell so they have to be in the same row
                                )) {
                                match collided_entity.entity_type {
                                    EntityType::Box => {
                                        let new_pos = collided_entity.position + direction;
                                        boxes_to_collision_test.push((idx, new_pos));
                                        boxes_to_push.push((idx, new_pos));
                                    },
                                    EntityType::Wall => {
                                        can_move = false;
                                        break 'outer;   //Nothing can move, we would be moving into a wall
                                    },
                                    EntityType::Robot => panic!("New position shouldn't be the same as the robots current position."),
                                }
                            }
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
                        boxes_to_collision_test.clear();
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