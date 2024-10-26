use super::*;


//------------------------- -10 -------------------------

impl<P1, P2> MulScale<Scale<P2, 0>> for Scale<P1, -10>
where
    P1: Prefix,
    P2: Prefix,
{
    type Output = Scale<P1, -10>;
}

impl<P> MulScale<Scale<P, 1>> for Scale<P, -10>
where
    P: Prefix,
{
    type Output = Scale<P, -9>;
}

impl<P> MulScale<Scale<P, 2>> for Scale<P, -10>
where
    P: Prefix,
{
    type Output = Scale<P, -8>;
}

impl<P> MulScale<Scale<P, 3>> for Scale<P, -10>
where
    P: Prefix,
{
    type Output = Scale<P, -7>;
}

impl<P> MulScale<Scale<P, 4>> for Scale<P, -10>
where
    P: Prefix,
{
    type Output = Scale<P, -6>;
}

impl<P> MulScale<Scale<P, 5>> for Scale<P, -10>
where
    P: Prefix,
{
    type Output = Scale<P, -5>;
}

impl<P> MulScale<Scale<P, 6>> for Scale<P, -10>
where
    P: Prefix,
{
    type Output = Scale<P, -4>;
}

impl<P> MulScale<Scale<P, 7>> for Scale<P, -10>
where
    P: Prefix,
{
    type Output = Scale<P, -3>;
}

impl<P> MulScale<Scale<P, 8>> for Scale<P, -10>
where
    P: Prefix,
{
    type Output = Scale<P, -2>;
}

impl<P> MulScale<Scale<P, 9>> for Scale<P, -10>
where
    P: Prefix,
{
    type Output = Scale<P, -1>;
}

impl<P> MulScale<Scale<P, 10>> for Scale<P, -10>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

//------------------------- -9 -------------------------

impl<P> MulScale<Scale<P, -1>> for Scale<P, -9>
where
    P: Prefix,
{
    type Output = Scale<P, -10>;
}

impl<P1, P2> MulScale<Scale<P2, 0>> for Scale<P1, -9>
where
    P1: Prefix,
    P2: Prefix,
{
    type Output = Scale<P1, -9>;
}

impl<P> MulScale<Scale<P, 1>> for Scale<P, -9>
where
    P: Prefix,
{
    type Output = Scale<P, -8>;
}

impl<P> MulScale<Scale<P, 2>> for Scale<P, -9>
where
    P: Prefix,
{
    type Output = Scale<P, -7>;
}

impl<P> MulScale<Scale<P, 3>> for Scale<P, -9>
where
    P: Prefix,
{
    type Output = Scale<P, -6>;
}

impl<P> MulScale<Scale<P, 4>> for Scale<P, -9>
where
    P: Prefix,
{
    type Output = Scale<P, -5>;
}

impl<P> MulScale<Scale<P, 5>> for Scale<P, -9>
where
    P: Prefix,
{
    type Output = Scale<P, -4>;
}

impl<P> MulScale<Scale<P, 6>> for Scale<P, -9>
where
    P: Prefix,
{
    type Output = Scale<P, -3>;
}

impl<P> MulScale<Scale<P, 7>> for Scale<P, -9>
where
    P: Prefix,
{
    type Output = Scale<P, -2>;
}

impl<P> MulScale<Scale<P, 8>> for Scale<P, -9>
where
    P: Prefix,
{
    type Output = Scale<P, -1>;
}

impl<P> MulScale<Scale<P, 9>> for Scale<P, -9>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> MulScale<Scale<P, 10>> for Scale<P, -9>
where
    P: Prefix,
{
    type Output = Scale<P, 1>;
}

//------------------------- -8 -------------------------

impl<P> MulScale<Scale<P, -2>> for Scale<P, -8>
where
    P: Prefix,
{
    type Output = Scale<P, -10>;
}

impl<P> MulScale<Scale<P, -1>> for Scale<P, -8>
where
    P: Prefix,
{
    type Output = Scale<P, -9>;
}

impl<P1, P2> MulScale<Scale<P2, 0>> for Scale<P1, -8>
where
    P1: Prefix,
    P2: Prefix,
{
    type Output = Scale<P1, -8>;
}

impl<P> MulScale<Scale<P, 1>> for Scale<P, -8>
where
    P: Prefix,
{
    type Output = Scale<P, -7>;
}

impl<P> MulScale<Scale<P, 2>> for Scale<P, -8>
where
    P: Prefix,
{
    type Output = Scale<P, -6>;
}

impl<P> MulScale<Scale<P, 3>> for Scale<P, -8>
where
    P: Prefix,
{
    type Output = Scale<P, -5>;
}

impl<P> MulScale<Scale<P, 4>> for Scale<P, -8>
where
    P: Prefix,
{
    type Output = Scale<P, -4>;
}

impl<P> MulScale<Scale<P, 5>> for Scale<P, -8>
where
    P: Prefix,
{
    type Output = Scale<P, -3>;
}

impl<P> MulScale<Scale<P, 6>> for Scale<P, -8>
where
    P: Prefix,
{
    type Output = Scale<P, -2>;
}

impl<P> MulScale<Scale<P, 7>> for Scale<P, -8>
where
    P: Prefix,
{
    type Output = Scale<P, -1>;
}

impl<P> MulScale<Scale<P, 8>> for Scale<P, -8>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> MulScale<Scale<P, 9>> for Scale<P, -8>
where
    P: Prefix,
{
    type Output = Scale<P, 1>;
}

impl<P> MulScale<Scale<P, 10>> for Scale<P, -8>
where
    P: Prefix,
{
    type Output = Scale<P, 2>;
}

//------------------------- -7 -------------------------

impl<P> MulScale<Scale<P, -3>> for Scale<P, -7>
where
    P: Prefix,
{
    type Output = Scale<P, -10>;
}

impl<P> MulScale<Scale<P, -2>> for Scale<P, -7>
where
    P: Prefix,
{
    type Output = Scale<P, -9>;
}

