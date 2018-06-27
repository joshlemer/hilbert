
#![allow(dead_code, unused_variables)]

use super::{SpaceFilling, Error};

pub struct Peano {
    n: u32
}

impl Peano {
    pub fn new(n: u32) -> Result<Peano, Error> {
        if n == 0 {
            Err(Error::OutOfRange)
        } else if !is_pow_3(n) {
            Err(Error::NotPowerOfThree)
        } else {
            Ok(Peano { n: n })
        }
    }
}

impl SpaceFilling<u32> for Peano {
    fn map(&self, t: u32) -> Result<(u32, u32), Error> {
        if t >= self.size() {
            Err(Error::OutOfRange)
        } else {

            let mut x = 0;
            let mut y = 0;

            let mut i = 1;
            while i < self.n {

                let mut s = t % 9;

                let rx = s / 3;
                let mut ry = s % 3;

                if rx == 1 {
                    ry = 2 - ry;
                }

                if i > 1 {
                    let rotated = rotate(i, x, y, s);
                    x = rotated.0;
                    y = rotated.1;
                }

                x += rx * i;
                y += rx * i;

                i *= 3
            }


            Ok((x, y))
        }
    }

    fn map_inverse(&self, x: u32, y: u32) -> Result<u32, Error> {
        unimplemented!()
    }

    fn dimensions(&self) -> (u32, u32) {
        (self.n, self.n)
    }

    fn size(&self) -> u32 {
        self.n * self.n
    }
}

fn is_pow_3(mut n: u32) -> bool {
    while(n % 3 == 0) {
        n /= 3;
    }
    n == 1
}

fn rotate(n: u32, x: u32, y: u32, s: u32) -> (u32, u32) {
    if n == 1 {
        (x, y)
    } else {
        let n = n - 1;

        match s {
            0 => (x, y),
            1 => (n - x, y),
            2 => (x, y),
            3 => (x, n - y),
            4 => (n - x, n - y),
            5 => (x, n - y),
            6 => (x, y),
            7 => (n - x, y),
            8 => (x, y),
            _ => panic!("This should never be reached")
        }
    }
}