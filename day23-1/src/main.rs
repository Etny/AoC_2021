
use self::PodType::*;

//Gonna do this one later
fn main() {

    let mut pods = vec![Pod::new(B, 0, 0), Pod::new(A, 0, 1), Pod::new(C, 1, 0), Pod::new(D, 1, 1), Pod::new(B, 2, 0), Pod::new(C, 2, 1), Pod::new(D, 3, 0), Pod::new(A, 3, 1)];
    let moves = solve(&mut pods);

    println!("{}", calc_moves(3, 1, 4, 9));

    // print(&pods);
    println!("{}", moves);
}

fn solve(pods: &mut Vec<Pod>) -> u32 {

    let mut moves = 0;

    pods.sort_by_key(|p| p.pos);

    let to_solve = [D, C, B, A];

    for solve in to_solve {
        for i in 0..pods.len() {
            let pod = pods.len() - 1 - i;
            if pods[pod].typ != solve { continue; }

            if let Some(target) = pods[pod].typ.next_target(&pods) {
                if target.0 == pods[pod].pos.0 && target.1 == 0 { continue; }

                moves += move_to(pods.len() - 1 - i, target.0, target.1, pods);
            }
        }

       
    }

    for i in 0..pods.len() {
        if pods[i].pos.0 != 4 { continue; }

        if let Some(target) = pods[i].typ.next_target(pods) {
            moves += move_to(i, target.0, target.1, pods);
        }
    }

    moves
}

fn move_to(pod: usize, room: usize, index: usize, pods: &mut Vec<Pod>) -> u32{
    // println!("After move {:?} -> {:?}:", pods[pod].pos, (room, index));

    let mut res = 0;

    if let Some(p) = pod_at(room, index, pods) {
        let idx = if room >= pods[pod].pos.0 { room_hallway_index(room) + 1 } else { room_hallway_index(room) - 1};

        res += move_into_hallway(p, idx, pods);
    }

    if pods[pod].pos.1 == 1 {
        if let Some(p) = pod_at(pods[pod].pos.0, 0, pods) {
            let idx = if room >= pods[pod].pos.0 { room_hallway_index(pods[p].pos.0) - 1 } else { room_hallway_index(pods[p].pos.0) + 1};

            res += move_into_hallway(p, idx, pods);
        }
    }

    res += calc_moves(pods[pod].pos.0, pods[pod].pos.1, room, index) * pods[pod].typ.move_cost();
    
    pods[pod].pos = (room, index);
    print(pods);
    res
}

fn move_into_hallway(pod: usize, index: usize, pods: &mut Vec<Pod>) -> u32{
    let mut res = 0;
    if pods[pod].pos.1 == 1 {
        if let Some(p) = pod_at(pods[pod].pos.0, 0, pods) {
            let idx = match room_hallway_index(pods[pod].pos.0).cmp(&index) {
                std::cmp::Ordering::Less => index - 2,
                std::cmp::Ordering::Greater => index + 2,
                _ => panic!()
            };
            res += move_into_hallway(p, idx, pods);
        }
    }

    res += calc_moves(pods[pod].pos.0, pods[pod].pos.1, 4, index) * pods[pod].typ.move_cost();

    pods[pod].pos = (4, index);
    print(pods);

    res
}

fn calc_moves(src_r: usize, src_i: usize, dest_r: usize, dest_i: usize) -> u32 {
    let mut ret: usize;

    if dest_r == 4 {
        ret = src_i + 2; 
    } else {
        if src_r == 4 {
            ret = (src_i as i32 - room_hallway_index(dest_r) as i32).abs() as usize;
            ret += 1 + dest_i;
        } else {
            ret = (room_hallway_index(src_r) as i32 - room_hallway_index(dest_r) as i32).abs() as usize;
            ret += 2 + src_i + dest_i;
        }
    }

    ret as u32
}

fn room_hallway_index(room: usize) -> usize {
    2 + (2 * room)
}

fn pod_at(room: usize, index: usize, pods: &Vec<Pod>) -> Option<usize> {
    (0..pods.len()).filter(|p| pods[*p].pos == (room, index)).next()
}

fn type_at(room: usize, index: usize, pods: &Vec<Pod>) -> Option<PodType> {
    match pod_at(room, index, pods) {
        Some(p) => Some(pods[p].typ),
        None => None
    }
}

fn print(pods: &Vec<Pod>) {
    let hallway: String = (0..11).map(|i| if let Some(p) = pod_at(4, i, pods) { pods[p].typ.to_string() } else { ".".to_owned() } ).collect();
    println!("{}", hallway);

    println!();

    let top: String = (0..=3).map(|i| if let Some(p) = pod_at(i, 0, pods) { pods[p].typ.to_string() } else { " ".to_owned() }).collect();
    println!("  {}  ", top.chars().flat_map(|i| [i, ' ']).collect::<String>());

    let bot: String = (0..=3).map(|i| if let Some(p) = pod_at(i, 1, pods) { pods[p].typ.to_string() } else { " ".to_owned() }).collect();
    println!("  {}  ", bot.chars().flat_map(|i| [i, ' ']).collect::<String>());

    println!("-----------------");
    println!();
}

#[derive(Clone)]
struct Pod {
    typ: PodType,
    pub pos: (usize, usize)
}

impl Pod {
    fn new(typ: PodType, r: usize, i: usize) -> Self {
        Self { typ, pos: (r, i) }
    }
}


#[derive(Clone, Copy, PartialEq, Eq)]
enum PodType {
    A, B, C, D
}

impl PodType {
    fn dest(&self) -> usize {
        match *self {
            A => 0,
            B => 1,
            C => 2,
            D => 3
        }
    } 

    fn next_target(&self, pods: &Vec<Pod>) -> Option<(usize, usize)> {
        if let Some(a) = type_at(self.dest(), 1, pods) {
            if a == *self {
                if let Some(a) = type_at(self.dest(), 0, pods) {
                    if a == *self {
                        None
                    } else {
                        Some((self.dest(), 0))
                    }
                } else {
                    Some((self.dest(), 0))
                }
            } else {
                Some((self.dest(), 1))
            }
        } else {
            Some((self.dest(), 1))
        }

    }

    fn move_cost(&self) -> u32 {
        match *self {
            A => 1,
            B => 10,
            C => 100,
            D => 1000
        }
    }

    fn to_string(&self) -> String {
        match *self {
            A => "A".to_owned(),
            B => "B".to_owned(),
            C => "C".to_owned(),
            D => "D".to_owned()
        }
    }
}