impl<P> MulScale<Scale<P, -1>> for Scale<P, -7>
where
    P: Prefix,
{
    type Output = Scale<P, -8>;
}

impl<P1, P2> MulScale<Scale<P2, 0>> for Scale<P1, -7>
where
    P1: Prefix,
    P2: Prefix,
{
    type Output = Scale<P1, -7>;
}

impl<P> MulScale<Scale<P, 1>> for Scale<P, -7>
where
    P: Prefix,
{
    type Output = Scale<P, -6>;
}

impl<P> MulScale<Scale<P, 2>> for Scale<P, -7>
where
    P: Prefix,
{
    type Output = Scale<P, -5>;
}

impl<P> MulScale<Scale<P, 3>> for Scale<P, -7>
where
    P: Prefix,
{
    type Output = Scale<P, -4>;
}

impl<P> MulScale<Scale<P, 4>> for Scale<P, -7>
where
    P: Prefix,
{
    type Output = Scale<P, -3>;
}

impl<P> MulScale<Scale<P, 5>> for Scale<P, -7>
where
    P: Prefix,
{
    type Output = Scale<P, -2>;
}

impl<P> MulScale<Scale<P, 6>> for Scale<P, -7>
where
    P: Prefix,
{
    type Output = Scale<P, -1>;
}

impl<P> MulScale<Scale<P, 7>> for Scale<P, -7>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> MulScale<Scale<P, 8>> for Scale<P, -7>
where
    P: Prefix,
{
    type Output = Scale<P, 1>;
}

impl<P> MulScale<Scale<P, 9>> for Scale<P, -7>
where
    P: Prefix,
{
    type Output = Scale<P, 2>;
}

impl<P> MulScale<Scale<P, 10>> for Scale<P, -7>
where
    P: Prefix,
{
    type Output = Scale<P, 3>;
}

//------------------------- -6 -------------------------

impl<P> MulScale<Scale<P, -4>> for Scale<P, -6>
where
    P: Prefix,
{
    type Output = Scale<P, -10>;
}

impl<P> MulScale<Scale<P, -3>> for Scale<P, -6>
where
    P: Prefix,
{
    type Output = Scale<P, -9>;
}

impl<P> MulScale<Scale<P, -2>> for Scale<P, -6>
where
    P: Prefix,
{
    type Output = Scale<P, -8>;
}

impl<P> MulScale<Scale<P, -1>> for Scale<P, -6>
where
    P: Prefix,
{
    type Output = Scale<P, -7>;
}

impl<P1, P2> MulScale<Scale<P2, 0>> for Scale<P1, -6>
where
    P1: Prefix,
    P2: Prefix,
{
    type Output = Scale<P1, -6>;
}

impl<P> MulScale<Scale<P, 1>> for Scale<P, -6>
where
    P: Prefix,
{
    type Output = Scale<P, -5>;
}

impl<P> MulScale<Scale<P, 2>> for Scale<P, -6>
where
    P: Prefix,
{
    type Output = Scale<P, -4>;
}

impl<P> MulScale<Scale<P, 3>> for Scale<P, -6>
where
    P: Prefix,
{
    type Output = Scale<P, -3>;
}

impl<P> MulScale<Scale<P, 4>> for Scale<P, -6>
where
    P: Prefix,
{
    type Output = Scale<P, -2>;
}

impl<P> MulScale<Scale<P, 5>> for Scale<P, -6>
where
    P: Prefix,
{
    type Output = Scale<P, -1>;
}

impl<P> MulScale<Scale<P, 6>> for Scale<P, -6>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> MulScale<Scale<P, 7>> for Scale<P, -6>
where
    P: Prefix,
{
    type Output = Scale<P, 1>;
}

impl<P> MulScale<Scale<P, 8>> for Scale<P, -6>
where
    P: Prefix,
{
    type Output = Scale<P, 2>;
}

impl<P> MulScale<Scale<P, 9>> for Scale<P, -6>
where
    P: Prefix,
{
    type Output = Scale<P, 3>;
}

impl<P> MulScale<Scale<P, 10>> for Scale<P, -6>
where
    P: Prefix,
{
    type Output = Scale<P, 4>;
}

//------------------------- -5 -------------------------

impl<P> MulScale<Scale<P, -5>> for Scale<P, -5>
where
    P: Prefix,
{
    type Output = Scale<P, -10>;
}

impl<P> MulScale<Scale<P, -4>> for Scale<P, -5>
where
    P: Prefix,
{
    type Output = Scale<P, -9>;
}

impl<P> MulScale<Scale<P, -3>> for Scale<P, -5>
where
    P: Prefix,
{
    type Output = Scale<P, -8>;
}

impl<P> MulScale<Scale<P, -2>> for Scale<P, -5>
where
    P: Prefix,
{
    type Output = Scale<P, -7>;
}

impl<P> MulScale<Scale<P, -1>> for Scale<P, -5>
where
    P: Prefix,
{
    type Output = Scale<P, -6>;
}

impl<P1, P2> MulScale<Scale<P2, 0>> for Scale<P1, -5>
where
    P1: Prefix,
    P2: Prefix,
{
    type Output = Scale<P1, -5>;
}

impl<P> MulScale<Scale<P, 1>> for Scale<P, -5>
where
    P: Prefix,
{
    type Output = Scale<P, -4>;
}

impl<P> MulScale<Scale<P, 2>> for Scale<P, -5>
where
    P: Prefix,
{
    type Output = Scale<P, -3>;
}

impl<P> MulScale<Scale<P, 3>> for Scale<P, -5>
where
    P: Prefix,
{
    type Output = Scale<P, -2>;
}

impl<P> MulScale<Scale<P, 4>> for Scale<P, -5>
where
    P: Prefix,
{
    type Output = Scale<P, -1>;
}

impl<P> MulScale<Scale<P, 5>> for Scale<P, -5>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> MulScale<Scale<P, 6>> for Scale<P, -5>
where
    P: Prefix,
{
    type Output = Scale<P, 1>;
}

impl<P> MulScale<Scale<P, 7>> for Scale<P, -5>
where
    P: Prefix,
{
    type Output = Scale<P, 2>;
}

