#[macro_use]
extern crate nom;

use std::collections::HashSet;
use std::str::FromStr;
use nom::{space, digit};


/// A particle in space
#[derive(Debug, PartialEq, Clone)]
struct Particle {
    pos: (i32, i32, i32),
    vel: (i32, i32, i32),
    acc: (i32, i32, i32),
}

impl FromStr for Particle {
    type Err = nom::ErrorKind;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        named!(integer<&str, u32>, map_res!(digit, str::parse));
        named!(number<&str, i32>, alt!(
            preceded!(tag!("-"), integer) => { |n| -(n as i32) } |
                                 integer  => { |n|   n as i32  }
        ));
        named!(triple<&str, (i32, i32, i32)>, do_parse!(
            tag!("<") >> a: number >> tag!(",") >> b: number >> tag!(",") >> c: number >> tag!(">") >> ((a, b, c))
        ));
        complete!(s, do_parse!(
            tag!("p=") >> p: triple >>
            tag!(",") >> space >>
            tag!("v=") >> v: triple >>
            tag!(",") >> space >>
            tag!("a=") >> a: triple >>
            (Particle { pos: p, vel: v, acc: a })
        )).to_result()
    }
}

impl Particle {
    /// Returns a new particle that advanced t ticks in time
    fn tick(&self, t: usize) -> Particle {
        let mut pos = self.pos;
        let mut vel = self.vel;
        for _ in 0..t {
            vel = (vel.0 + self.acc.0, vel.1 + self.acc.1, vel.2 + self.acc.2);
            pos = (pos.0 + vel.0, pos.1 + vel.1, pos.2 + vel.2);
        }
        Particle { pos: pos, vel: vel, acc: self.acc }
    }

    /// Manhattan distance to origin
    fn distance(&self) -> i32 {
        self.pos.0.abs() + self.pos.1.abs() + self.pos.2.abs()
    }
}


/// A cloud of particles in space
#[derive(Debug, Clone)]
struct Cloud(Vec<Option<Particle>>);

impl FromStr for Cloud {
    type Err = nom::ErrorKind;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Cloud(try!(s.lines().map(str::parse).map(|r| r.map(|p| Some(p))).collect())))
    }
}

impl Cloud {
    /// Number of particles
    fn count(&self) -> usize {
        self.0.iter().filter(|o| o.is_some()).count()
    }

    /// Returns a new cloud with colliding particles removed
    fn collision(&self) -> Cloud {
        let mut collisioned: HashSet<usize> = HashSet::new();
        for i in 1..self.0.len() {
            for j in 0..i {
                if let Some(ref p1) = self.0[i] {
                    if let Some(ref p2) = self.0[j] {
                        if p1.pos == p2.pos {
                            collisioned.insert(i);
                            collisioned.insert(j);
                        }
                    }
                }
            }
        }
        Cloud(self.0.iter()
            .enumerate()
            .map(|(i, o)|
                if !collisioned.contains(&i) { o.clone() } else { None }
            )
            .collect()
        )
    }

    /// Returns a new cloud that advanced t ticks in time
    fn tick(&self, t: usize) -> Cloud {
        Cloud(self.0.iter()
            .map(|o| match *o {
                Some(ref p) => Some(p.tick(t)),
                None => None
            })
            .collect()
        )
    }

    /// Returns a new cloud that advanced t ticks in time, removing colliding particles
    fn tick_with_collision(&self, t: usize) -> Cloud {
        (0..t).fold(self.clone(), |c, _| c.collision().tick(1))
    }

    /// Index of particle with smallest distance to origin
    fn nearest(&self) -> Option<usize> {
        self.0.iter()
            .filter_map(|o| match *o {
                Some(ref p) => Some(p.distance()),
                None => None
            })
            .enumerate()
            .min_by_key(|&(_, d)| d)
            .map(|(i, _)| i)
    }
}


