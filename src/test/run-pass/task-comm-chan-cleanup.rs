use std;
import comm;

fn main() {
    let p = comm::Port();
    let c = comm::Chan(p);
    comm::send(c, ~"coffee");
}