
use nalgebra::{Matrix3, Vector3};

type Vec3 = Vector3<f32>;
type Matrix = Matrix3<f32>;

#[derive(Clone)]
pub struct Scanner {
    id: u32,
    other_scanners: Vec<Vec3>,
    pub beacons: Vec<Beacon>,
}

#[derive(Clone)]
pub struct Beacon {
    pos: Vec3,
    rel: Vec<Vec3>,
}

impl Scanner {
    pub fn from_dec<'a, T>(iter: &mut T) -> Self
    where
        T: Iterator<Item = &'a str>,
    {
        let name_line = iter.next().unwrap();
        let id = name_line
            .chars()
            .skip(12)
            .take_while(|n| n.is_numeric() || *n == '-')
            .collect::<String>()
            .parse()
            .unwrap();

        let mut beacons = vec![];

        while let Some(line) = iter.next() {
            if line.replace(" ", "").len() == 0 {
                break;
            }

            let pos = Vec3::from_iterator(line.split(',').map(|n| n.parse().unwrap()));

            beacons.push(Beacon { pos, rel: vec![] })
        }

        Self::set_relative_positions(&mut beacons);

        Scanner {
            id,
            other_scanners: vec![Vec3::from_iterator([0.0,0.0,0.0])],
            beacons,
        }
    }

    pub fn compare(&mut self, other: &Scanner, mut orientations: Vec<Matrix>) -> u32 {
        let mut count = 0;
        println!("Comparing to scanner {}", other.id);

        let mut new_beacons = vec![];

        'o: for beacon_rel in &self.beacons {
            for other_rel in &other.beacons {
                if let Some(or) = self.compare_rels(other, beacon_rel, other_rel, &mut orientations) {
                    let other_pos = beacon_rel.pos - (or.try_inverse().unwrap() * other_rel.pos);

                    self.other_scanners.push(other_pos);

                    println!("Scanner {} pos: {:?} ({:?})", other.id, other_pos, other_rel.pos);

                    for beacon in &other.beacons {
                        let new_pos = other_pos + (or.try_inverse().unwrap() * beacon.pos);


                        if self.beacons.iter().any(|b| b.pos == new_pos) {  count += 1; continue;  }
                        new_beacons.push(Beacon { pos: new_pos, rel: vec![]})
                    }

                    break 'o;
                }
            }
        }

        println!("Matches: {}", count);

        if !new_beacons.is_empty() {
            self.beacons.extend(new_beacons);
            Self::set_relative_positions(&mut self.beacons);
        }

        count
    }

    fn compare_rels(
        &self,
        _other: &Scanner,
        beacon: &Beacon,
        beacon_other: &Beacon,
        orientations: &mut Vec<Matrix>,
    ) -> Option<Matrix> {
        let mut orrs = vec![];
        'o: for or in orientations.iter() {
            let mut match_found = false;
            for pos1 in &beacon.rel {
                let pos = *or * *pos1;
                let other_pos = (beacon_other.pos) + pos;

                if other_pos.x.abs() > 1000.0 || other_pos.y.abs() > 1000.0 || other_pos.z.abs() > 1000.0
                {
                    continue;
                }

                if !beacon_other.rel.contains(&(pos)) {
                    continue 'o;
                }

                match_found = true;
            }

            if match_found { orrs.push(*or) };
        }


        if orrs.len() == 1 {
            return Some(orrs[0]);
        }

        None
    }

    pub fn max_distance_to_scanner(&self) -> f32{
        let mut dist: f32 = 0.0;

        for i in 0..self.other_scanners.len() {
            for j in 0..self.other_scanners.len() {
                if j == i { continue; }

                let new_vec = self.other_scanners[i] - self.other_scanners[j];
                let new_d = new_vec.x.abs() + new_vec.y.abs() + new_vec.z.abs();
                dist = dist.max(new_d);
            }
        }

        dist
    }

    fn set_relative_positions(beacons: &mut Vec<Beacon>) {
        for i in 0..beacons.len() {
            beacons[i].rel.clear();
            for j in 0..beacons.len() {
                let pos = beacons[j].pos - beacons[i].pos;
                if pos.magnitude() == 0.0 { continue; }
                beacons[i].rel.push(pos);
            }
        }
    }
}
pub fn orientations() -> Vec<Matrix> {
    let mut out = vec![];
    let vecs = vec![vec![1.0, 0.0, 0.0], vec![0.0, 1.0, 0.0], vec![0.0, 0.0, 1.0]];

    for x in 0..3 {
        for y in 0..3 {
            if y == x {
                continue;
            }
            for z in 0..3 {
                if z == x || z == y {
                    continue;
                }
                for inv_x in [-1, 1] {
                    for inv_y in [-1, 1] {
                        for inv_z in [-1, 1] {
                            let vec_x = Vec3::from_iterator(vecs[x].clone()) * (inv_x as f32);
                            let vec_y = Vec3::from_iterator(vecs[y].clone()) * (inv_y as f32);
                            let vec_z = Vec3::from_iterator(vecs[z].clone()) * (inv_z as f32); 

                            let mat = Matrix::from_columns(&[vec_x, vec_y, vec_z]);

                            if determinant_i32(&mat) == 1.0 {
                                out.push(mat);
                            }
                        }
                    }
                }
            }
        }
    }

    out
}

#[rustfmt::skip]
fn determinant_i32(m: &Matrix) -> f32 {
    (m[0] * m[4] * m[8]) +
    (m[1] * m[5] * m[6]) +
    (m[2] * m[3] * m[7]) -
    (m[2] * m[4] * m[6]) -
    (m[1] * m[3] * m[8]) -
    (m[0] * m[5] * m[7])
}