impl<P> MulScale<Scale<P, 8>> for Scale<P, -5>
where
    P: Prefix,
{
    type Output = Scale<P, 3>;
}

impl<P> MulScale<Scale<P, 9>> for Scale<P, -5>
where
    P: Prefix,
{
    type Output = Scale<P, 4>;
}

impl<P> MulScale<Scale<P, 10>> for Scale<P, -5>
where
    P: Prefix,
{
    type Output = Scale<P, 5>;
}

//------------------------- -4 -------------------------

impl<P> MulScale<Scale<P, -6>> for Scale<P, -4>
where
    P: Prefix,
{
    type Output = Scale<P, -10>;
}

impl<P> MulScale<Scale<P, -5>> for Scale<P, -4>
where
    P: Prefix,
{
    type Output = Scale<P, -9>;
}

impl<P> MulScale<Scale<P, -4>> for Scale<P, -4>
where
    P: Prefix,
{
    type Output = Scale<P, -8>;
}

impl<P> MulScale<Scale<P, -3>> for Scale<P, -4>
where
    P: Prefix,
{
    type Output = Scale<P, -7>;
}

impl<P> MulScale<Scale<P, -2>> for Scale<P, -4>
where
    P: Prefix,
{
    type Output = Scale<P, -6>;
}

impl<P> MulScale<Scale<P, -1>> for Scale<P, -4>
where
    P: Prefix,
{
    type Output = Scale<P, -5>;
}

impl<P1, P2> MulScale<Scale<P2, 0>> for Scale<P1, -4>
where
    P1: Prefix,
    P2: Prefix,
{
    type Output = Scale<P1, -4>;
}

impl<P> MulScale<Scale<P, 1>> for Scale<P, -4>
where
    P: Prefix,
{
    type Output = Scale<P, -3>;
}

impl<P> MulScale<Scale<P, 2>> for Scale<P, -4>
where
    P: Prefix,
{
    type Output = Scale<P, -2>;
}

impl<P> MulScale<Scale<P, 3>> for Scale<P, -4>
where
    P: Prefix,
{
    type Output = Scale<P, -1>;
}

impl<P> MulScale<Scale<P, 4>> for Scale<P, -4>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> MulScale<Scale<P, 5>> for Scale<P, -4>
where
    P: Prefix,
{
    type Output = Scale<P, 1>;
}

impl<P> MulScale<Scale<P, 6>> for Scale<P, -4>
where
    P: Prefix,
{
    type Output = Scale<P, 2>;
}

impl<P> MulScale<Scale<P, 7>> for Scale<P, -4>
where
    P: Prefix,
{
    type Output = Scale<P, 3>;
}

impl<P> MulScale<Scale<P, 8>> for Scale<P, -4>
where
    P: Prefix,
{
    type Output = Scale<P, 4>;
}

impl<P> MulScale<Scale<P, 9>> for Scale<P, -4>
where
    P: Prefix,
{
    type Output = Scale<P, 5>;
}

impl<P> MulScale<Scale<P, 10>> for Scale<P, -4>
where
    P: Prefix,
{
    type Output = Scale<P, 6>;
}

//------------------------- -3 -------------------------

impl<P> MulScale<Scale<P, -7>> for Scale<P, -3>
where
    P: Prefix,
{
    type Output = Scale<P, -10>;
}

impl<P> MulScale<Scale<P, -6>> for Scale<P, -3>
where
    P: Prefix,
{
    type Output = Scale<P, -9>;
}

impl<P> MulScale<Scale<P, -5>> for Scale<P, -3>
where
    P: Prefix,
{
    type Output = Scale<P, -8>;
}

impl<P> MulScale<Scale<P, -4>> for Scale<P, -3>
where
    P: Prefix,
{
    type Output = Scale<P, -7>;
}

impl<P> MulScale<Scale<P, -3>> for Scale<P, -3>
where
    P: Prefix,
{
    type Output = Scale<P, -6>;
}

impl<P> MulScale<Scale<P, -2>> for Scale<P, -3>
where
    P: Prefix,
{
    type Output = Scale<P, -5>;
}

impl<P> MulScale<Scale<P, -1>> for Scale<P, -3>
where
    P: Prefix,
{
    type Output = Scale<P, -4>;
}

impl<P1, P2> MulScale<Scale<P2, 0>> for Scale<P1, -3>
where
    P1: Prefix,
    P2: Prefix,
{
    type Output = Scale<P1, -3>;
}

impl<P> MulScale<Scale<P, 1>> for Scale<P, -3>
where
    P: Prefix,
{
    type Output = Scale<P, -2>;
}

impl<P> MulScale<Scale<P, 2>> for Scale<P, -3>
where
    P: Prefix,
{
    type Output = Scale<P, -1>;
}

impl<P> MulScale<Scale<P, 3>> for Scale<P, -3>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> MulScale<Scale<P, 4>> for Scale<P, -3>
where
    P: Prefix,
{
    type Output = Scale<P, 1>;
}

impl<P> MulScale<Scale<P, 5>> for Scale<P, -3>
where
    P: Prefix,
{
    type Output = Scale<P, 2>;
}

impl<P> MulScale<Scale<P, 6>> for Scale<P, -3>
where
    P: Prefix,
{
    type Output = Scale<P, 3>;
}

impl<P> MulScale<Scale<P, 7>> for Scale<P, -3>
where
    P: Prefix,
{
    type Output = Scale<P, 4>;
}

impl<P> MulScale<Scale<P, 8>> for Scale<P, -3>
where
    P: Prefix,
{
    type Output = Scale<P, 5>;
}

impl<P> MulScale<Scale<P, 9>> for Scale<P, -3>
where
    P: Prefix,
{
    type Output = Scale<P, 6>;
}

impl<P> MulScale<Scale<P, 10>> for Scale<P, -3>
where
    P: Prefix,
{
    type Output = Scale<P, 7>;
}

//------------------------- -2 -------------------------

