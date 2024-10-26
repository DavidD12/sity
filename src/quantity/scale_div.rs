use super::*;


//------------------------- -10 -------------------------

impl<P> DivScale<Scale<P, -10>> for Scale<P, -10>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> DivScale<Scale<P, -9>> for Scale<P, -10>
where
    P: Prefix,
{
    type Output = Scale<P, -1>;
}

impl<P> DivScale<Scale<P, -8>> for Scale<P, -10>
where
    P: Prefix,
{
    type Output = Scale<P, -2>;
}

impl<P> DivScale<Scale<P, -7>> for Scale<P, -10>
where
    P: Prefix,
{
    type Output = Scale<P, -3>;
}

impl<P> DivScale<Scale<P, -6>> for Scale<P, -10>
where
    P: Prefix,
{
    type Output = Scale<P, -4>;
}

impl<P> DivScale<Scale<P, -5>> for Scale<P, -10>
where
    P: Prefix,
{
    type Output = Scale<P, -5>;
}

impl<P> DivScale<Scale<P, -4>> for Scale<P, -10>
where
    P: Prefix,
{
    type Output = Scale<P, -6>;
}

impl<P> DivScale<Scale<P, -3>> for Scale<P, -10>
where
    P: Prefix,
{
    type Output = Scale<P, -7>;
}

impl<P> DivScale<Scale<P, -2>> for Scale<P, -10>
where
    P: Prefix,
{
    type Output = Scale<P, -8>;
}

impl<P> DivScale<Scale<P, -1>> for Scale<P, -10>
where
    P: Prefix,
{
    type Output = Scale<P, -9>;
}

impl<P1, P2> DivScale<Scale<P2, 0>> for Scale<P1, -10>
where
    P1: Prefix,
    P2: Prefix,
{
    type Output = Scale<P1, -10>;
}

//------------------------- -9 -------------------------

impl<P> DivScale<Scale<P, -10>> for Scale<P, -9>
where
    P: Prefix,
{
    type Output = Scale<P, 1>;
}

impl<P> DivScale<Scale<P, -9>> for Scale<P, -9>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> DivScale<Scale<P, -8>> for Scale<P, -9>
where
    P: Prefix,
{
    type Output = Scale<P, -1>;
}

impl<P> DivScale<Scale<P, -7>> for Scale<P, -9>
where
    P: Prefix,
{
    type Output = Scale<P, -2>;
}

impl<P> DivScale<Scale<P, -6>> for Scale<P, -9>
where
    P: Prefix,
{
    type Output = Scale<P, -3>;
}

impl<P> DivScale<Scale<P, -5>> for Scale<P, -9>
where
    P: Prefix,
{
    type Output = Scale<P, -4>;
}

impl<P> DivScale<Scale<P, -4>> for Scale<P, -9>
where
    P: Prefix,
{
    type Output = Scale<P, -5>;
}

impl<P> DivScale<Scale<P, -3>> for Scale<P, -9>
where
    P: Prefix,
{
    type Output = Scale<P, -6>;
}

impl<P> DivScale<Scale<P, -2>> for Scale<P, -9>
where
    P: Prefix,
{
    type Output = Scale<P, -7>;
}

impl<P> DivScale<Scale<P, -1>> for Scale<P, -9>
where
    P: Prefix,
{
    type Output = Scale<P, -8>;
}

impl<P1, P2> DivScale<Scale<P2, 0>> for Scale<P1, -9>
where
    P1: Prefix,
    P2: Prefix,
{
    type Output = Scale<P1, -9>;
}

impl<P> DivScale<Scale<P, 1>> for Scale<P, -9>
where
    P: Prefix,
{
    type Output = Scale<P, -10>;
}

//------------------------- -8 -------------------------

impl<P> DivScale<Scale<P, -10>> for Scale<P, -8>
where
    P: Prefix,
{
    type Output = Scale<P, 2>;
}

impl<P> DivScale<Scale<P, -9>> for Scale<P, -8>
where
    P: Prefix,
{
    type Output = Scale<P, 1>;
}

impl<P> DivScale<Scale<P, -8>> for Scale<P, -8>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> DivScale<Scale<P, -7>> for Scale<P, -8>
where
    P: Prefix,
{
    type Output = Scale<P, -1>;
}

impl<P> DivScale<Scale<P, -6>> for Scale<P, -8>
where
    P: Prefix,
{
    type Output = Scale<P, -2>;
}

impl<P> DivScale<Scale<P, -5>> for Scale<P, -8>
where
    P: Prefix,
{
    type Output = Scale<P, -3>;
}

impl<P> DivScale<Scale<P, -4>> for Scale<P, -8>
where
    P: Prefix,
{
    type Output = Scale<P, -4>;
}

impl<P> DivScale<Scale<P, -3>> for Scale<P, -8>
where
    P: Prefix,
{
    type Output = Scale<P, -5>;
}

impl<P> DivScale<Scale<P, -2>> for Scale<P, -8>
where
    P: Prefix,
{
    type Output = Scale<P, -6>;
}

impl<P> DivScale<Scale<P, -1>> for Scale<P, -8>
where
    P: Prefix,
{
    type Output = Scale<P, -7>;
}

impl<P1, P2> DivScale<Scale<P2, 0>> for Scale<P1, -8>
where
    P1: Prefix,
    P2: Prefix,
{
    type Output = Scale<P1, -8>;
}

impl<P> DivScale<Scale<P, 1>> for Scale<P, -8>
where
    P: Prefix,
{
    type Output = Scale<P, -9>;
}

impl<P> DivScale<Scale<P, 2>> for Scale<P, -8>
where
    P: Prefix,
{
    type Output = Scale<P, -10>;
}

//------------------------- -7 -------------------------

impl<P> DivScale<Scale<P, -10>> for Scale<P, -7>
where
    P: Prefix,
{
    type Output = Scale<P, 3>;
}

impl<P> DivScale<Scale<P, -9>> for Scale<P, -7>
where
    P: Prefix,
{
    type Output = Scale<P, 2>;
}

impl<P> DivScale<Scale<P, -8>> for Scale<P, -7>
where
    P: Prefix,
{
    type Output = Scale<P, 1>;
}

impl<P> DivScale<Scale<P, -7>> for Scale<P, -7>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> DivScale<Scale<P, -6>> for Scale<P, -7>
where
    P: Prefix,
{
    type Output = Scale<P, -1>;
}

impl<P> DivScale<Scale<P, -5>> for Scale<P, -7>
where
    P: Prefix,
{
    type Output = Scale<P, -2>;
}

