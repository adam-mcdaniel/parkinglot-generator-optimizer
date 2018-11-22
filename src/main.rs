use std::env;

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
    // get command-line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 5 {
        println!("Usage: './parkinglot WIDTH HEIGHT Y_POSITION_OF_EXITS_ON_LEFT&RIGHT PARKING_LOTS_TO_TEST'");
        return Ok(());
    }


    let w = args[1].parse::<i32>().expect("Width of lot must be a 32 bit integer.");
    let h = args[2].parse::<i32>().expect("Height of lot must be a 32 bit integer.");
    let exit_y_pos = args[3].parse::<i32>().expect("Exit Y position must be a 32 bit integer.") - 1;
    let number_to_generate = args[4].parse::<i32>().expect("Number of parkinglots to generate must be a 32 bit integer.");


    println!("
Road:  ' '
Space: '#'
Exit:  '@'
");



    println!("\nOptimize!\n\n\n");

    // initialize score variable, will track the best score
    let mut score = 0;
    // initialize new_score variable, will track the current lot's score
    let mut new_score = 0;
    // initialize best_lot variable, contains the best lot
    let mut best_lot = ParkingLot::new(w, h);

    // initialize a growable array named threads on the heap
    // this will store the handlers for all the threads
    let mut threads = vec![];

    // start the threads and push their handlers to the "threads" vector
    for j in 0..number_to_generate {

        // start a thread using a closure to generate and optimize
        // a randomly generated parkinglot
        // return the optimized lot by value
        threads.push(thread::spawn(move || {
            println!("thread {} started", j+1);
            let mut lot = random_lot(
                vec![Pos(0, exit_y_pos), Pos(w-1, exit_y_pos)],
                w, h
                );
            lot = make_symmetrical_vertical(&lot);
            return optimize(&lot);
        }));
    }

    // initialize counter that counts the finished threads
    let mut i = 0;
    for t in threads {
        // initialize counter that counts the finished threads
        i += 1;

        // block main thread until thread is finished
        // get the lot returned by the aforementioned closure
        let mut lot = t.join().unwrap();

        // notify user thread finished
        println!("thread {} done", i);

        // store score of lot
        new_score = score_lot(&lot);

        // compare new score to the best score
        if new_score >= score {
            score = new_score;
            best_lot = lot;
        }

        // write lot to file
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("output/designs.txt").unwrap();

        file.write_all(best_lot.to_str().as_bytes())?;
        file.flush()?;
        // flush buffer just in case there's
        // a panic and file doesn't get written to
    }


    // Show best lot
    println!("And the best lot is:");
    best_lot.show();

    // return 0, main thread finished with 0 errors
    return Ok(());
}