impl<P> MulScale<Scale<P, -8>> for Scale<P, -2>
where
    P: Prefix,
{
    type Output = Scale<P, -10>;
}

impl<P> MulScale<Scale<P, -7>> for Scale<P, -2>
where
    P: Prefix,
{
    type Output = Scale<P, -9>;
}

impl<P> MulScale<Scale<P, -6>> for Scale<P, -2>
where
    P: Prefix,
{
    type Output = Scale<P, -8>;
}

impl<P> MulScale<Scale<P, -5>> for Scale<P, -2>
where
    P: Prefix,
{
    type Output = Scale<P, -7>;
}

impl<P> MulScale<Scale<P, -4>> for Scale<P, -2>
where
    P: Prefix,
{
    type Output = Scale<P, -6>;
}

impl<P> MulScale<Scale<P, -3>> for Scale<P, -2>
where
    P: Prefix,
{
    type Output = Scale<P, -5>;
}

impl<P> MulScale<Scale<P, -2>> for Scale<P, -2>
where
    P: Prefix,
{
    type Output = Scale<P, -4>;
}

impl<P> MulScale<Scale<P, -1>> for Scale<P, -2>
where
    P: Prefix,
{
    type Output = Scale<P, -3>;
}

impl<P1, P2> MulScale<Scale<P2, 0>> for Scale<P1, -2>
where
    P1: Prefix,
    P2: Prefix,
{
    type Output = Scale<P1, -2>;
}

impl<P> MulScale<Scale<P, 1>> for Scale<P, -2>
where
    P: Prefix,
{
    type Output = Scale<P, -1>;
}

impl<P> MulScale<Scale<P, 2>> for Scale<P, -2>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> MulScale<Scale<P, 3>> for Scale<P, -2>
where
    P: Prefix,
{
    type Output = Scale<P, 1>;
}

impl<P> MulScale<Scale<P, 4>> for Scale<P, -2>
where
    P: Prefix,
{
    type Output = Scale<P, 2>;
}

impl<P> MulScale<Scale<P, 5>> for Scale<P, -2>
where
    P: Prefix,
{
    type Output = Scale<P, 3>;
}

impl<P> MulScale<Scale<P, 6>> for Scale<P, -2>
where
    P: Prefix,
{
    type Output = Scale<P, 4>;
}

impl<P> MulScale<Scale<P, 7>> for Scale<P, -2>
where
    P: Prefix,
{
    type Output = Scale<P, 5>;
}

impl<P> MulScale<Scale<P, 8>> for Scale<P, -2>
where
    P: Prefix,
{
    type Output = Scale<P, 6>;
}

impl<P> MulScale<Scale<P, 9>> for Scale<P, -2>
where
    P: Prefix,
{
    type Output = Scale<P, 7>;
}

impl<P> MulScale<Scale<P, 10>> for Scale<P, -2>
where
    P: Prefix,
{
    type Output = Scale<P, 8>;
}

//------------------------- -1 -------------------------

impl<P> MulScale<Scale<P, -9>> for Scale<P, -1>
where
    P: Prefix,
{
    type Output = Scale<P, -10>;
}

impl<P> MulScale<Scale<P, -8>> for Scale<P, -1>
where
    P: Prefix,
{
    type Output = Scale<P, -9>;
}

impl<P> MulScale<Scale<P, -7>> for Scale<P, -1>
where
    P: Prefix,
{
    type Output = Scale<P, -8>;
}

impl<P> MulScale<Scale<P, -6>> for Scale<P, -1>
where
    P: Prefix,
{
    type Output = Scale<P, -7>;
}

impl<P> MulScale<Scale<P, -5>> for Scale<P, -1>
where
    P: Prefix,
{
    type Output = Scale<P, -6>;
}

impl<P> MulScale<Scale<P, -4>> for Scale<P, -1>
where
    P: Prefix,
{
    type Output = Scale<P, -5>;
}

impl<P> MulScale<Scale<P, -3>> for Scale<P, -1>
where
    P: Prefix,
{
    type Output = Scale<P, -4>;
}

impl<P> MulScale<Scale<P, -2>> for Scale<P, -1>
where
    P: Prefix,
{
    type Output = Scale<P, -3>;
}

impl<P> MulScale<Scale<P, -1>> for Scale<P, -1>
where
    P: Prefix,
{
    type Output = Scale<P, -2>;
}

impl<P1, P2> MulScale<Scale<P2, 0>> for Scale<P1, -1>
where
    P1: Prefix,
    P2: Prefix,
{
    type Output = Scale<P1, -1>;
}

impl<P> MulScale<Scale<P, 1>> for Scale<P, -1>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> MulScale<Scale<P, 2>> for Scale<P, -1>
where
    P: Prefix,
{
    type Output = Scale<P, 1>;
}

impl<P> MulScale<Scale<P, 3>> for Scale<P, -1>
where
    P: Prefix,
{
    type Output = Scale<P, 2>;
}

impl<P> MulScale<Scale<P, 4>> for Scale<P, -1>
where
    P: Prefix,
{
    type Output = Scale<P, 3>;
}

impl<P> MulScale<Scale<P, 5>> for Scale<P, -1>
where
    P: Prefix,
{
    type Output = Scale<P, 4>;
}

impl<P> MulScale<Scale<P, 6>> for Scale<P, -1>
where
    P: Prefix,
{
    type Output = Scale<P, 5>;
}

impl<P> MulScale<Scale<P, 7>> for Scale<P, -1>
where
    P: Prefix,
{
    type Output = Scale<P, 6>;
}

impl<P> MulScale<Scale<P, 8>> for Scale<P, -1>
where
    P: Prefix,
{
    type Output = Scale<P, 7>;
}

impl<P> MulScale<Scale<P, 9>> for Scale<P, -1>
where
    P: Prefix,
{
    type Output = Scale<P, 8>;
}

impl<P> MulScale<Scale<P, 10>> for Scale<P, -1>
where
    P: Prefix,
{
    type Output = Scale<P, 9>;
}

//------------------------- 0 -------------------------