impl<P> DivScale<Scale<P, -4>> for Scale<P, -7>
where
    P: Prefix,
{
    type Output = Scale<P, -3>;
}

impl<P> DivScale<Scale<P, -3>> for Scale<P, -7>
where
    P: Prefix,
{
    type Output = Scale<P, -4>;
}

impl<P> DivScale<Scale<P, -2>> for Scale<P, -7>
where
    P: Prefix,
{
    type Output = Scale<P, -5>;
}

impl<P> DivScale<Scale<P, -1>> for Scale<P, -7>
where
    P: Prefix,
{
    type Output = Scale<P, -6>;
}

impl<P1, P2> DivScale<Scale<P2, 0>> for Scale<P1, -7>
where
    P1: Prefix,
    P2: Prefix,
{
    type Output = Scale<P1, -7>;
}

impl<P> DivScale<Scale<P, 1>> for Scale<P, -7>
where
    P: Prefix,
{
    type Output = Scale<P, -8>;
}

impl<P> DivScale<Scale<P, 2>> for Scale<P, -7>
where
    P: Prefix,
{
    type Output = Scale<P, -9>;
}

impl<P> DivScale<Scale<P, 3>> for Scale<P, -7>
where
    P: Prefix,
{
    type Output = Scale<P, -10>;
}

//------------------------- -6 -------------------------

impl<P> DivScale<Scale<P, -10>> for Scale<P, -6>
where
    P: Prefix,
{
    type Output = Scale<P, 4>;
}

impl<P> DivScale<Scale<P, -9>> for Scale<P, -6>
where
    P: Prefix,
{
    type Output = Scale<P, 3>;
}

impl<P> DivScale<Scale<P, -8>> for Scale<P, -6>
where
    P: Prefix,
{
    type Output = Scale<P, 2>;
}

impl<P> DivScale<Scale<P, -7>> for Scale<P, -6>
where
    P: Prefix,
{
    type Output = Scale<P, 1>;
}

impl<P> DivScale<Scale<P, -6>> for Scale<P, -6>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> DivScale<Scale<P, -5>> for Scale<P, -6>
where
    P: Prefix,
{
    type Output = Scale<P, -1>;
}

impl<P> DivScale<Scale<P, -4>> for Scale<P, -6>
where
    P: Prefix,
{
    type Output = Scale<P, -2>;
}

impl<P> DivScale<Scale<P, -3>> for Scale<P, -6>
where
    P: Prefix,
{
    type Output = Scale<P, -3>;
}

impl<P> DivScale<Scale<P, -2>> for Scale<P, -6>
where
    P: Prefix,
{
    type Output = Scale<P, -4>;
}

impl<P> DivScale<Scale<P, -1>> for Scale<P, -6>
where
    P: Prefix,
{
    type Output = Scale<P, -5>;
}

impl<P1, P2> DivScale<Scale<P2, 0>> for Scale<P1, -6>
where
    P1: Prefix,
    P2: Prefix,
{
    type Output = Scale<P1, -6>;
}

impl<P> DivScale<Scale<P, 1>> for Scale<P, -6>
where
    P: Prefix,
{
    type Output = Scale<P, -7>;
}

impl<P> DivScale<Scale<P, 2>> for Scale<P, -6>
where
    P: Prefix,
{
    type Output = Scale<P, -8>;
}

impl<P> DivScale<Scale<P, 3>> for Scale<P, -6>
where
    P: Prefix,
{
    type Output = Scale<P, -9>;
}

impl<P> DivScale<Scale<P, 4>> for Scale<P, -6>
where
    P: Prefix,
{
    type Output = Scale<P, -10>;
}

//------------------------- -5 -------------------------

impl<P> DivScale<Scale<P, -10>> for Scale<P, -5>
where
    P: Prefix,
{
    type Output = Scale<P, 5>;
}

impl<P> DivScale<Scale<P, -9>> for Scale<P, -5>
where
    P: Prefix,
{
    type Output = Scale<P, 4>;
}

impl<P> DivScale<Scale<P, -8>> for Scale<P, -5>
where
    P: Prefix,
{
    type Output = Scale<P, 3>;
}

impl<P> DivScale<Scale<P, -7>> for Scale<P, -5>
where
    P: Prefix,
{
    type Output = Scale<P, 2>;
}

impl<P> DivScale<Scale<P, -6>> for Scale<P, -5>
where
    P: Prefix,
{
    type Output = Scale<P, 1>;
}

impl<P> DivScale<Scale<P, -5>> for Scale<P, -5>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> DivScale<Scale<P, -4>> for Scale<P, -5>
where
    P: Prefix,
{
    type Output = Scale<P, -1>;
}

impl<P> DivScale<Scale<P, -3>> for Scale<P, -5>
where
    P: Prefix,
{
    type Output = Scale<P, -2>;
}

impl<P> DivScale<Scale<P, -2>> for Scale<P, -5>
where
    P: Prefix,
{
    type Output = Scale<P, -3>;
}

impl<P> DivScale<Scale<P, -1>> for Scale<P, -5>
where
    P: Prefix,
{
    type Output = Scale<P, -4>;
}

impl<P1, P2> DivScale<Scale<P2, 0>> for Scale<P1, -5>
where
    P1: Prefix,
    P2: Prefix,
{
    type Output = Scale<P1, -5>;
}

impl<P> DivScale<Scale<P, 1>> for Scale<P, -5>
where
    P: Prefix,
{
    type Output = Scale<P, -6>;
}

impl<P> DivScale<Scale<P, 2>> for Scale<P, -5>
where
    P: Prefix,
{
    type Output = Scale<P, -7>;
}

impl<P> DivScale<Scale<P, 3>> for Scale<P, -5>
where
    P: Prefix,
{
    type Output = Scale<P, -8>;
}

impl<P> DivScale<Scale<P, 4>> for Scale<P, -5>
where
    P: Prefix,
{
    type Output = Scale<P, -9>;
}

impl<P> DivScale<Scale<P, 5>> for Scale<P, -5>
where
    P: Prefix,
{
    type Output = Scale<P, -10>;
}

//------------------------- -4 -------------------------

impl<P> DivScale<Scale<P, -10>> for Scale<P, -4>
where
    P: Prefix,
{
    type Output = Scale<P, 6>;
}

impl<P> DivScale<Scale<P, -9>> for Scale<P, -4>
where
    P: Prefix,
{
    type Output = Scale<P, 5>;
}

impl<P> DivScale<Scale<P, -8>> for Scale<P, -4>
where
    P: Prefix,
{
    type Output = Scale<P, 4>;
}

