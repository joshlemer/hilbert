#![allow(dead_code, unused_variables)]

use std;

//#![allow(unused_variables)]

#[derive(Debug)]
pub enum Error {
    OutOfRange,
    NotPowerOfTwo,
    NotPositive
}

pub struct Hilbert {
    n: u32,
}


impl Hilbert {


    pub fn new(n: u32) -> Result<Hilbert, Error> {
        if n <= 0 {
            Err(Error::NotPositive)
        } else if n & (n - 1) != 0 {
            Err(Error::NotPowerOfTwo)
        } else {
            Ok(Hilbert { n })
        }
    }
}

fn rotate(n: u32, x: u32, y: u32, rx: bool, ry: bool) -> (u32, u32) {

    let mut x = x as i64;
    let mut y = y as i64;
    let n = n as i64;

    if !ry {
        if rx {
            x = n - 1 - x;
            y = n - 1 - y;
        }

        std::mem::swap(&mut x, &mut y);

    }



    (x as u32, y as u32)
}


impl SpaceFilling<u32> for Hilbert {
    fn map(&self, mut t: u32) -> (u32, u32) {
        if t >= self.n * self.n {
            panic!();
        } else {
            println!("{}, {}, {}", t, self.n, self.n * self.n);

            let mut x = 0;
            let mut y = 0;


            let mut i = 1;
            while i < self.n {
                let rx = t & 2 == 2;
                let mut ry = t & 1 == 1;

                if rx {
                    ry = !ry;
                }

                let rotated = rotate(i, x, y, rx, ry);
                x = rotated.0;
                y = rotated.1;

                if rx {
                    x = x + i;
                }

                if ry {
                    y = y + i;
                }

                t /= 4;


                i *= 2;
            }

            (x, y)
        }
    }

    fn map_inverse(&self, mut x: u32, mut y: u32) -> u32 {
        if x >= self.n || y >= self.n {
            panic!();
        } else {
            let mut t = 0;
            let mut i = self.n / 2;

            while i > 0 {
                let rx = (x & i) > 0;
                let ry = (y & i) > 0;

                let mut a = 0;

                if rx {
                    a = 3;
                }

                t += i * i * (a ^ (if ry { 1 } else { 0 }));

                println!("i, x, y, rx, ry = {},{},{},{},{}", i, x, y, rx, ry);
                let (_x, _y) = rotate(i, x, y, rx, ry);
                x = _x;
                y = _y;

                i /= 2;
            }

            t
        }


    }

    fn dimensions(&self) -> (u32, u32) {
        (self.n, self.n)
    }
}

pub fn foo((a, b): (i32, i32)) {
    println!();
}

pub struct Peano {
    n: usize
}

impl Peano {
    pub fn new(n: usize) -> Self {
        if n == 0 {
            panic!();
        } else {
            Peano { n: n }
        }
    }
}

impl SpaceFilling<i32> for Peano {
    fn map(&self, t: i32) -> (i32, i32) {
        unimplemented!()
    }

    fn map_inverse(&self, x: i32, y: i32) -> i32 {
        unimplemented!()
    }

    fn dimensions(&self) -> (i32, i32) {
        unimplemented!()
    }
}

pub trait SpaceFilling<Num> {

    fn map(&self, t: Num) -> (Num, Num);

    fn map_inverse(&self, x: Num, y: Num) -> Num;

    fn dimensions(&self) -> (Num, Num);
}



#[cfg(test)]
mod tests {
    use super::*;

    const TEST_CASES: &[(u32, u32, u32)] = &[
        (0, 0, 0),
        (16, 4, 0),
        (32, 4, 4),
        (48, 3, 7),
        (64, 0, 8),
        (80, 0, 12),
        (96, 4, 12),
        (112, 7, 11),
        (128, 8, 8),
        (144, 8, 12),
        (160, 12, 12),
        (170, 15, 15),
        (176, 15, 11),
        (192, 15, 7),
        (208, 11, 7),
        (224, 11, 3),
        (240, 12, 0),
        (255, 15, 0),
    ];

    #[test]
    fn test_map() {
        let h: Hilbert = Hilbert::new(16).unwrap();

        for (d, x, y) in TEST_CASES {
            assert_eq!(h.map(*d), (*x, *y));
        }
    }
    #[test]
    fn test_map_inverse() {
        let h: Hilbert = Hilbert::new(16).unwrap();

        for (d, x, y) in TEST_CASES {
            assert_eq!(h.map_inverse(*x, *y), *d);
        }
    }
}
