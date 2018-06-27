pub mod hilbert;

pub mod peano;

pub trait SpaceFilling<Num> {

    fn map(&self, t: Num) -> Result<(Num, Num), Error>;

    fn map_inverse(&self, x: Num, y: Num) -> Result<Num, Error>;

    fn dimensions(&self) -> (Num, Num);

    fn size(&self) -> Num;
}

#[derive(Debug, PartialEq)]
pub enum Error {
    OutOfRange,
    NotPowerOfTwo,
    NotPowerOfThree,
    NotPositive,
}
