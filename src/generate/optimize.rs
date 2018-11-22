extern crate rand;
use generate::optimize::rand::prelude::*;

use generate::astar::*;
use generate::parkinglot::*;



pub fn score_lot(lot: &ParkingLot) -> i32 {
    let mut score = 0;

    let spaces = filter_crowded_lot(&lot)
                    .get_spaces()
                    .len() as i32;
    let average_distance_from_exits = 
        average_distance_from_exits(&lot);

    for exit in lot.get_exits() {
        let Pos(x, y) = exit;

        if lot.surrounding_walls(x, y) > 2 {
            score -= 5;
        }
    }

    score += spaces*2;
    score -= average_distance_from_exits;

    // println!("Spaces: {}\nAverage Distance: {}",
    //             spaces,
    //             average_distance_from_exits);

    return score;
}



pub fn optimize(lot: &ParkingLot) -> ParkingLot {
    let mut new_lot = lot.clone();
    let mut best_lot = lot.clone();

    let mut score = 0;

    let mut no_improvement_counter = 0;

    let stagnation_limit = 3;
    loop {
        new_lot = filter_crowded_lot(&new_lot);
        new_lot = batch_optimize(new_lot.clone());
        new_lot = step_optimize(new_lot.clone());
        let newest_score = score_lot(&new_lot);

        if newest_score <= score {
            if no_improvement_counter >= stagnation_limit {
                break;
            }
            no_improvement_counter += 1;
        } else {
            no_improvement_counter = 0;
            score = newest_score;
            best_lot = new_lot.clone();
            // new_lot.show();

        }
    }

    return best_lot;
}


pub fn batch_optimize(lot: ParkingLot) -> ParkingLot {

    let mutation_factor = 5;


    let mut rng = thread_rng();


    let mut new_lot = lot.clone();
    let mut test_lot = lot.clone();


    for x in 0..new_lot.w {
        for y in 0..new_lot.h {
            let current_position = test_lot.position(x, y);
            
            match current_position {
                EXIT => {continue;}
                SPACE => {
                    if rng.gen_range(0, mutation_factor) == 0 {
                        test_lot.set_at(ROAD, x, y);
                        }
                    }
                ROAD => {
                    if rng.gen_range(0, mutation_factor) == 0 {
                        test_lot.set_at(SPACE, x, y);
                        }
                    }
                _ => {}
            }

        }
    }

    if score_lot(&test_lot) >= score_lot(&new_lot) {
        new_lot = test_lot;
    }

    new_lot = make_symmetrical_vertical(&new_lot);

    return new_lot;
}


pub fn step_optimize(lot: ParkingLot) -> ParkingLot {

    let mut new_lot = lot.clone();
    let mut test_lot: ParkingLot;


    for x in 0..new_lot.w {
        for y in 0..new_lot.h {

            if new_lot.surrounding_walls(x, y) > 3 {
                continue;
            }


            test_lot = new_lot.clone();

            let current_position = test_lot.position(x, y);
            
            match current_position {
                EXIT => {continue;}
                SPACE => {
                    test_lot.set_at(ROAD, x, y);
                    }
                ROAD => {
                    test_lot.set_at(SPACE, x, y);
                    }
                _ => {}
            }

            if score_lot(&test_lot) >= score_lot(&new_lot) {
                // new_lot.set(test_lot.lot.clone());
                new_lot = test_lot;
            }
        }
    }
    new_lot = make_symmetrical_vertical(&new_lot);

    return new_lot;
}


pub fn make_symmetrical_vertical(lot: &ParkingLot) -> ParkingLot {
    let mut new_lot = lot.clone();

    // mirror first half
    for x in 0..(lot.w/2) {
        for y in 0..lot.h {
            let to_copy = new_lot.position(x, y);

            new_lot.set_at(to_copy,
                lot.w-x-1,
                y
                );
        }
    }

    if score_lot(&new_lot) > score_lot(&lot) {
        return new_lot;
    }

    // failed, so mirror second half
    let mut new_lot = lot.clone();    
    for x in (lot.w/2)..lot.w {
        for y in 0..lot.h {
            let to_copy = new_lot.position(x, y);

            new_lot.set_at(to_copy,
                lot.w-x-1,
                y
                );
        }
    }

    return match new_lot.get_exits() == lot.get_exits() {
        true => new_lot.clone(),
        false => lot.clone()
    };
}


pub fn average_distance_from_exits(lot: &ParkingLot) -> i32 {
    let mut average_distance = 0;
    let mut total_distance = 0;
    let mut i = 0;

    let new_lot = lot.clone();
    for y in 0..lot.h {
        for x in 0..lot.w {
            if lot.position(x, y) == EXIT {
                continue;
            }

            for exit in new_lot.get_exits() {
                let Pos(ex, ey) = exit;
                match get_path(Pos(x, y), exit, &new_lot) {
                    Some(path) => {
                        i += 1;
                        total_distance += path.len() as i32;
                        average_distance = total_distance / i;
                    }
                    None => {
                        i += 1;
                        total_distance += (((
                                (ex-x).pow(2) + (ey-y).pow(2)
                            ) as f32).powf(0.5)) as i32;
                        average_distance = total_distance / i;
                    }
                }
            }
        }
    }

    return average_distance;
}


#[allow(dead_code)]
pub fn random_lot(exits: Vec<Pos>, w: i32, h: i32) -> ParkingLot {
    let mut rng = thread_rng();

    let mut lot = ParkingLot::new(w, h);

    let mut result: Vec<Vec<i32>> = vec![];

    for y in 0..h {
        let mut row = vec![];
        for x in 0..w {
            if exits.contains(&Pos(x, y)) {
                row.push(2);
            } else {
                row.push(rng.gen_range(0, 2));
            }
        }
        result.push(row);
    }

    lot.set(result);

    return lot;
}


#[allow(dead_code)]
pub fn filter_crowded_lot(lot: &ParkingLot) -> ParkingLot {
    let mut new_lot = lot.clone();

    for y in 0..lot.h {
        for x in 0..lot.w {

            if lot.position(x, y) == EXIT {
                continue;
            }
            
            // let number_of_surrounding_walls = lot.surrounding_walls(x, y);

            // if number_of_surrounding_walls > 3 && lot.position(x, y) != EXIT {
            //     new_lot.set_at(ROAD, x, y);
            // }
            // println!("Surrounding Walls at ({}, {}): {}", x, y, number_of_surrounding_walls);
            let mut available_exits = 0;

            for exit in new_lot.get_exits() {
                match get_path(Pos(x, y), exit, &new_lot) {
                    Some(_) => {available_exits += 1;}
                    None => {}
                }
            }

            if available_exits == 0 {
                new_lot.set_at(ROAD, x, y);
            }

        }
    }

    return new_lot;
}