fn main() {
    let cloud: Cloud = include_str!("day20.txt").parse().unwrap();
    println!("Particle staying closest to origin: {}", cloud.tick(1000).nearest().unwrap());
    println!("Particles left after collisions: {}", cloud.tick_with_collision(1000).count());
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples1() {
        let cloud = Cloud::from_str("p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>\np=<4,0,0>, v=<0,0,0>, a=<-2,0,0>\n").unwrap();
        assert_eq!(cloud.tick(0).0[0], Some(Particle { pos: ( 3, 0, 0), vel: ( 2, 0, 0), acc: (-1, 0, 0) }));
        assert_eq!(cloud.tick(0).0[1], Some(Particle { pos: ( 4, 0, 0), vel: ( 0, 0, 0), acc: (-2, 0, 0) }));
        assert_eq!(cloud.tick(1).0[0], Some(Particle { pos: ( 4, 0, 0), vel: ( 1, 0, 0), acc: (-1, 0, 0) }));
        assert_eq!(cloud.tick(1).0[1], Some(Particle { pos: ( 2, 0, 0), vel: (-2, 0, 0), acc: (-2, 0, 0) }));
        assert_eq!(cloud.tick(2).0[0], Some(Particle { pos: ( 4, 0, 0), vel: ( 0, 0, 0), acc: (-1, 0, 0) }));
        assert_eq!(cloud.tick(2).0[1], Some(Particle { pos: (-2, 0, 0), vel: (-4, 0, 0), acc: (-2, 0, 0) }));
        assert_eq!(cloud.tick(3).0[0], Some(Particle { pos: ( 3, 0, 0), vel: (-1, 0, 0), acc: (-1, 0, 0) }));
        assert_eq!(cloud.tick(3).0[1], Some(Particle { pos: (-8, 0, 0), vel: (-6, 0, 0), acc: (-2, 0, 0) }));
    }

    #[test]
    fn samples2() {
        let cloud = Cloud::from_str("p=<-6,0,0>, v=<3,0,0>, a=<0,0,0>\np=<-4,0,0>, v=<2,0,0>, a=<0,0,0>\np=<-2,0,0>, v=<1,0,0>, a=<0,0,0>\np=<3,0,0>, v=<-1,0,0>, a=<0,0,0>\n").unwrap();
        assert_eq!(cloud.tick_with_collision(0).0[0], Some(Particle { pos: (-6, 0, 0), vel: ( 3, 0, 0), acc: ( 0, 0, 0) }));
        assert_eq!(cloud.tick_with_collision(0).0[1], Some(Particle { pos: (-4, 0, 0), vel: ( 2, 0, 0), acc: ( 0, 0, 0) }));
        assert_eq!(cloud.tick_with_collision(0).0[2], Some(Particle { pos: (-2, 0, 0), vel: ( 1, 0, 0), acc: ( 0, 0, 0) }));
        assert_eq!(cloud.tick_with_collision(0).0[3], Some(Particle { pos: ( 3, 0, 0), vel: (-1, 0, 0), acc: ( 0, 0, 0) }));
        assert_eq!(cloud.tick_with_collision(0).count(), 4);
        assert_eq!(cloud.tick_with_collision(1).0[0], Some(Particle { pos: (-3, 0, 0), vel: ( 3, 0, 0), acc: ( 0, 0, 0) }));
        assert_eq!(cloud.tick_with_collision(1).0[1], Some(Particle { pos: (-2, 0, 0), vel: ( 2, 0, 0), acc: ( 0, 0, 0) }));
        assert_eq!(cloud.tick_with_collision(1).0[2], Some(Particle { pos: (-1, 0, 0), vel: ( 1, 0, 0), acc: ( 0, 0, 0) }));
        assert_eq!(cloud.tick_with_collision(1).0[3], Some(Particle { pos: ( 2, 0, 0), vel: (-1, 0, 0), acc: ( 0, 0, 0) }));
        assert_eq!(cloud.tick_with_collision(1).count(), 4);
        assert_eq!(cloud.tick_with_collision(2).0[0], Some(Particle { pos: ( 0, 0, 0), vel: ( 3, 0, 0), acc: ( 0, 0, 0) }));
        assert_eq!(cloud.tick_with_collision(2).0[1], Some(Particle { pos: ( 0, 0, 0), vel: ( 2, 0, 0), acc: ( 0, 0, 0) }));
        assert_eq!(cloud.tick_with_collision(2).0[2], Some(Particle { pos: ( 0, 0, 0), vel: ( 1, 0, 0), acc: ( 0, 0, 0) }));
        assert_eq!(cloud.tick_with_collision(2).0[3], Some(Particle { pos: ( 1, 0, 0), vel: (-1, 0, 0), acc: ( 0, 0, 0) }));
        assert_eq!(cloud.tick_with_collision(2).count(), 4);
        assert_eq!(cloud.tick_with_collision(3).0[0], None);
        assert_eq!(cloud.tick_with_collision(3).0[1], None);
        assert_eq!(cloud.tick_with_collision(3).0[2], None);
        assert_eq!(cloud.tick_with_collision(3).0[3], Some(Particle { pos: ( 0, 0, 0), vel: (-1, 0, 0), acc: ( 0, 0, 0) }));
        assert_eq!(cloud.tick_with_collision(3).count(), 1);
    }
}