impl<P1, P2, const N: i32> MulScale<Scale<P2, N>> for Scale<P1, 0>
where
    P1: Prefix,
    P2: Prefix,
{
type Output = Scale<P2, N>;
}

//------------------------- 1 -------------------------

impl<P> MulScale<Scale<P, -10>> for Scale<P, 1>
where
    P: Prefix,
{
    type Output = Scale<P, -9>;
}

impl<P> MulScale<Scale<P, -9>> for Scale<P, 1>
where
    P: Prefix,
{
    type Output = Scale<P, -8>;
}

impl<P> MulScale<Scale<P, -8>> for Scale<P, 1>
where
    P: Prefix,
{
    type Output = Scale<P, -7>;
}

impl<P> MulScale<Scale<P, -7>> for Scale<P, 1>
where
    P: Prefix,
{
    type Output = Scale<P, -6>;
}

impl<P> MulScale<Scale<P, -6>> for Scale<P, 1>
where
    P: Prefix,
{
    type Output = Scale<P, -5>;
}

impl<P> MulScale<Scale<P, -5>> for Scale<P, 1>
where
    P: Prefix,
{
    type Output = Scale<P, -4>;
}

impl<P> MulScale<Scale<P, -4>> for Scale<P, 1>
where
    P: Prefix,
{
    type Output = Scale<P, -3>;
}

impl<P> MulScale<Scale<P, -3>> for Scale<P, 1>
where
    P: Prefix,
{
    type Output = Scale<P, -2>;
}

impl<P> MulScale<Scale<P, -2>> for Scale<P, 1>
where
    P: Prefix,
{
    type Output = Scale<P, -1>;
}

impl<P> MulScale<Scale<P, -1>> for Scale<P, 1>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P1, P2> MulScale<Scale<P2, 0>> for Scale<P1, 1>
where
    P1: Prefix,
    P2: Prefix,
{
    type Output = Scale<P1, 1>;
}

impl<P> MulScale<Scale<P, 1>> for Scale<P, 1>
where
    P: Prefix,
{
    type Output = Scale<P, 2>;
}

impl<P> MulScale<Scale<P, 2>> for Scale<P, 1>
where
    P: Prefix,
{
    type Output = Scale<P, 3>;
}

impl<P> MulScale<Scale<P, 3>> for Scale<P, 1>
where
    P: Prefix,
{
    type Output = Scale<P, 4>;
}

impl<P> MulScale<Scale<P, 4>> for Scale<P, 1>
where
    P: Prefix,
{
    type Output = Scale<P, 5>;
}

impl<P> MulScale<Scale<P, 5>> for Scale<P, 1>
where
    P: Prefix,
{
    type Output = Scale<P, 6>;
}

impl<P> MulScale<Scale<P, 6>> for Scale<P, 1>
where
    P: Prefix,
{
    type Output = Scale<P, 7>;
}

impl<P> MulScale<Scale<P, 7>> for Scale<P, 1>
where
    P: Prefix,
{
    type Output = Scale<P, 8>;
}

impl<P> MulScale<Scale<P, 8>> for Scale<P, 1>
where
    P: Prefix,
{
    type Output = Scale<P, 9>;
}

impl<P> MulScale<Scale<P, 9>> for Scale<P, 1>
where
    P: Prefix,
{
    type Output = Scale<P, 10>;
}

//------------------------- 2 -------------------------

impl<P> MulScale<Scale<P, -10>> for Scale<P, 2>
where
    P: Prefix,
{
    type Output = Scale<P, -8>;
}

impl<P> MulScale<Scale<P, -9>> for Scale<P, 2>
where
    P: Prefix,
{
    type Output = Scale<P, -7>;
}

impl<P> MulScale<Scale<P, -8>> for Scale<P, 2>
where
    P: Prefix,
{
    type Output = Scale<P, -6>;
}

impl<P> MulScale<Scale<P, -7>> for Scale<P, 2>
where
    P: Prefix,
{
    type Output = Scale<P, -5>;
}

impl<P> MulScale<Scale<P, -6>> for Scale<P, 2>
where
    P: Prefix,
{
    type Output = Scale<P, -4>;
}

impl<P> MulScale<Scale<P, -5>> for Scale<P, 2>
where
    P: Prefix,
{
    type Output = Scale<P, -3>;
}

impl<P> MulScale<Scale<P, -4>> for Scale<P, 2>
where
    P: Prefix,
{
    type Output = Scale<P, -2>;
}

impl<P> MulScale<Scale<P, -3>> for Scale<P, 2>
where
    P: Prefix,
{
    type Output = Scale<P, -1>;
}

impl<P> MulScale<Scale<P, -2>> for Scale<P, 2>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> MulScale<Scale<P, -1>> for Scale<P, 2>
where
    P: Prefix,
{
    type Output = Scale<P, 1>;
}

impl<P1, P2> MulScale<Scale<P2, 0>> for Scale<P1, 2>
where
    P1: Prefix,
    P2: Prefix,
{
    type Output = Scale<P1, 2>;
}

impl<P> MulScale<Scale<P, 1>> for Scale<P, 2>
where
    P: Prefix,
{
    type Output = Scale<P, 3>;
}

impl<P> MulScale<Scale<P, 2>> for Scale<P, 2>
where
    P: Prefix,
{
    type Output = Scale<P, 4>;
}

impl<P> MulScale<Scale<P, 3>> for Scale<P, 2>
where
    P: Prefix,
{
    type Output = Scale<P, 5>;
}

impl<P> MulScale<Scale<P, 4>> for Scale<P, 2>
where
    P: Prefix,
{
    type Output = Scale<P, 6>;
}

impl<P> MulScale<Scale<P, 5>> for Scale<P, 2>
where
    P: Prefix,
{
    type Output = Scale<P, 7>;
}

impl<P> MulScale<Scale<P, 6>> for Scale<P, 2>
where
    P: Prefix,
{
    type Output = Scale<P, 8>;
}

impl<P> MulScale<Scale<P, 7>> for Scale<P, 2>
where
    P: Prefix,
{
    type Output = Scale<P, 9>;
}

