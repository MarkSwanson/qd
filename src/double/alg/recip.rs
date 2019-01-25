// Copyright (c) 2019 Thomas Otterson
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

use crate::double::Double;

impl Double {
    /// Calculates the reciprocal of the number, or 1/x.
    ///
    /// # Examples
    /// ```
    /// # #[macro_use] extern crate qd;
    /// # use qd::Double;
    /// # fn main() {
    /// let x = Double::PI.recip();
    /// let expected = dd!("0.31830988618379067153776752674503");
    ///
    /// let diff = (x - expected).abs();
    /// assert!(diff < dd!(1e-30));
    /// # }
    /// ```
    #[inline]
    pub fn recip(self) -> Double {
        Double::ONE / self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_close!(
            dd!("0.31830988618379067153776752674503"),
            Double::PI.recip()
        );
        assert_close!(dd!("0.36787944117144232159552377016146"), Double::E.recip());
    }

    #[test]
    fn special() {
        assert_exact!(Double::INFINITY, dd!(0.0).recip());
        assert_exact!(Double::NEG_INFINITY, dd!(-0.0).recip());
        assert_exact!(Double::ZERO, Double::INFINITY.recip());
        assert_exact!(Double::NEG_ZERO, Double::NEG_INFINITY.recip());
    }
}