impl<P> DivScale<Scale<P, -7>> for Scale<P, -4>
where
    P: Prefix,
{
    type Output = Scale<P, 3>;
}

impl<P> DivScale<Scale<P, -6>> for Scale<P, -4>
where
    P: Prefix,
{
    type Output = Scale<P, 2>;
}

impl<P> DivScale<Scale<P, -5>> for Scale<P, -4>
where
    P: Prefix,
{
    type Output = Scale<P, 1>;
}

impl<P> DivScale<Scale<P, -4>> for Scale<P, -4>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> DivScale<Scale<P, -3>> for Scale<P, -4>
where
    P: Prefix,
{
    type Output = Scale<P, -1>;
}

impl<P> DivScale<Scale<P, -2>> for Scale<P, -4>
where
    P: Prefix,
{
    type Output = Scale<P, -2>;
}

impl<P> DivScale<Scale<P, -1>> for Scale<P, -4>
where
    P: Prefix,
{
    type Output = Scale<P, -3>;
}

impl<P1, P2> DivScale<Scale<P2, 0>> for Scale<P1, -4>
where
    P1: Prefix,
    P2: Prefix,
{
    type Output = Scale<P1, -4>;
}

impl<P> DivScale<Scale<P, 1>> for Scale<P, -4>
where
    P: Prefix,
{
    type Output = Scale<P, -5>;
}

impl<P> DivScale<Scale<P, 2>> for Scale<P, -4>
where
    P: Prefix,
{
    type Output = Scale<P, -6>;
}

impl<P> DivScale<Scale<P, 3>> for Scale<P, -4>
where
    P: Prefix,
{
    type Output = Scale<P, -7>;
}

impl<P> DivScale<Scale<P, 4>> for Scale<P, -4>
where
    P: Prefix,
{
    type Output = Scale<P, -8>;
}

impl<P> DivScale<Scale<P, 5>> for Scale<P, -4>
where
    P: Prefix,
{
    type Output = Scale<P, -9>;
}

impl<P> DivScale<Scale<P, 6>> for Scale<P, -4>
where
    P: Prefix,
{
    type Output = Scale<P, -10>;
}

//------------------------- -3 -------------------------

impl<P> DivScale<Scale<P, -10>> for Scale<P, -3>
where
    P: Prefix,
{
    type Output = Scale<P, 7>;
}

impl<P> DivScale<Scale<P, -9>> for Scale<P, -3>
where
    P: Prefix,
{
    type Output = Scale<P, 6>;
}

impl<P> DivScale<Scale<P, -8>> for Scale<P, -3>
where
    P: Prefix,
{
    type Output = Scale<P, 5>;
}

impl<P> DivScale<Scale<P, -7>> for Scale<P, -3>
where
    P: Prefix,
{
    type Output = Scale<P, 4>;
}

impl<P> DivScale<Scale<P, -6>> for Scale<P, -3>
where
    P: Prefix,
{
    type Output = Scale<P, 3>;
}

impl<P> DivScale<Scale<P, -5>> for Scale<P, -3>
where
    P: Prefix,
{
    type Output = Scale<P, 2>;
}

impl<P> DivScale<Scale<P, -4>> for Scale<P, -3>
where
    P: Prefix,
{
    type Output = Scale<P, 1>;
}

impl<P> DivScale<Scale<P, -3>> for Scale<P, -3>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> DivScale<Scale<P, -2>> for Scale<P, -3>
where
    P: Prefix,
{
    type Output = Scale<P, -1>;
}

impl<P> DivScale<Scale<P, -1>> for Scale<P, -3>
where
    P: Prefix,
{
    type Output = Scale<P, -2>;
}

impl<P1, P2> DivScale<Scale<P2, 0>> for Scale<P1, -3>
where
    P1: Prefix,
    P2: Prefix,
{
    type Output = Scale<P1, -3>;
}

impl<P> DivScale<Scale<P, 1>> for Scale<P, -3>
where
    P: Prefix,
{
    type Output = Scale<P, -4>;
}

impl<P> DivScale<Scale<P, 2>> for Scale<P, -3>
where
    P: Prefix,
{
    type Output = Scale<P, -5>;
}

impl<P> DivScale<Scale<P, 3>> for Scale<P, -3>
where
    P: Prefix,
{
    type Output = Scale<P, -6>;
}

impl<P> DivScale<Scale<P, 4>> for Scale<P, -3>
where
    P: Prefix,
{
    type Output = Scale<P, -7>;
}

impl<P> DivScale<Scale<P, 5>> for Scale<P, -3>
where
    P: Prefix,
{
    type Output = Scale<P, -8>;
}

impl<P> DivScale<Scale<P, 6>> for Scale<P, -3>
where
    P: Prefix,
{
    type Output = Scale<P, -9>;
}

impl<P> DivScale<Scale<P, 7>> for Scale<P, -3>
where
    P: Prefix,
{
    type Output = Scale<P, -10>;
}

//------------------------- -2 -------------------------

impl<P> DivScale<Scale<P, -10>> for Scale<P, -2>
where
    P: Prefix,
{
    type Output = Scale<P, 8>;
}

impl<P> DivScale<Scale<P, -9>> for Scale<P, -2>
where
    P: Prefix,
{
    type Output = Scale<P, 7>;
}

impl<P> DivScale<Scale<P, -8>> for Scale<P, -2>
where
    P: Prefix,
{
    type Output = Scale<P, 6>;
}

impl<P> DivScale<Scale<P, -7>> for Scale<P, -2>
where
    P: Prefix,
{
    type Output = Scale<P, 5>;
}

impl<P> DivScale<Scale<P, -6>> for Scale<P, -2>
where
    P: Prefix,
{
    type Output = Scale<P, 4>;
}

impl<P> DivScale<Scale<P, -5>> for Scale<P, -2>
where
    P: Prefix,
{
    type Output = Scale<P, 3>;
}

impl<P> DivScale<Scale<P, -4>> for Scale<P, -2>
where
    P: Prefix,
{
    type Output = Scale<P, 2>;
}

impl<P> DivScale<Scale<P, -3>> for Scale<P, -2>
where
    P: Prefix,
{
    type Output = Scale<P, 1>;
}

impl<P> DivScale<Scale<P, -2>> for Scale<P, -2>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> DivScale<Scale<P, -1>> for Scale<P, -2>
where
    P: Prefix,
{
    type Output = Scale<P, -1>;
}

