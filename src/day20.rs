#[macro_use]
extern crate nom;

use std::str::FromStr;
use nom::{space, digit};


/// A particle in space
#[derive(Debug)]
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
            map!(preceded!(tag!("-"), integer), |n| -(n as i32)) |
            map!(integer, |n| n as i32)
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
#[derive(Debug)]
struct Cloud(Vec<Particle>);

impl FromStr for Cloud {
    type Err = nom::ErrorKind;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Cloud(try!(s.lines().map(str::parse).collect())))
    }
}

impl Cloud {
    /// Returns a new cloud that advanced t ticks in time
    fn tick(&self, t: usize) -> Cloud {
        Cloud(self.0.iter().map(|p| p.tick(t)).collect())
    }

    /// Index of particle with smallest distance to origin
    fn nearest(&self) -> Option<usize> {
        self.0.iter().map(Particle::distance).enumerate().min_by_key(|&(_, d)| d).map(|(i, _)| i)
    }
}


fn main() {
    let cloud: Cloud = include_str!("day20.txt").parse().unwrap();
    println!("Particle staying closest to origin: {}", cloud.tick(1000).nearest().unwrap());
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn samples1() {
        let cloud = Cloud::from_str("p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>\np=<4,0,0>, v=<0,0,0>, a=<-2,0,0>\n").unwrap();
        assert_eq!(cloud.0[0].tick(0).pos, ( 3, 0, 0));
        assert_eq!(cloud.0[1].tick(0).pos, ( 4, 0, 0));
        assert_eq!(cloud.0[0].tick(1).pos, ( 4, 0, 0));
        assert_eq!(cloud.0[1].tick(1).pos, ( 2, 0, 0));
        assert_eq!(cloud.0[0].tick(2).pos, ( 4, 0, 0));
        assert_eq!(cloud.0[1].tick(2).pos, (-2, 0, 0));
        assert_eq!(cloud.0[0].tick(3).pos, ( 3, 0, 0));
        assert_eq!(cloud.0[1].tick(3).pos, (-8, 0, 0));
    }
}
