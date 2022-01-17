fn main() {
    let mut p1 = (9, 0);
    let mut p2 = (7, 0);

    let mut die = (1..=100).cycle();
    let mut rolls = 0;

    while p1.1 < 1000 && p2.0 < 1000 {
        update_player(&mut p1, &mut die, &mut rolls);
        if p1.1 >= 1000 { break; }
        update_player(&mut p2, &mut die, &mut rolls);
    }

    println!("{}", p1.1.min(p2.1) * rolls);
}

fn update_player<T>(p: &mut (u32, u32), die: &mut T, rolls: &mut u32) 
    where T: Iterator<Item=u32>
{
    let roll: u32 = die.take(3).sum();
    let pos = (p.0 + roll) % 10;
    *p = (pos, p.1 + 1 + pos);
    *rolls += 3;
} 