impl<P1, P2> DivScale<Scale<P2, 0>> for Scale<P1, -2>
where
    P1: Prefix,
    P2: Prefix,
{
    type Output = Scale<P1, -2>;
}

impl<P> DivScale<Scale<P, 1>> for Scale<P, -2>
where
    P: Prefix,
{
    type Output = Scale<P, -3>;
}

impl<P> DivScale<Scale<P, 2>> for Scale<P, -2>
where
    P: Prefix,
{
    type Output = Scale<P, -4>;
}

impl<P> DivScale<Scale<P, 3>> for Scale<P, -2>
where
    P: Prefix,
{
    type Output = Scale<P, -5>;
}

impl<P> DivScale<Scale<P, 4>> for Scale<P, -2>
where
    P: Prefix,
{
    type Output = Scale<P, -6>;
}

impl<P> DivScale<Scale<P, 5>> for Scale<P, -2>
where
    P: Prefix,
{
    type Output = Scale<P, -7>;
}

impl<P> DivScale<Scale<P, 6>> for Scale<P, -2>
where
    P: Prefix,
{
    type Output = Scale<P, -8>;
}

impl<P> DivScale<Scale<P, 7>> for Scale<P, -2>
where
    P: Prefix,
{
    type Output = Scale<P, -9>;
}

impl<P> DivScale<Scale<P, 8>> for Scale<P, -2>
where
    P: Prefix,
{
    type Output = Scale<P, -10>;
}

//------------------------- -1 -------------------------

impl<P> DivScale<Scale<P, -10>> for Scale<P, -1>
where
    P: Prefix,
{
    type Output = Scale<P, 9>;
}

impl<P> DivScale<Scale<P, -9>> for Scale<P, -1>
where
    P: Prefix,
{
    type Output = Scale<P, 8>;
}

impl<P> DivScale<Scale<P, -8>> for Scale<P, -1>
where
    P: Prefix,
{
    type Output = Scale<P, 7>;
}

impl<P> DivScale<Scale<P, -7>> for Scale<P, -1>
where
    P: Prefix,
{
    type Output = Scale<P, 6>;
}

impl<P> DivScale<Scale<P, -6>> for Scale<P, -1>
where
    P: Prefix,
{
    type Output = Scale<P, 5>;
}

impl<P> DivScale<Scale<P, -5>> for Scale<P, -1>
where
    P: Prefix,
{
    type Output = Scale<P, 4>;
}

impl<P> DivScale<Scale<P, -4>> for Scale<P, -1>
where
    P: Prefix,
{
    type Output = Scale<P, 3>;
}

impl<P> DivScale<Scale<P, -3>> for Scale<P, -1>
where
    P: Prefix,
{
    type Output = Scale<P, 2>;
}

impl<P> DivScale<Scale<P, -2>> for Scale<P, -1>
where
    P: Prefix,
{
    type Output = Scale<P, 1>;
}

impl<P> DivScale<Scale<P, -1>> for Scale<P, -1>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P1, P2> DivScale<Scale<P2, 0>> for Scale<P1, -1>
where
    P1: Prefix,
    P2: Prefix,
{
    type Output = Scale<P1, -1>;
}

impl<P> DivScale<Scale<P, 1>> for Scale<P, -1>
where
    P: Prefix,
{
    type Output = Scale<P, -2>;
}

impl<P> DivScale<Scale<P, 2>> for Scale<P, -1>
where
    P: Prefix,
{
    type Output = Scale<P, -3>;
}

impl<P> DivScale<Scale<P, 3>> for Scale<P, -1>
where
    P: Prefix,
{
    type Output = Scale<P, -4>;
}

impl<P> DivScale<Scale<P, 4>> for Scale<P, -1>
where
    P: Prefix,
{
    type Output = Scale<P, -5>;
}

impl<P> DivScale<Scale<P, 5>> for Scale<P, -1>
where
    P: Prefix,
{
    type Output = Scale<P, -6>;
}

impl<P> DivScale<Scale<P, 6>> for Scale<P, -1>
where
    P: Prefix,
{
    type Output = Scale<P, -7>;
}

impl<P> DivScale<Scale<P, 7>> for Scale<P, -1>
where
    P: Prefix,
{
    type Output = Scale<P, -8>;
}

impl<P> DivScale<Scale<P, 8>> for Scale<P, -1>
where
    P: Prefix,
{
    type Output = Scale<P, -9>;
}

impl<P> DivScale<Scale<P, 9>> for Scale<P, -1>
where
    P: Prefix,
{
    type Output = Scale<P, -10>;
}

//------------------------- 0 -------------------------

impl<P1, P2> DivScale<Scale<P2, -10>> for Scale<P1, 0>
where
    P1: Prefix,
    P2: Prefix,
{
    type Output = Scale<P2, 10>;
}

impl<P1, P2> DivScale<Scale<P2, -9>> for Scale<P1, 0>
where
    P1: Prefix,
    P2: Prefix,
{
    type Output = Scale<P2, 9>;
}

impl<P1, P2> DivScale<Scale<P2, -8>> for Scale<P1, 0>
where
    P1: Prefix,
    P2: Prefix,
{
    type Output = Scale<P2, 8>;
}

impl<P1, P2> DivScale<Scale<P2, -7>> for Scale<P1, 0>
where
    P1: Prefix,
    P2: Prefix,
{
    type Output = Scale<P2, 7>;
}

impl<P1, P2> DivScale<Scale<P2, -6>> for Scale<P1, 0>
where
    P1: Prefix,
    P2: Prefix,
{
    type Output = Scale<P2, 6>;
}

impl<P1, P2> DivScale<Scale<P2, -5>> for Scale<P1, 0>
where
    P1: Prefix,
    P2: Prefix,
{
    type Output = Scale<P2, 5>;
}

impl<P1, P2> DivScale<Scale<P2, -4>> for Scale<P1, 0>
where
    P1: Prefix,
    P2: Prefix,
{
    type Output = Scale<P2, 4>;
}

impl<P1, P2> DivScale<Scale<P2, -3>> for Scale<P1, 0>
where
    P1: Prefix,
    P2: Prefix,
{
    type Output = Scale<P2, 3>;
}

impl<P1, P2> DivScale<Scale<P2, -2>> for Scale<P1, 0>
where
    P1: Prefix,
    P2: Prefix,
{
    type Output = Scale<P2, 2>;
}

impl<P1, P2> DivScale<Scale<P2, -1>> for Scale<P1, 0>
where
    P1: Prefix,
    P2: Prefix,
{
    type Output = Scale<P2, 1>;
}

