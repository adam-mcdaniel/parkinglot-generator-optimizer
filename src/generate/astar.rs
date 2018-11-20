extern crate pathfinding;
#[allow(unused_imports)]
use generate::astar::pathfinding::prelude::bfs;



use generate::parkinglot::*;


impl Pos {
    #[allow(unused_variables, dead_code)]
    pub fn successors(&self, lot: &ParkingLot) -> Vec<Pos> {
        let &Pos(x, y) = self;


        let mut result: Vec<Pos> = vec![];
        match lot.position(x+1, y) {
            ROAD  => result.push(Pos(x+1, y)),
            EXIT  => result.push(Pos(x+1, y)),
            _     => {}
        };
        match lot.position(x, y+1) {
            ROAD  => result.push(Pos(x, y+1)),
            EXIT  => result.push(Pos(x, y+1)),
            _     => {}
        };
        match lot.position(x-1, y) {
            ROAD  => result.push(Pos(x-1, y)),
            EXIT  => result.push(Pos(x-1, y)),
            _     => {}
        };
        match lot.position(x, y-1) {
            ROAD  => result.push(Pos(x, y-1)),
            EXIT  => result.push(Pos(x, y-1)),
            _     => {}
        };

        // println!("Step: {:?}", result);
        return result;
    }
}


pub fn get_path(start: Pos, goal: Pos, lot: &ParkingLot) -> Option<Vec<Pos>> {

    let result = bfs(&start, move |p| { p.successors(&lot) }, |p| *p == goal);

    return result;
}
