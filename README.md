# parkinglot-generator-optimizer
A tool made to generate the best possible geometries for parking lots with specific dimensions and exit locations

## What PLGO does and how it Works

PLGO creates random parkinglots with provided exits and dimensions, and uses multithreading to optimize each for the best flow rate and largest number of parking spots. It takes the current design at each iteration and does incremental improvements until the improvements stagnate, then it makes a large change to the parkinglot and starts the minor incremental improvements over. When the design has peaked, it compares it to the current best lot. If it is equal or better than the current best, it writes it to a save file. When all the threads are handled and terminated, the main thread is terminated.

## How to Install & Run

```
curl https://sh.rustup.rs -sSf | sh #install rust

git clone https://github.com/adam-mcdaniel/parkinglot-generator-optimizer
cd parkinglot-generator-optimizer
cargo build --release

./target/release/parking 10 10 5 10
```