impl<P1, P2> DivScale<Scale<P2, 0>> for Scale<P1, 0>
where
    P1: Prefix,
    P2: Prefix,
{
    type Output = Scale<P2, 0>;
}

impl<P1, P2> DivScale<Scale<P2, 1>> for Scale<P1, 0>
where
    P1: Prefix,
    P2: Prefix,
{
    type Output = Scale<P2, -1>;
}

impl<P1, P2> DivScale<Scale<P2, 2>> for Scale<P1, 0>
where
    P1: Prefix,
    P2: Prefix,
{
    type Output = Scale<P2, -2>;
}

impl<P1, P2> DivScale<Scale<P2, 3>> for Scale<P1, 0>
where
    P1: Prefix,
    P2: Prefix,
{
    type Output = Scale<P2, -3>;
}

impl<P1, P2> DivScale<Scale<P2, 4>> for Scale<P1, 0>
where
    P1: Prefix,
    P2: Prefix,
{
    type Output = Scale<P2, -4>;
}

impl<P1, P2> DivScale<Scale<P2, 5>> for Scale<P1, 0>
where
    P1: Prefix,
    P2: Prefix,
{
    type Output = Scale<P2, -5>;
}

impl<P1, P2> DivScale<Scale<P2, 6>> for Scale<P1, 0>
where
    P1: Prefix,
    P2: Prefix,
{
    type Output = Scale<P2, -6>;
}

impl<P1, P2> DivScale<Scale<P2, 7>> for Scale<P1, 0>
where
    P1: Prefix,
    P2: Prefix,
{
    type Output = Scale<P2, -7>;
}

impl<P1, P2> DivScale<Scale<P2, 8>> for Scale<P1, 0>
where
    P1: Prefix,
    P2: Prefix,
{
    type Output = Scale<P2, -8>;
}

impl<P1, P2> DivScale<Scale<P2, 9>> for Scale<P1, 0>
where
    P1: Prefix,
    P2: Prefix,
{
    type Output = Scale<P2, -9>;
}

impl<P1, P2> DivScale<Scale<P2, 10>> for Scale<P1, 0>
where
    P1: Prefix,
    P2: Prefix,
{
    type Output = Scale<P2, -10>;
}

//------------------------- 1 -------------------------

impl<P> DivScale<Scale<P, -9>> for Scale<P, 1>
where
    P: Prefix,
{
    type Output = Scale<P, 10>;
}

impl<P> DivScale<Scale<P, -8>> for Scale<P, 1>
where
    P: Prefix,
{
    type Output = Scale<P, 9>;
}

impl<P> DivScale<Scale<P, -7>> for Scale<P, 1>
where
    P: Prefix,
{
    type Output = Scale<P, 8>;
}

impl<P> DivScale<Scale<P, -6>> for Scale<P, 1>
where
    P: Prefix,
{
    type Output = Scale<P, 7>;
}

impl<P> DivScale<Scale<P, -5>> for Scale<P, 1>
where
    P: Prefix,
{
    type Output = Scale<P, 6>;
}

impl<P> DivScale<Scale<P, -4>> for Scale<P, 1>
where
    P: Prefix,
{
    type Output = Scale<P, 5>;
}

impl<P> DivScale<Scale<P, -3>> for Scale<P, 1>
where
    P: Prefix,
{
    type Output = Scale<P, 4>;
}

impl<P> DivScale<Scale<P, -2>> for Scale<P, 1>
where
    P: Prefix,
{
    type Output = Scale<P, 3>;
}

impl<P> DivScale<Scale<P, -1>> for Scale<P, 1>
where
    P: Prefix,
{
    type Output = Scale<P, 2>;
}

impl<P1, P2> DivScale<Scale<P2, 0>> for Scale<P1, 1>
where
    P1: Prefix,
    P2: Prefix,
{
    type Output = Scale<P1, 1>;
}

impl<P> DivScale<Scale<P, 1>> for Scale<P, 1>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> DivScale<Scale<P, 2>> for Scale<P, 1>
where
    P: Prefix,
{
    type Output = Scale<P, -1>;
}

impl<P> DivScale<Scale<P, 3>> for Scale<P, 1>
where
    P: Prefix,
{
    type Output = Scale<P, -2>;
}

impl<P> DivScale<Scale<P, 4>> for Scale<P, 1>
where
    P: Prefix,
{
    type Output = Scale<P, -3>;
}

impl<P> DivScale<Scale<P, 5>> for Scale<P, 1>
where
    P: Prefix,
{
    type Output = Scale<P, -4>;
}

impl<P> DivScale<Scale<P, 6>> for Scale<P, 1>
where
    P: Prefix,
{
    type Output = Scale<P, -5>;
}

impl<P> DivScale<Scale<P, 7>> for Scale<P, 1>
where
    P: Prefix,
{
    type Output = Scale<P, -6>;
}

impl<P> DivScale<Scale<P, 8>> for Scale<P, 1>
where
    P: Prefix,
{
    type Output = Scale<P, -7>;
}

impl<P> DivScale<Scale<P, 9>> for Scale<P, 1>
where
    P: Prefix,
{
    type Output = Scale<P, -8>;
}

impl<P> DivScale<Scale<P, 10>> for Scale<P, 1>
where
    P: Prefix,
{
    type Output = Scale<P, -9>;
}

//------------------------- 2 -------------------------

impl<P> DivScale<Scale<P, -8>> for Scale<P, 2>
where
    P: Prefix,
{
    type Output = Scale<P, 10>;
}

impl<P> DivScale<Scale<P, -7>> for Scale<P, 2>
where
    P: Prefix,
{
    type Output = Scale<P, 9>;
}

impl<P> DivScale<Scale<P, -6>> for Scale<P, 2>
where
    P: Prefix,
{
    type Output = Scale<P, 8>;
}

impl<P> DivScale<Scale<P, -5>> for Scale<P, 2>
where
    P: Prefix,
{
    type Output = Scale<P, 7>;
}

impl<P> DivScale<Scale<P, -4>> for Scale<P, 2>
where
    P: Prefix,
{
    type Output = Scale<P, 6>;
}

impl<P> DivScale<Scale<P, -3>> for Scale<P, 2>
where
    P: Prefix,
{
    type Output = Scale<P, 5>;
}

impl<P> DivScale<Scale<P, -2>> for Scale<P, 2>
where
    P: Prefix,
{
    type Output = Scale<P, 4>;
}

impl<P> DivScale<Scale<P, -1>> for Scale<P, 2>
where
    P: Prefix,
{
    type Output = Scale<P, 3>;
}

