#![allow(dead_code, unused_variables)]

use std;

use super::{SpaceFilling, Error};


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
    fn map(&self, mut t: u32) -> Result<(u32, u32), Error> {
        if t >= self.n * self.n {
            Err(Error::OutOfRange)
        } else {

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

            Ok((x, y))
        }
    }

    fn map_inverse(&self, mut x: u32, mut y: u32) -> Result<u32, Error> {
        if x >= self.n || y >= self.n {
            Err(Error::OutOfRange)
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

                let (_x, _y) = rotate(i, x, y, rx, ry);
                x = _x;
                y = _y;

                i /= 2;
            }

            Ok(t)
        }


    }

    fn dimensions(&self) -> (u32, u32) {
        (self.n, self.n)
    }

    fn size(&self) -> u32 {
        self.n * self.n
    }
}

pub fn foo((a, b): (i32, i32)) {
    println!();
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
    fn test_new_errors() {

        let new_test_cases = &[
            (0, Error::NotPositive),
            (3, Error::NotPowerOfTwo),
            (5, Error::NotPowerOfTwo),
        ];

        for (n, e) in new_test_cases {
            assert_eq!(Hilbert::new(*n).err().unwrap(), *e);
        }

    }

    #[test]
    fn test_map_range_errors() {

        let test_cases = &[
            (0, None),
            (255, None),
            (256, Some(Error::OutOfRange)),
        ];

        let h = Hilbert::new(16).unwrap();


        for (n, e) in test_cases {
            assert_eq!(h.map(*n).err(), *e);
        }
    }
    #[test]
    fn test_map_inverse_range_errors() {

        let test_cases = &[
            (0, 0, None),
            (15, 15, None),
            (16, 15, Some(Error::OutOfRange)),
            (15, 16, Some(Error::OutOfRange)),
        ];

        let h = Hilbert::new(16).unwrap();


        for (x, y, e) in test_cases {
            assert_eq!(h.map_inverse(*x,*y).err(), *e);
        }
    }

    #[test]
    fn test_small_map() {
        let h = Hilbert::new(1).unwrap();
        assert_eq!(h.map(0), Ok((0, 0)));
        assert_eq!(h.map_inverse(0, 0), Ok(0));
    }

    #[test]
    fn test_all_map_values() {
        let h = Hilbert::new(16).unwrap();

        for d in 0..h.size() {
            let (x, y) = h.map(d).unwrap();
            assert!(x < h.dimensions().0);
            assert!(y < h.dimensions().0);
            assert_eq!(d, h.map_inverse(x, y).unwrap());
        }
    }

    #[test]
    fn test_map() {
        let h: Hilbert = Hilbert::new(16).unwrap();

        for (d, x, y) in TEST_CASES {
            assert_eq!(h.map(*d).unwrap(), (*x, *y));
        }
    }

    #[test]
    fn test_map_inverse() {
        let h: Hilbert = Hilbert::new(16).unwrap();

        for (d, x, y) in TEST_CASES {
            assert_eq!(h.map_inverse(*x, *y).unwrap(), *d);
        }
    }
}
