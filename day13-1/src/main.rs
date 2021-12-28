mod input;
use input::*;

fn main() {
    let mut dots: Vec<(i32, i32)> = vec![];

    for line in INPUT_DOTS.trim().lines() {
        let coords: Vec<_> = line.trim().split(',').collect();
        dots.push((coords[0].parse().unwrap(), coords[1].parse().unwrap()));
    }

    for fold_line in INPUT_FOLDS.trim().lines() {
        let fold_dec = fold_line.split(' ').skip(2).next().unwrap().split('=').collect::<Vec<_>>();
        fold(&mut dots, fold_dec[0] == "x", fold_dec[1].parse().unwrap());
    }

    println!("{}", dots.len());
}

fn fold(dots: &mut Vec<(i32, i32)>, fold_x: bool, fold_val: i32) {
    let mut dots_new = dots
        .into_iter()
        .map(|dot| {
            if fold_x {
                if dot.0 <= fold_val {
                    *dot
                } else {
                    (fold_val - (dot.0 - fold_val), dot.1)
                }
            } else {
                if dot.1 <= fold_val {
                    *dot
                } else {
                    (dot.0, fold_val - (dot.1 - fold_val))
                }
            }
        })
        .collect::<Vec<_>>();

    dots_new.sort();
    dots_new.dedup();

    *dots = dots_new;
}