impl<P1, P2> DivScale<Scale<P2, 0>> for Scale<P1, 2>
where
    P1: Prefix,
    P2: Prefix,
{
    type Output = Scale<P1, 2>;
}

impl<P> DivScale<Scale<P, 1>> for Scale<P, 2>
where
    P: Prefix,
{
    type Output = Scale<P, 1>;
}

impl<P> DivScale<Scale<P, 2>> for Scale<P, 2>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> DivScale<Scale<P, 3>> for Scale<P, 2>
where
    P: Prefix,
{
    type Output = Scale<P, -1>;
}

impl<P> DivScale<Scale<P, 4>> for Scale<P, 2>
where
    P: Prefix,
{
    type Output = Scale<P, -2>;
}

impl<P> DivScale<Scale<P, 5>> for Scale<P, 2>
where
    P: Prefix,
{
    type Output = Scale<P, -3>;
}

impl<P> DivScale<Scale<P, 6>> for Scale<P, 2>
where
    P: Prefix,
{
    type Output = Scale<P, -4>;
}

impl<P> DivScale<Scale<P, 7>> for Scale<P, 2>
where
    P: Prefix,
{
    type Output = Scale<P, -5>;
}

impl<P> DivScale<Scale<P, 8>> for Scale<P, 2>
where
    P: Prefix,
{
    type Output = Scale<P, -6>;
}

impl<P> DivScale<Scale<P, 9>> for Scale<P, 2>
where
    P: Prefix,
{
    type Output = Scale<P, -7>;
}

impl<P> DivScale<Scale<P, 10>> for Scale<P, 2>
where
    P: Prefix,
{
    type Output = Scale<P, -8>;
}

//------------------------- 3 -------------------------

impl<P> DivScale<Scale<P, -7>> for Scale<P, 3>
where
    P: Prefix,
{
    type Output = Scale<P, 10>;
}

impl<P> DivScale<Scale<P, -6>> for Scale<P, 3>
where
    P: Prefix,
{
    type Output = Scale<P, 9>;
}

impl<P> DivScale<Scale<P, -5>> for Scale<P, 3>
where
    P: Prefix,
{
    type Output = Scale<P, 8>;
}

impl<P> DivScale<Scale<P, -4>> for Scale<P, 3>
where
    P: Prefix,
{
    type Output = Scale<P, 7>;
}

impl<P> DivScale<Scale<P, -3>> for Scale<P, 3>
where
    P: Prefix,
{
    type Output = Scale<P, 6>;
}

impl<P> DivScale<Scale<P, -2>> for Scale<P, 3>
where
    P: Prefix,
{
    type Output = Scale<P, 5>;
}

impl<P> DivScale<Scale<P, -1>> for Scale<P, 3>
where
    P: Prefix,
{
    type Output = Scale<P, 4>;
}

impl<P1, P2> DivScale<Scale<P2, 0>> for Scale<P1, 3>
where
    P1: Prefix,
    P2: Prefix,
{
    type Output = Scale<P1, 3>;
}

impl<P> DivScale<Scale<P, 1>> for Scale<P, 3>
where
    P: Prefix,
{
    type Output = Scale<P, 2>;
}

impl<P> DivScale<Scale<P, 2>> for Scale<P, 3>
where
    P: Prefix,
{
    type Output = Scale<P, 1>;
}

impl<P> DivScale<Scale<P, 3>> for Scale<P, 3>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> DivScale<Scale<P, 4>> for Scale<P, 3>
where
    P: Prefix,
{
    type Output = Scale<P, -1>;
}

impl<P> DivScale<Scale<P, 5>> for Scale<P, 3>
where
    P: Prefix,
{
    type Output = Scale<P, -2>;
}

impl<P> DivScale<Scale<P, 6>> for Scale<P, 3>
where
    P: Prefix,
{
    type Output = Scale<P, -3>;
}

impl<P> DivScale<Scale<P, 7>> for Scale<P, 3>
where
    P: Prefix,
{
    type Output = Scale<P, -4>;
}

impl<P> DivScale<Scale<P, 8>> for Scale<P, 3>
where
    P: Prefix,
{
    type Output = Scale<P, -5>;
}

impl<P> DivScale<Scale<P, 9>> for Scale<P, 3>
where
    P: Prefix,
{
    type Output = Scale<P, -6>;
}

impl<P> DivScale<Scale<P, 10>> for Scale<P, 3>
where
    P: Prefix,
{
    type Output = Scale<P, -7>;
}

//------------------------- 4 -------------------------

impl<P> DivScale<Scale<P, -6>> for Scale<P, 4>
where
    P: Prefix,
{
    type Output = Scale<P, 10>;
}

impl<P> DivScale<Scale<P, -5>> for Scale<P, 4>
where
    P: Prefix,
{
    type Output = Scale<P, 9>;
}

impl<P> DivScale<Scale<P, -4>> for Scale<P, 4>
where
    P: Prefix,
{
    type Output = Scale<P, 8>;
}

impl<P> DivScale<Scale<P, -3>> for Scale<P, 4>
where
    P: Prefix,
{
    type Output = Scale<P, 7>;
}

impl<P> DivScale<Scale<P, -2>> for Scale<P, 4>
where
    P: Prefix,
{
    type Output = Scale<P, 6>;
}

impl<P> DivScale<Scale<P, -1>> for Scale<P, 4>
where
    P: Prefix,
{
    type Output = Scale<P, 5>;
}

impl<P1, P2> DivScale<Scale<P2, 0>> for Scale<P1, 4>
where
    P1: Prefix,
    P2: Prefix,
{
    type Output = Scale<P1, 4>;
}

impl<P> DivScale<Scale<P, 1>> for Scale<P, 4>
where
    P: Prefix,
{
    type Output = Scale<P, 3>;
}

impl<P> DivScale<Scale<P, 2>> for Scale<P, 4>
where
    P: Prefix,
{
    type Output = Scale<P, 2>;
}

impl<P> DivScale<Scale<P, 3>> for Scale<P, 4>
where
    P: Prefix,
{
    type Output = Scale<P, 1>;
}

impl<P> DivScale<Scale<P, 4>> for Scale<P, 4>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> DivScale<Scale<P, 5>> for Scale<P, 4>
where
    P: Prefix,
{
    type Output = Scale<P, -1>;
}

impl<P> DivScale<Scale<P, 6>> for Scale<P, 4>
where
    P: Prefix,
{
    type Output = Scale<P, -2>;
}

impl<P> DivScale<Scale<P, 7>> for Scale<P, 4>
where
    P: Prefix,
{
    type Output = Scale<P, -3>;
}