impl<P> MulScale<Scale<P, 8>> for Scale<P, 2>
where
    P: Prefix,
{
    type Output = Scale<P, 10>;
}

//------------------------- 3 -------------------------

impl<P> MulScale<Scale<P, -10>> for Scale<P, 3>
where
    P: Prefix,
{
    type Output = Scale<P, -7>;
}

impl<P> MulScale<Scale<P, -9>> for Scale<P, 3>
where
    P: Prefix,
{
    type Output = Scale<P, -6>;
}

impl<P> MulScale<Scale<P, -8>> for Scale<P, 3>
where
    P: Prefix,
{
    type Output = Scale<P, -5>;
}

impl<P> MulScale<Scale<P, -7>> for Scale<P, 3>
where
    P: Prefix,
{
    type Output = Scale<P, -4>;
}

impl<P> MulScale<Scale<P, -6>> for Scale<P, 3>
where
    P: Prefix,
{
    type Output = Scale<P, -3>;
}

impl<P> MulScale<Scale<P, -5>> for Scale<P, 3>
where
    P: Prefix,
{
    type Output = Scale<P, -2>;
}

impl<P> MulScale<Scale<P, -4>> for Scale<P, 3>
where
    P: Prefix,
{
    type Output = Scale<P, -1>;
}

impl<P> MulScale<Scale<P, -3>> for Scale<P, 3>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> MulScale<Scale<P, -2>> for Scale<P, 3>
where
    P: Prefix,
{
    type Output = Scale<P, 1>;
}

impl<P> MulScale<Scale<P, -1>> for Scale<P, 3>
where
    P: Prefix,
{
    type Output = Scale<P, 2>;
}

impl<P1, P2> MulScale<Scale<P2, 0>> for Scale<P1, 3>
where
    P1: Prefix,
    P2: Prefix,
{
    type Output = Scale<P1, 3>;
}

impl<P> MulScale<Scale<P, 1>> for Scale<P, 3>
where
    P: Prefix,
{
    type Output = Scale<P, 4>;
}

impl<P> MulScale<Scale<P, 2>> for Scale<P, 3>
where
    P: Prefix,
{
    type Output = Scale<P, 5>;
}

impl<P> MulScale<Scale<P, 3>> for Scale<P, 3>
where
    P: Prefix,
{
    type Output = Scale<P, 6>;
}

impl<P> MulScale<Scale<P, 4>> for Scale<P, 3>
where
    P: Prefix,
{
    type Output = Scale<P, 7>;
}

impl<P> MulScale<Scale<P, 5>> for Scale<P, 3>
where
    P: Prefix,
{
    type Output = Scale<P, 8>;
}

impl<P> MulScale<Scale<P, 6>> for Scale<P, 3>
where
    P: Prefix,
{
    type Output = Scale<P, 9>;
}

impl<P> MulScale<Scale<P, 7>> for Scale<P, 3>
where
    P: Prefix,
{
    type Output = Scale<P, 10>;
}

//------------------------- 4 -------------------------

impl<P> MulScale<Scale<P, -10>> for Scale<P, 4>
where
    P: Prefix,
{
    type Output = Scale<P, -6>;
}

impl<P> MulScale<Scale<P, -9>> for Scale<P, 4>
where
    P: Prefix,
{
    type Output = Scale<P, -5>;
}

impl<P> MulScale<Scale<P, -8>> for Scale<P, 4>
where
    P: Prefix,
{
    type Output = Scale<P, -4>;
}

impl<P> MulScale<Scale<P, -7>> for Scale<P, 4>
where
    P: Prefix,
{
    type Output = Scale<P, -3>;
}

impl<P> MulScale<Scale<P, -6>> for Scale<P, 4>
where
    P: Prefix,
{
    type Output = Scale<P, -2>;
}

impl<P> MulScale<Scale<P, -5>> for Scale<P, 4>
where
    P: Prefix,
{
    type Output = Scale<P, -1>;
}

impl<P> MulScale<Scale<P, -4>> for Scale<P, 4>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> MulScale<Scale<P, -3>> for Scale<P, 4>
where
    P: Prefix,
{
    type Output = Scale<P, 1>;
}

impl<P> MulScale<Scale<P, -2>> for Scale<P, 4>
where
    P: Prefix,
{
    type Output = Scale<P, 2>;
}

impl<P> MulScale<Scale<P, -1>> for Scale<P, 4>
where
    P: Prefix,
{
    type Output = Scale<P, 3>;
}

impl<P1, P2> MulScale<Scale<P2, 0>> for Scale<P1, 4>
where
    P1: Prefix,
    P2: Prefix,
{
    type Output = Scale<P1, 4>;
}

impl<P> MulScale<Scale<P, 1>> for Scale<P, 4>
where
    P: Prefix,
{
    type Output = Scale<P, 5>;
}

impl<P> MulScale<Scale<P, 2>> for Scale<P, 4>
where
    P: Prefix,
{
    type Output = Scale<P, 6>;
}

impl<P> MulScale<Scale<P, 3>> for Scale<P, 4>
where
    P: Prefix,
{
    type Output = Scale<P, 7>;
}

impl<P> MulScale<Scale<P, 4>> for Scale<P, 4>
where
    P: Prefix,
{
    type Output = Scale<P, 8>;
}

impl<P> MulScale<Scale<P, 5>> for Scale<P, 4>
where
    P: Prefix,
{
    type Output = Scale<P, 9>;
}

impl<P> MulScale<Scale<P, 6>> for Scale<P, 4>
where
    P: Prefix,
{
    type Output = Scale<P, 10>;
}

//------------------------- 5 -------------------------

impl<P> MulScale<Scale<P, -10>> for Scale<P, 5>
where
    P: Prefix,
{
    type Output = Scale<P, -5>;
}

impl<P> MulScale<Scale<P, -9>> for Scale<P, 5>
where
    P: Prefix,
{
    type Output = Scale<P, -4>;
}

impl<P> MulScale<Scale<P, -8>> for Scale<P, 5>
where
    P: Prefix,
{
    type Output = Scale<P, -3>;
}

