use std::num::pow;

pub struct MultiplesOfSquares {
    xx: uint,
    dxx: uint,
    increment: uint,
    limit: uint
}

impl MultiplesOfSquares {
    #[inline]
    pub fn new(start_x: uint, multiple: uint, skip: uint, limit: uint) -> MultiplesOfSquares {
        MultiplesOfSquares {
            xx: pow(start_x, 2) * multiple,
            dxx: (pow(skip, 2) + start_x*skip*2) * multiple,
            increment: pow(skip, 2) * 2 * multiple,
            limit: limit
        }
    }
}

impl Iterator<uint> for MultiplesOfSquares {
    fn next(&mut self) -> Option<uint> {
        if self.xx >= self.limit as uint {
            return None
        }

        let return_value = Some(self.xx);
        self.xx += self.dxx;
        self.dxx += self.increment;

        return_value
    }
}