impl<P> DivScale<Scale<P, 8>> for Scale<P, 4>
where
    P: Prefix,
{
    type Output = Scale<P, -4>;
}

impl<P> DivScale<Scale<P, 9>> for Scale<P, 4>
where
    P: Prefix,
{
    type Output = Scale<P, -5>;
}

impl<P> DivScale<Scale<P, 10>> for Scale<P, 4>
where
    P: Prefix,
{
    type Output = Scale<P, -6>;
}

//------------------------- 5 -------------------------

impl<P> DivScale<Scale<P, -5>> for Scale<P, 5>
where
    P: Prefix,
{
    type Output = Scale<P, 10>;
}

impl<P> DivScale<Scale<P, -4>> for Scale<P, 5>
where
    P: Prefix,
{
    type Output = Scale<P, 9>;
}

impl<P> DivScale<Scale<P, -3>> for Scale<P, 5>
where
    P: Prefix,
{
    type Output = Scale<P, 8>;
}

impl<P> DivScale<Scale<P, -2>> for Scale<P, 5>
where
    P: Prefix,
{
    type Output = Scale<P, 7>;
}

impl<P> DivScale<Scale<P, -1>> for Scale<P, 5>
where
    P: Prefix,
{
    type Output = Scale<P, 6>;
}

impl<P1, P2> DivScale<Scale<P2, 0>> for Scale<P1, 5>
where
    P1: Prefix,
    P2: Prefix,
{
    type Output = Scale<P1, 5>;
}

impl<P> DivScale<Scale<P, 1>> for Scale<P, 5>
where
    P: Prefix,
{
    type Output = Scale<P, 4>;
}

impl<P> DivScale<Scale<P, 2>> for Scale<P, 5>
where
    P: Prefix,
{
    type Output = Scale<P, 3>;
}

impl<P> DivScale<Scale<P, 3>> for Scale<P, 5>
where
    P: Prefix,
{
    type Output = Scale<P, 2>;
}

impl<P> DivScale<Scale<P, 4>> for Scale<P, 5>
where
    P: Prefix,
{
    type Output = Scale<P, 1>;
}

impl<P> DivScale<Scale<P, 5>> for Scale<P, 5>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> DivScale<Scale<P, 6>> for Scale<P, 5>
where
    P: Prefix,
{
    type Output = Scale<P, -1>;
}

impl<P> DivScale<Scale<P, 7>> for Scale<P, 5>
where
    P: Prefix,
{
    type Output = Scale<P, -2>;
}

impl<P> DivScale<Scale<P, 8>> for Scale<P, 5>
where
    P: Prefix,
{
    type Output = Scale<P, -3>;
}

impl<P> DivScale<Scale<P, 9>> for Scale<P, 5>
where
    P: Prefix,
{
    type Output = Scale<P, -4>;
}

impl<P> DivScale<Scale<P, 10>> for Scale<P, 5>
where
    P: Prefix,
{
    type Output = Scale<P, -5>;
}

//------------------------- 6 -------------------------

impl<P> DivScale<Scale<P, -4>> for Scale<P, 6>
where
    P: Prefix,
{
    type Output = Scale<P, 10>;
}

impl<P> DivScale<Scale<P, -3>> for Scale<P, 6>
where
    P: Prefix,
{
    type Output = Scale<P, 9>;
}

impl<P> DivScale<Scale<P, -2>> for Scale<P, 6>
where
    P: Prefix,
{
    type Output = Scale<P, 8>;
}

impl<P> DivScale<Scale<P, -1>> for Scale<P, 6>
where
    P: Prefix,
{
    type Output = Scale<P, 7>;
}

impl<P1, P2> DivScale<Scale<P2, 0>> for Scale<P1, 6>
where
    P1: Prefix,
    P2: Prefix,
{
    type Output = Scale<P1, 6>;
}

impl<P> DivScale<Scale<P, 1>> for Scale<P, 6>
where
    P: Prefix,
{
    type Output = Scale<P, 5>;
}

impl<P> DivScale<Scale<P, 2>> for Scale<P, 6>
where
    P: Prefix,
{
    type Output = Scale<P, 4>;
}

impl<P> DivScale<Scale<P, 3>> for Scale<P, 6>
where
    P: Prefix,
{
    type Output = Scale<P, 3>;
}

impl<P> DivScale<Scale<P, 4>> for Scale<P, 6>
where
    P: Prefix,
{
    type Output = Scale<P, 2>;
}

impl<P> DivScale<Scale<P, 5>> for Scale<P, 6>
where
    P: Prefix,
{
    type Output = Scale<P, 1>;
}

impl<P> DivScale<Scale<P, 6>> for Scale<P, 6>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> DivScale<Scale<P, 7>> for Scale<P, 6>
where
    P: Prefix,
{
    type Output = Scale<P, -1>;
}

impl<P> DivScale<Scale<P, 8>> for Scale<P, 6>
where
    P: Prefix,
{
    type Output = Scale<P, -2>;
}

impl<P> DivScale<Scale<P, 9>> for Scale<P, 6>
where
    P: Prefix,
{
    type Output = Scale<P, -3>;
}

impl<P> DivScale<Scale<P, 10>> for Scale<P, 6>
where
    P: Prefix,
{
    type Output = Scale<P, -4>;
}

//------------------------- 7 -------------------------

impl<P> DivScale<Scale<P, -3>> for Scale<P, 7>
where
    P: Prefix,
{
    type Output = Scale<P, 10>;
}

impl<P> DivScale<Scale<P, -2>> for Scale<P, 7>
where
    P: Prefix,
{
    type Output = Scale<P, 9>;
}

impl<P> DivScale<Scale<P, -1>> for Scale<P, 7>
where
    P: Prefix,
{
    type Output = Scale<P, 8>;
}

impl<P1, P2> DivScale<Scale<P2, 0>> for Scale<P1, 7>
where
    P1: Prefix,
    P2: Prefix,
{
    type Output = Scale<P1, 7>;
}

impl<P> DivScale<Scale<P, 1>> for Scale<P, 7>
where
    P: Prefix,
{
    type Output = Scale<P, 6>;
}

impl<P> DivScale<Scale<P, 2>> for Scale<P, 7>
where
    P: Prefix,
{
    type Output = Scale<P, 5>;
}

impl<P> DivScale<Scale<P, 3>> for Scale<P, 7>
where
    P: Prefix,
{
    type Output = Scale<P, 4>;
}