impl<P> MulScale<Scale<P, -7>> for Scale<P, 5>
where
    P: Prefix,
{
    type Output = Scale<P, -2>;
}

impl<P> MulScale<Scale<P, -6>> for Scale<P, 5>
where
    P: Prefix,
{
    type Output = Scale<P, -1>;
}

impl<P> MulScale<Scale<P, -5>> for Scale<P, 5>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> MulScale<Scale<P, -4>> for Scale<P, 5>
where
    P: Prefix,
{
    type Output = Scale<P, 1>;
}

impl<P> MulScale<Scale<P, -3>> for Scale<P, 5>
where
    P: Prefix,
{
    type Output = Scale<P, 2>;
}

impl<P> MulScale<Scale<P, -2>> for Scale<P, 5>
where
    P: Prefix,
{
    type Output = Scale<P, 3>;
}

impl<P> MulScale<Scale<P, -1>> for Scale<P, 5>
where
    P: Prefix,
{
    type Output = Scale<P, 4>;
}

impl<P1, P2> MulScale<Scale<P2, 0>> for Scale<P1, 5>
where
    P1: Prefix,
    P2: Prefix,
{
    type Output = Scale<P1, 5>;
}

impl<P> MulScale<Scale<P, 1>> for Scale<P, 5>
where
    P: Prefix,
{
    type Output = Scale<P, 6>;
}

impl<P> MulScale<Scale<P, 2>> for Scale<P, 5>
where
    P: Prefix,
{
    type Output = Scale<P, 7>;
}

impl<P> MulScale<Scale<P, 3>> for Scale<P, 5>
where
    P: Prefix,
{
    type Output = Scale<P, 8>;
}

impl<P> MulScale<Scale<P, 4>> for Scale<P, 5>
where
    P: Prefix,
{
    type Output = Scale<P, 9>;
}

impl<P> MulScale<Scale<P, 5>> for Scale<P, 5>
where
    P: Prefix,
{
    type Output = Scale<P, 10>;
}

//------------------------- 6 -------------------------

impl<P> MulScale<Scale<P, -10>> for Scale<P, 6>
where
    P: Prefix,
{
    type Output = Scale<P, -4>;
}

impl<P> MulScale<Scale<P, -9>> for Scale<P, 6>
where
    P: Prefix,
{
    type Output = Scale<P, -3>;
}

impl<P> MulScale<Scale<P, -8>> for Scale<P, 6>
where
    P: Prefix,
{
    type Output = Scale<P, -2>;
}

impl<P> MulScale<Scale<P, -7>> for Scale<P, 6>
where
    P: Prefix,
{
    type Output = Scale<P, -1>;
}

impl<P> MulScale<Scale<P, -6>> for Scale<P, 6>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> MulScale<Scale<P, -5>> for Scale<P, 6>
where
    P: Prefix,
{
    type Output = Scale<P, 1>;
}

impl<P> MulScale<Scale<P, -4>> for Scale<P, 6>
where
    P: Prefix,
{
    type Output = Scale<P, 2>;
}

impl<P> MulScale<Scale<P, -3>> for Scale<P, 6>
where
    P: Prefix,
{
    type Output = Scale<P, 3>;
}

impl<P> MulScale<Scale<P, -2>> for Scale<P, 6>
where
    P: Prefix,
{
    type Output = Scale<P, 4>;
}

impl<P> MulScale<Scale<P, -1>> for Scale<P, 6>
where
    P: Prefix,
{
    type Output = Scale<P, 5>;
}

impl<P1, P2> MulScale<Scale<P2, 0>> for Scale<P1, 6>
where
    P1: Prefix,
    P2: Prefix,
{
    type Output = Scale<P1, 6>;
}

impl<P> MulScale<Scale<P, 1>> for Scale<P, 6>
where
    P: Prefix,
{
    type Output = Scale<P, 7>;
}

impl<P> MulScale<Scale<P, 2>> for Scale<P, 6>
where
    P: Prefix,
{
    type Output = Scale<P, 8>;
}

impl<P> MulScale<Scale<P, 3>> for Scale<P, 6>
where
    P: Prefix,
{
    type Output = Scale<P, 9>;
}

impl<P> MulScale<Scale<P, 4>> for Scale<P, 6>
where
    P: Prefix,
{
    type Output = Scale<P, 10>;
}

//------------------------- 7 -------------------------

impl<P> MulScale<Scale<P, -10>> for Scale<P, 7>
where
    P: Prefix,
{
    type Output = Scale<P, -3>;
}

impl<P> MulScale<Scale<P, -9>> for Scale<P, 7>
where
    P: Prefix,
{
    type Output = Scale<P, -2>;
}

impl<P> MulScale<Scale<P, -8>> for Scale<P, 7>
where
    P: Prefix,
{
    type Output = Scale<P, -1>;
}

impl<P> MulScale<Scale<P, -7>> for Scale<P, 7>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> MulScale<Scale<P, -6>> for Scale<P, 7>
where
    P: Prefix,
{
    type Output = Scale<P, 1>;
}

impl<P> MulScale<Scale<P, -5>> for Scale<P, 7>
where
    P: Prefix,
{
    type Output = Scale<P, 2>;
}

impl<P> MulScale<Scale<P, -4>> for Scale<P, 7>
where
    P: Prefix,
{
    type Output = Scale<P, 3>;
}

impl<P> MulScale<Scale<P, -3>> for Scale<P, 7>
where
    P: Prefix,
{
    type Output = Scale<P, 4>;
}

impl<P> MulScale<Scale<P, -2>> for Scale<P, 7>
where
    P: Prefix,
{
    type Output = Scale<P, 5>;
}

impl<P> MulScale<Scale<P, -1>> for Scale<P, 7>
where
    P: Prefix,
{
    type Output = Scale<P, 6>;
}

impl<P1, P2> MulScale<Scale<P2, 0>> for Scale<P1, 7>
where
    P1: Prefix,
    P2: Prefix,
{
    type Output = Scale<P1, 7>;
}

