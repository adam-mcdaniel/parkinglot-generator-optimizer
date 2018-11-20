use generate::optimize::*;


pub const ROAD:  i32 = 0;
pub const SPACE: i32 = 1;
pub const EXIT:  i32 = 2;
pub const OUT_OF_BOUNDS: i32 = 3;



#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Pos(pub i32, pub i32);

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ParkingLot {
    pub w: i32,
    pub h: i32,
    pub lot: Vec<Vec<i32>>
}


#[allow(dead_code)]
impl ParkingLot {
    pub fn new(w: i32, h: i32) -> Self {
        let lot = vec![vec![0; w as usize]; h as usize];

        ParkingLot {
            w,
            h,
            lot
        }
    }


    pub fn position(&self, x: i32, y: i32) -> i32 {
        if (x < self.w && y < self.h) && (x >= 0 && y >= 0) {
            return self.lot[y as usize][x as usize];
        } else {
            return OUT_OF_BOUNDS;
        }

    }


    pub fn set_at(&mut self, i: i32, x: i32, y: i32) {
        if (x < self.w && y < self.h) && (x >= 0 && y >= 0) {
            self.lot[y as usize][x as usize] = i;
        }
    }


    pub fn get_exits(&self) -> Vec<Pos> {
        let mut exits: Vec<Pos> = vec![];


        for x in 0..self.w {
            for y in 0..self.h {

                match self.position(x, y) {
                    EXIT  => {
                        exits.push(Pos(x, y));
                    },
                    _      => {}
                };
            }
        }

        return exits;
    }

    pub fn get_spaces(&self) -> Vec<Pos> {
        let mut spaces: Vec<Pos> = vec![];

        for x in 0..self.w {
            for y in 0..self.h {

                match self.position(x, y) {
                    SPACE  => {
                        spaces.push(Pos(x, y));
                    },
                    _      => {}
                };
            }
        }

        return spaces;
    }


    pub fn surrounding_walls(&self, x: i32, y: i32) -> i32 {
        let mut i = 0;

        i += match self.position(x+1, y) {
            SPACE => 1,
            OUT_OF_BOUNDS => 1,
            _ => 0,
        };
        i += match self.position(x-1, y) {
            SPACE => 1,
            OUT_OF_BOUNDS => 1,
            _ => 0,
        };
        i += match self.position(x, y+1) {
            SPACE => 1,
            OUT_OF_BOUNDS => 1,
            _ => 0,
        };
        i += match self.position(x, y-1) {
            SPACE => 1,
            OUT_OF_BOUNDS => 1,
            _ => 0,
        };
        return i;
    }


    pub fn set(&mut self, new_lot: Vec<Vec<i32>>) {
        self.h = new_lot.len() as i32;
        self.w = new_lot[0].len() as i32;

        self.lot = new_lot;
    }


    pub fn show(&self) {

        println!("+{}+", "-".repeat(self.w as usize));

        
        for row in &self.lot {
            print!("|");
            for column in row {
                print!("{}",
                    match column.clone() {
                        ROAD  => " ",
                        SPACE => "#",
                        EXIT  => "@",
                        _     => " "
                    });
            }
            println!("|");
        }
        println!("+{}+", "-".repeat(self.w as usize));

        println!("Score: {}", score_lot(&self));
    }



    pub fn to_str(&self) -> String {
        let mut result = "".to_owned();
        result += &format!("+{}+\n", "-".repeat(self.w as usize));
        
        for row in &self.lot {
            result += "|";
            for column in row {
                result += match column.clone() {
                        ROAD  => " ",
                        SPACE => "#",
                        EXIT  => "@",
                        _     => " "
                    };
            }
            // println!("|");
            result += "|\n";
        }

        result += &format!("+{}+\n", "-".repeat(self.w as usize));
        result += &format!("Score: {}\n\n", score_lot(&self));
        return result.to_string();
    }
}