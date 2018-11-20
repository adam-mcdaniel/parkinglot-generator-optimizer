use std::fs::OpenOptions;
use std::io::prelude::*;
use std::thread;


mod generate;

#[allow(unused_imports)]
use generate::astar::*;
use generate::optimize::*;
use generate::parkinglot::*;


#[allow(unused_assignments)]
fn main() -> std::io::Result<()> {

    let number_to_generate = 10;

    println!("
Road:  ' '
Space: '#'
Exit:  '@'
");

    let w = 15;
    let h = 6;



    println!("\nOptimize!\n\n\n");

    let mut score = 0;
    let mut new_score = 0;
    let mut best_lot = ParkingLot::new(w, h);

    let mut threads = vec![];

    for _ in 0..number_to_generate {
        threads.push(thread::spawn(move || {
            let mut lot = random_lot(vec![Pos(0, 1), Pos(w-1, 1)], w, h);
            lot = make_symmetrical_vertical(&lot);
            return optimize(&lot);
        }));
    }

    let mut i = 0;
    for t in threads {
        i += 1;
        let mut lot = t.join().unwrap();
        println!("thread {} done", i);

        new_score = score_lot(&lot);

        if new_score >= score {
            score = new_score;
            best_lot = lot;

            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open("threaded_designs.txt").unwrap();

            file.write_all(best_lot.to_str().as_bytes())?;
            file.flush()?;
        }
    }



    println!("And the best lot is:");
    best_lot.show();


    return Ok(());
}