impl<P> MulScale<Scale<P, 1>> for Scale<P, 7>
where
    P: Prefix,
{
    type Output = Scale<P, 8>;
}

impl<P> MulScale<Scale<P, 2>> for Scale<P, 7>
where
    P: Prefix,
{
    type Output = Scale<P, 9>;
}

impl<P> MulScale<Scale<P, 3>> for Scale<P, 7>
where
    P: Prefix,
{
    type Output = Scale<P, 10>;
}

//------------------------- 8 -------------------------

impl<P> MulScale<Scale<P, -10>> for Scale<P, 8>
where
    P: Prefix,
{
    type Output = Scale<P, -2>;
}

impl<P> MulScale<Scale<P, -9>> for Scale<P, 8>
where
    P: Prefix,
{
    type Output = Scale<P, -1>;
}

impl<P> MulScale<Scale<P, -8>> for Scale<P, 8>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> MulScale<Scale<P, -7>> for Scale<P, 8>
where
    P: Prefix,
{
    type Output = Scale<P, 1>;
}

impl<P> MulScale<Scale<P, -6>> for Scale<P, 8>
where
    P: Prefix,
{
    type Output = Scale<P, 2>;
}

impl<P> MulScale<Scale<P, -5>> for Scale<P, 8>
where
    P: Prefix,
{
    type Output = Scale<P, 3>;
}

impl<P> MulScale<Scale<P, -4>> for Scale<P, 8>
where
    P: Prefix,
{
    type Output = Scale<P, 4>;
}

impl<P> MulScale<Scale<P, -3>> for Scale<P, 8>
where
    P: Prefix,
{
    type Output = Scale<P, 5>;
}

impl<P> MulScale<Scale<P, -2>> for Scale<P, 8>
where
    P: Prefix,
{
    type Output = Scale<P, 6>;
}

impl<P> MulScale<Scale<P, -1>> for Scale<P, 8>
where
    P: Prefix,
{
    type Output = Scale<P, 7>;
}

impl<P1, P2> MulScale<Scale<P2, 0>> for Scale<P1, 8>
where
    P1: Prefix,
    P2: Prefix,
{
    type Output = Scale<P1, 8>;
}

impl<P> MulScale<Scale<P, 1>> for Scale<P, 8>
where
    P: Prefix,
{
    type Output = Scale<P, 9>;
}

impl<P> MulScale<Scale<P, 2>> for Scale<P, 8>
where
    P: Prefix,
{
    type Output = Scale<P, 10>;
}

//------------------------- 9 -------------------------

impl<P> MulScale<Scale<P, -10>> for Scale<P, 9>
where
    P: Prefix,
{
    type Output = Scale<P, -1>;
}

impl<P> MulScale<Scale<P, -9>> for Scale<P, 9>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> MulScale<Scale<P, -8>> for Scale<P, 9>
where
    P: Prefix,
{
    type Output = Scale<P, 1>;
}

impl<P> MulScale<Scale<P, -7>> for Scale<P, 9>
where
    P: Prefix,
{
    type Output = Scale<P, 2>;
}

impl<P> MulScale<Scale<P, -6>> for Scale<P, 9>
where
    P: Prefix,
{
    type Output = Scale<P, 3>;
}

impl<P> MulScale<Scale<P, -5>> for Scale<P, 9>
where
    P: Prefix,
{
    type Output = Scale<P, 4>;
}

impl<P> MulScale<Scale<P, -4>> for Scale<P, 9>
where
    P: Prefix,
{
    type Output = Scale<P, 5>;
}

impl<P> MulScale<Scale<P, -3>> for Scale<P, 9>
where
    P: Prefix,
{
    type Output = Scale<P, 6>;
}

impl<P> MulScale<Scale<P, -2>> for Scale<P, 9>
where
    P: Prefix,
{
    type Output = Scale<P, 7>;
}

impl<P> MulScale<Scale<P, -1>> for Scale<P, 9>
where
    P: Prefix,
{
    type Output = Scale<P, 8>;
}

impl<P1, P2> MulScale<Scale<P2, 0>> for Scale<P1, 9>
where
    P1: Prefix,
    P2: Prefix,
{
    type Output = Scale<P1, 9>;
}

impl<P> MulScale<Scale<P, 1>> for Scale<P, 9>
where
    P: Prefix,
{
    type Output = Scale<P, 10>;
}

//------------------------- 10 -------------------------

impl<P> MulScale<Scale<P, -10>> for Scale<P, 10>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> MulScale<Scale<P, -9>> for Scale<P, 10>
where
    P: Prefix,
{
    type Output = Scale<P, 1>;
}

impl<P> MulScale<Scale<P, -8>> for Scale<P, 10>
where
    P: Prefix,
{
    type Output = Scale<P, 2>;
}

impl<P> MulScale<Scale<P, -7>> for Scale<P, 10>
where
    P: Prefix,
{
    type Output = Scale<P, 3>;
}

impl<P> MulScale<Scale<P, -6>> for Scale<P, 10>
where
    P: Prefix,
{
    type Output = Scale<P, 4>;
}

impl<P> MulScale<Scale<P, -5>> for Scale<P, 10>
where
    P: Prefix,
{
    type Output = Scale<P, 5>;
}

impl<P> MulScale<Scale<P, -4>> for Scale<P, 10>
where
    P: Prefix,
{
    type Output = Scale<P, 6>;
}

impl<P> MulScale<Scale<P, -3>> for Scale<P, 10>
where
    P: Prefix,
{
    type Output = Scale<P, 7>;
}

impl<P> MulScale<Scale<P, -2>> for Scale<P, 10>
where
    P: Prefix,
{
    type Output = Scale<P, 8>;
}

impl<P> MulScale<Scale<P, -1>> for Scale<P, 10>
where
    P: Prefix,
{
    type Output = Scale<P, 9>;
}

impl<P1, P2> MulScale<Scale<P2, 0>> for Scale<P1, 10>
where
    P1: Prefix,
    P2: Prefix,
{
    type Output = Scale<P1, 10>;
}