impl<P> DivScale<Scale<P, 4>> for Scale<P, 7>
where
    P: Prefix,
{
    type Output = Scale<P, 3>;
}

impl<P> DivScale<Scale<P, 5>> for Scale<P, 7>
where
    P: Prefix,
{
    type Output = Scale<P, 2>;
}

impl<P> DivScale<Scale<P, 6>> for Scale<P, 7>
where
    P: Prefix,
{
    type Output = Scale<P, 1>;
}

impl<P> DivScale<Scale<P, 7>> for Scale<P, 7>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> DivScale<Scale<P, 8>> for Scale<P, 7>
where
    P: Prefix,
{
    type Output = Scale<P, -1>;
}

impl<P> DivScale<Scale<P, 9>> for Scale<P, 7>
where
    P: Prefix,
{
    type Output = Scale<P, -2>;
}

impl<P> DivScale<Scale<P, 10>> for Scale<P, 7>
where
    P: Prefix,
{
    type Output = Scale<P, -3>;
}

//------------------------- 8 -------------------------

impl<P> DivScale<Scale<P, -2>> for Scale<P, 8>
where
    P: Prefix,
{
    type Output = Scale<P, 10>;
}

impl<P> DivScale<Scale<P, -1>> for Scale<P, 8>
where
    P: Prefix,
{
    type Output = Scale<P, 9>;
}

impl<P1, P2> DivScale<Scale<P2, 0>> for Scale<P1, 8>
where
    P1: Prefix,
    P2: Prefix,
{
    type Output = Scale<P1, 8>;
}

impl<P> DivScale<Scale<P, 1>> for Scale<P, 8>
where
    P: Prefix,
{
    type Output = Scale<P, 7>;
}

impl<P> DivScale<Scale<P, 2>> for Scale<P, 8>
where
    P: Prefix,
{
    type Output = Scale<P, 6>;
}

impl<P> DivScale<Scale<P, 3>> for Scale<P, 8>
where
    P: Prefix,
{
    type Output = Scale<P, 5>;
}

impl<P> DivScale<Scale<P, 4>> for Scale<P, 8>
where
    P: Prefix,
{
    type Output = Scale<P, 4>;
}

impl<P> DivScale<Scale<P, 5>> for Scale<P, 8>
where
    P: Prefix,
{
    type Output = Scale<P, 3>;
}

impl<P> DivScale<Scale<P, 6>> for Scale<P, 8>
where
    P: Prefix,
{
    type Output = Scale<P, 2>;
}

impl<P> DivScale<Scale<P, 7>> for Scale<P, 8>
where
    P: Prefix,
{
    type Output = Scale<P, 1>;
}

impl<P> DivScale<Scale<P, 8>> for Scale<P, 8>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> DivScale<Scale<P, 9>> for Scale<P, 8>
where
    P: Prefix,
{
    type Output = Scale<P, -1>;
}

impl<P> DivScale<Scale<P, 10>> for Scale<P, 8>
where
    P: Prefix,
{
    type Output = Scale<P, -2>;
}

//------------------------- 9 -------------------------

impl<P> DivScale<Scale<P, -1>> for Scale<P, 9>
where
    P: Prefix,
{
    type Output = Scale<P, 10>;
}

impl<P1, P2> DivScale<Scale<P2, 0>> for Scale<P1, 9>
where
    P1: Prefix,
    P2: Prefix,
{
    type Output = Scale<P1, 9>;
}

impl<P> DivScale<Scale<P, 1>> for Scale<P, 9>
where
    P: Prefix,
{
    type Output = Scale<P, 8>;
}

impl<P> DivScale<Scale<P, 2>> for Scale<P, 9>
where
    P: Prefix,
{
    type Output = Scale<P, 7>;
}

impl<P> DivScale<Scale<P, 3>> for Scale<P, 9>
where
    P: Prefix,
{
    type Output = Scale<P, 6>;
}

impl<P> DivScale<Scale<P, 4>> for Scale<P, 9>
where
    P: Prefix,
{
    type Output = Scale<P, 5>;
}

impl<P> DivScale<Scale<P, 5>> for Scale<P, 9>
where
    P: Prefix,
{
    type Output = Scale<P, 4>;
}

impl<P> DivScale<Scale<P, 6>> for Scale<P, 9>
where
    P: Prefix,
{
    type Output = Scale<P, 3>;
}

impl<P> DivScale<Scale<P, 7>> for Scale<P, 9>
where
    P: Prefix,
{
    type Output = Scale<P, 2>;
}

impl<P> DivScale<Scale<P, 8>> for Scale<P, 9>
where
    P: Prefix,
{
    type Output = Scale<P, 1>;
}

impl<P> DivScale<Scale<P, 9>> for Scale<P, 9>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> DivScale<Scale<P, 10>> for Scale<P, 9>
where
    P: Prefix,
{
    type Output = Scale<P, -1>;
}

//------------------------- 10 -------------------------

impl<P1, P2> DivScale<Scale<P2, 0>> for Scale<P1, 10>
where
    P1: Prefix,
    P2: Prefix,
{
    type Output = Scale<P1, 10>;
}

impl<P> DivScale<Scale<P, 1>> for Scale<P, 10>
where
    P: Prefix,
{
    type Output = Scale<P, 9>;
}

impl<P> DivScale<Scale<P, 2>> for Scale<P, 10>
where
    P: Prefix,
{
    type Output = Scale<P, 8>;
}

impl<P> DivScale<Scale<P, 3>> for Scale<P, 10>
where
    P: Prefix,
{
    type Output = Scale<P, 7>;
}

impl<P> DivScale<Scale<P, 4>> for Scale<P, 10>
where
    P: Prefix,
{
    type Output = Scale<P, 6>;
}

impl<P> DivScale<Scale<P, 5>> for Scale<P, 10>
where
    P: Prefix,
{
    type Output = Scale<P, 5>;
}

impl<P> DivScale<Scale<P, 6>> for Scale<P, 10>
where
    P: Prefix,
{
    type Output = Scale<P, 4>;
}

impl<P> DivScale<Scale<P, 7>> for Scale<P, 10>
where
    P: Prefix,
{
    type Output = Scale<P, 3>;
}

impl<P> DivScale<Scale<P, 8>> for Scale<P, 10>
where
    P: Prefix,
{
    type Output = Scale<P, 2>;
}

impl<P> DivScale<Scale<P, 9>> for Scale<P, 10>
where
    P: Prefix,
{
    type Output = Scale<P, 1>;
}

impl<P> DivScale<Scale<P, 10>> for Scale<P, 10>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}
