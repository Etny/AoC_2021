mod input;
use input::*;

fn main() {
    let mut dots: Vec<(i32, i32)> = vec![];

    for line in INPUT_DOTS.trim().lines() {
        let coords: Vec<_> = line.trim().split(',').collect();
        dots.push((coords[0].parse().unwrap(), coords[1].parse().unwrap()));
    }

    for fold_line in INPUT_FOLDS.trim().lines() {
        let fold_dec = fold_line
            .split(' ')
            .skip(2)
            .next()
            .unwrap()
            .split('=')
            .collect::<Vec<_>>();
        fold(&mut dots, fold_dec[0] == "x", fold_dec[1].parse().unwrap());
    }

    let width = dots.iter().max_by_key(|d| d.0).unwrap().0;
    let height = dots.iter().max_by_key(|d| d.1).unwrap().1;

    for y in 0..=height {
        for x in 0..=width {
            print!("{}", if dots.contains(&(x, y)) { '#' } else { ' ' })
        }
        println!();
    }
}

fn fold(dots: &mut Vec<(i32, i32)>, fold_x: bool, fold_line: i32) {
    let mut dots_new = dots
        .into_iter()
        .map(|dot| {
            if fold_x {
                if dot.0 <= fold_line {
                    *dot
                } else {
                    (fold_line - (dot.0 - fold_line), dot.1)
                }
            } else {
                if dot.1 <= fold_line {
                    *dot
                } else {
                    (dot.0, fold_line - (dot.1 - fold_line))
                }
            }
        })
        .collect::<Vec<_>>();

    dots_new.sort();
    dots_new.dedup();

    *dots = dots_new;
}
