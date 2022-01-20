mod input;
mod brute;
mod fast;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();

    let do_fast = args.len() < 2 || args[1] == "fast";
    let do_min = args.len() < 3 || args[2] == "min";

    if do_fast {
        fast::solve_fast(do_min);
    } else {
        brute::solve_brute(do_min);
    }
}