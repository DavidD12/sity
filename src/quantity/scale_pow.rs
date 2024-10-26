use super::*;


//------------------------- -10 -------------------------

impl<P> PowScale<-1> for Scale<P, -10>
where
    P: Prefix,
{
    type Output = Scale<P, 10>;
}

impl<P> PowScale<0> for Scale<P, -10>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> PowScale<1> for Scale<P, -10>
where
    P: Prefix,
{
    type Output = Scale<P, -10>;
}

//------------------------- -9 -------------------------

impl<P> PowScale<-1> for Scale<P, -9>
where
    P: Prefix,
{
    type Output = Scale<P, 9>;
}

impl<P> PowScale<0> for Scale<P, -9>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> PowScale<1> for Scale<P, -9>
where
    P: Prefix,
{
    type Output = Scale<P, -9>;
}

//------------------------- -8 -------------------------

impl<P> PowScale<-1> for Scale<P, -8>
where
    P: Prefix,
{
    type Output = Scale<P, 8>;
}

impl<P> PowScale<0> for Scale<P, -8>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> PowScale<1> for Scale<P, -8>
where
    P: Prefix,
{
    type Output = Scale<P, -8>;
}

//------------------------- -7 -------------------------

impl<P> PowScale<-1> for Scale<P, -7>
where
    P: Prefix,
{
    type Output = Scale<P, 7>;
}

impl<P> PowScale<0> for Scale<P, -7>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> PowScale<1> for Scale<P, -7>
where
    P: Prefix,
{
    type Output = Scale<P, -7>;
}

//------------------------- -6 -------------------------

impl<P> PowScale<-1> for Scale<P, -6>
where
    P: Prefix,
{
    type Output = Scale<P, 6>;
}

impl<P> PowScale<0> for Scale<P, -6>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> PowScale<1> for Scale<P, -6>
where
    P: Prefix,
{
    type Output = Scale<P, -6>;
}

//------------------------- -5 -------------------------

impl<P> PowScale<-2> for Scale<P, -5>
where
    P: Prefix,
{
    type Output = Scale<P, 10>;
}

impl<P> PowScale<-1> for Scale<P, -5>
where
    P: Prefix,
{
    type Output = Scale<P, 5>;
}

impl<P> PowScale<0> for Scale<P, -5>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> PowScale<1> for Scale<P, -5>
where
    P: Prefix,
{
    type Output = Scale<P, -5>;
}

impl<P> PowScale<2> for Scale<P, -5>
where
    P: Prefix,
{
    type Output = Scale<P, -10>;
}

//------------------------- -4 -------------------------

impl<P> PowScale<-2> for Scale<P, -4>
where
    P: Prefix,
{
    type Output = Scale<P, 8>;
}

impl<P> PowScale<-1> for Scale<P, -4>
where
    P: Prefix,
{
    type Output = Scale<P, 4>;
}

impl<P> PowScale<0> for Scale<P, -4>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> PowScale<1> for Scale<P, -4>
where
    P: Prefix,
{
    type Output = Scale<P, -4>;
}

impl<P> PowScale<2> for Scale<P, -4>
where
    P: Prefix,
{
    type Output = Scale<P, -8>;
}

//------------------------- -3 -------------------------

impl<P> PowScale<-3> for Scale<P, -3>
where
    P: Prefix,
{
    type Output = Scale<P, 9>;
}

impl<P> PowScale<-2> for Scale<P, -3>
where
    P: Prefix,
{
    type Output = Scale<P, 6>;
}

impl<P> PowScale<-1> for Scale<P, -3>
where
    P: Prefix,
{
    type Output = Scale<P, 3>;
}

impl<P> PowScale<0> for Scale<P, -3>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> PowScale<1> for Scale<P, -3>
where
    P: Prefix,
{
    type Output = Scale<P, -3>;
}

impl<P> PowScale<2> for Scale<P, -3>
where
    P: Prefix,
{
    type Output = Scale<P, -6>;
}

impl<P> PowScale<3> for Scale<P, -3>
where
    P: Prefix,
{
    type Output = Scale<P, -9>;
}

//------------------------- -2 -------------------------

impl<P> PowScale<-5> for Scale<P, -2>
where
    P: Prefix,
{
    type Output = Scale<P, 10>;
}

impl<P> PowScale<-4> for Scale<P, -2>
where
    P: Prefix,
{
    type Output = Scale<P, 8>;
}

impl<P> PowScale<-3> for Scale<P, -2>
where
    P: Prefix,
{
    type Output = Scale<P, 6>;
}

impl<P> PowScale<-2> for Scale<P, -2>
where
    P: Prefix,
{
    type Output = Scale<P, 4>;
}

impl<P> PowScale<-1> for Scale<P, -2>
where
    P: Prefix,
{
    type Output = Scale<P, 2>;
}

impl<P> PowScale<0> for Scale<P, -2>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> PowScale<1> for Scale<P, -2>
where
    P: Prefix,
{
    type Output = Scale<P, -2>;
}

impl<P> PowScale<2> for Scale<P, -2>
where
    P: Prefix,
{
    type Output = Scale<P, -4>;
}

impl<P> PowScale<3> for Scale<P, -2>
where
    P: Prefix,
{
    type Output = Scale<P, -6>;
}

impl<P> PowScale<4> for Scale<P, -2>
where
    P: Prefix,
{
    type Output = Scale<P, -8>;
}

impl<P> PowScale<5> for Scale<P, -2>
where
    P: Prefix,
{
    type Output = Scale<P, -10>;
}

//------------------------- -1 -------------------------

impl<P> PowScale<-10> for Scale<P, -1>
where
    P: Prefix,
{
    type Output = Scale<P, 10>;
}

impl<P> PowScale<-9> for Scale<P, -1>
where
    P: Prefix,
{
    type Output = Scale<P, 9>;
}

impl<P> PowScale<-8> for Scale<P, -1>
where
    P: Prefix,
{
    type Output = Scale<P, 8>;
}

impl<P> PowScale<-7> for Scale<P, -1>
where
    P: Prefix,
{
    type Output = Scale<P, 7>;
}

impl<P> PowScale<-6> for Scale<P, -1>
where
    P: Prefix,
{
    type Output = Scale<P, 6>;
}

impl<P> PowScale<-5> for Scale<P, -1>
where
    P: Prefix,
{
    type Output = Scale<P, 5>;
}

impl<P> PowScale<-4> for Scale<P, -1>
where
    P: Prefix,
{
    type Output = Scale<P, 4>;
}

impl<P> PowScale<-3> for Scale<P, -1>
where
    P: Prefix,
{
    type Output = Scale<P, 3>;
}

impl<P> PowScale<-2> for Scale<P, -1>
where
    P: Prefix,
{
    type Output = Scale<P, 2>;
}

impl<P> PowScale<-1> for Scale<P, -1>
where
    P: Prefix,
{
    type Output = Scale<P, 1>;
}

impl<P> PowScale<0> for Scale<P, -1>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> PowScale<1> for Scale<P, -1>
where
    P: Prefix,
{
    type Output = Scale<P, -1>;
}

impl<P> PowScale<2> for Scale<P, -1>
where
    P: Prefix,
{
    type Output = Scale<P, -2>;
}

impl<P> PowScale<3> for Scale<P, -1>
where
    P: Prefix,
{
    type Output = Scale<P, -3>;
}

impl<P> PowScale<4> for Scale<P, -1>
where
    P: Prefix,
{
    type Output = Scale<P, -4>;
}

impl<P> PowScale<5> for Scale<P, -1>
where
    P: Prefix,
{
    type Output = Scale<P, -5>;
}

impl<P> PowScale<6> for Scale<P, -1>
where
    P: Prefix,
{
    type Output = Scale<P, -6>;
}

impl<P> PowScale<7> for Scale<P, -1>
where
    P: Prefix,
{
    type Output = Scale<P, -7>;
}

impl<P> PowScale<8> for Scale<P, -1>
where
    P: Prefix,
{
    type Output = Scale<P, -8>;
}

impl<P> PowScale<9> for Scale<P, -1>
where
    P: Prefix,
{
    type Output = Scale<P, -9>;
}

impl<P> PowScale<10> for Scale<P, -1>
where
    P: Prefix,
{
    type Output = Scale<P, -10>;
}

//------------------------- 0 -------------------------

impl<P> PowScale<-10> for Scale<P, 0>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> PowScale<-9> for Scale<P, 0>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> PowScale<-8> for Scale<P, 0>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> PowScale<-7> for Scale<P, 0>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> PowScale<-6> for Scale<P, 0>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> PowScale<-5> for Scale<P, 0>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> PowScale<-4> for Scale<P, 0>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> PowScale<-3> for Scale<P, 0>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> PowScale<-2> for Scale<P, 0>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> PowScale<-1> for Scale<P, 0>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> PowScale<0> for Scale<P, 0>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> PowScale<1> for Scale<P, 0>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> PowScale<2> for Scale<P, 0>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> PowScale<3> for Scale<P, 0>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> PowScale<4> for Scale<P, 0>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> PowScale<5> for Scale<P, 0>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> PowScale<6> for Scale<P, 0>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> PowScale<7> for Scale<P, 0>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> PowScale<8> for Scale<P, 0>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> PowScale<9> for Scale<P, 0>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> PowScale<10> for Scale<P, 0>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

//------------------------- 1 -------------------------

impl<P> PowScale<-10> for Scale<P, 1>
where
    P: Prefix,
{
    type Output = Scale<P, -10>;
}

impl<P> PowScale<-9> for Scale<P, 1>
where
    P: Prefix,
{
    type Output = Scale<P, -9>;
}

impl<P> PowScale<-8> for Scale<P, 1>
where
    P: Prefix,
{
    type Output = Scale<P, -8>;
}

impl<P> PowScale<-7> for Scale<P, 1>
where
    P: Prefix,
{
    type Output = Scale<P, -7>;
}

impl<P> PowScale<-6> for Scale<P, 1>
where
    P: Prefix,
{
    type Output = Scale<P, -6>;
}

impl<P> PowScale<-5> for Scale<P, 1>
where
    P: Prefix,
{
    type Output = Scale<P, -5>;
}

impl<P> PowScale<-4> for Scale<P, 1>
where
    P: Prefix,
{
    type Output = Scale<P, -4>;
}

impl<P> PowScale<-3> for Scale<P, 1>
where
    P: Prefix,
{
    type Output = Scale<P, -3>;
}

impl<P> PowScale<-2> for Scale<P, 1>
where
    P: Prefix,
{
    type Output = Scale<P, -2>;
}

impl<P> PowScale<-1> for Scale<P, 1>
where
    P: Prefix,
{
    type Output = Scale<P, -1>;
}

impl<P> PowScale<0> for Scale<P, 1>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> PowScale<1> for Scale<P, 1>
where
    P: Prefix,
{
    type Output = Scale<P, 1>;
}

impl<P> PowScale<2> for Scale<P, 1>
where
    P: Prefix,
{
    type Output = Scale<P, 2>;
}

impl<P> PowScale<3> for Scale<P, 1>
where
    P: Prefix,
{
    type Output = Scale<P, 3>;
}

impl<P> PowScale<4> for Scale<P, 1>
where
    P: Prefix,
{
    type Output = Scale<P, 4>;
}

impl<P> PowScale<5> for Scale<P, 1>
where
    P: Prefix,
{
    type Output = Scale<P, 5>;
}

impl<P> PowScale<6> for Scale<P, 1>
where
    P: Prefix,
{
    type Output = Scale<P, 6>;
}

impl<P> PowScale<7> for Scale<P, 1>
where
    P: Prefix,
{
    type Output = Scale<P, 7>;
}

impl<P> PowScale<8> for Scale<P, 1>
where
    P: Prefix,
{
    type Output = Scale<P, 8>;
}

impl<P> PowScale<9> for Scale<P, 1>
where
    P: Prefix,
{
    type Output = Scale<P, 9>;
}

impl<P> PowScale<10> for Scale<P, 1>
where
    P: Prefix,
{
    type Output = Scale<P, 10>;
}

//------------------------- 2 -------------------------

impl<P> PowScale<-5> for Scale<P, 2>
where
    P: Prefix,
{
    type Output = Scale<P, -10>;
}

impl<P> PowScale<-4> for Scale<P, 2>
where
    P: Prefix,
{
    type Output = Scale<P, -8>;
}

impl<P> PowScale<-3> for Scale<P, 2>
where
    P: Prefix,
{
    type Output = Scale<P, -6>;
}

impl<P> PowScale<-2> for Scale<P, 2>
where
    P: Prefix,
{
    type Output = Scale<P, -4>;
}

impl<P> PowScale<-1> for Scale<P, 2>
where
    P: Prefix,
{
    type Output = Scale<P, -2>;
}

impl<P> PowScale<0> for Scale<P, 2>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> PowScale<1> for Scale<P, 2>
where
    P: Prefix,
{
    type Output = Scale<P, 2>;
}

impl<P> PowScale<2> for Scale<P, 2>
where
    P: Prefix,
{
    type Output = Scale<P, 4>;
}

impl<P> PowScale<3> for Scale<P, 2>
where
    P: Prefix,
{
    type Output = Scale<P, 6>;
}

impl<P> PowScale<4> for Scale<P, 2>
where
    P: Prefix,
{
    type Output = Scale<P, 8>;
}

impl<P> PowScale<5> for Scale<P, 2>
where
    P: Prefix,
{
    type Output = Scale<P, 10>;
}

//------------------------- 3 -------------------------

impl<P> PowScale<-3> for Scale<P, 3>
where
    P: Prefix,
{
    type Output = Scale<P, -9>;
}

impl<P> PowScale<-2> for Scale<P, 3>
where
    P: Prefix,
{
    type Output = Scale<P, -6>;
}

impl<P> PowScale<-1> for Scale<P, 3>
where
    P: Prefix,
{
    type Output = Scale<P, -3>;
}

impl<P> PowScale<0> for Scale<P, 3>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> PowScale<1> for Scale<P, 3>
where
    P: Prefix,
{
    type Output = Scale<P, 3>;
}

impl<P> PowScale<2> for Scale<P, 3>
where
    P: Prefix,
{
    type Output = Scale<P, 6>;
}

impl<P> PowScale<3> for Scale<P, 3>
where
    P: Prefix,
{
    type Output = Scale<P, 9>;
}

//------------------------- 4 -------------------------

impl<P> PowScale<-2> for Scale<P, 4>
where
    P: Prefix,
{
    type Output = Scale<P, -8>;
}

impl<P> PowScale<-1> for Scale<P, 4>
where
    P: Prefix,
{
    type Output = Scale<P, -4>;
}

impl<P> PowScale<0> for Scale<P, 4>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> PowScale<1> for Scale<P, 4>
where
    P: Prefix,
{
    type Output = Scale<P, 4>;
}

impl<P> PowScale<2> for Scale<P, 4>
where
    P: Prefix,
{
    type Output = Scale<P, 8>;
}

//------------------------- 5 -------------------------

impl<P> PowScale<-2> for Scale<P, 5>
where
    P: Prefix,
{
    type Output = Scale<P, -10>;
}

impl<P> PowScale<-1> for Scale<P, 5>
where
    P: Prefix,
{
    type Output = Scale<P, -5>;
}

impl<P> PowScale<0> for Scale<P, 5>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> PowScale<1> for Scale<P, 5>
where
    P: Prefix,
{
    type Output = Scale<P, 5>;
}

impl<P> PowScale<2> for Scale<P, 5>
where
    P: Prefix,
{
    type Output = Scale<P, 10>;
}

//------------------------- 6 -------------------------

impl<P> PowScale<-1> for Scale<P, 6>
where
    P: Prefix,
{
    type Output = Scale<P, -6>;
}

impl<P> PowScale<0> for Scale<P, 6>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> PowScale<1> for Scale<P, 6>
where
    P: Prefix,
{
    type Output = Scale<P, 6>;
}

//------------------------- 7 -------------------------

impl<P> PowScale<-1> for Scale<P, 7>
where
    P: Prefix,
{
    type Output = Scale<P, -7>;
}

impl<P> PowScale<0> for Scale<P, 7>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> PowScale<1> for Scale<P, 7>
where
    P: Prefix,
{
    type Output = Scale<P, 7>;
}

//------------------------- 8 -------------------------

impl<P> PowScale<-1> for Scale<P, 8>
where
    P: Prefix,
{
    type Output = Scale<P, -8>;
}

impl<P> PowScale<0> for Scale<P, 8>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> PowScale<1> for Scale<P, 8>
where
    P: Prefix,
{
    type Output = Scale<P, 8>;
}

//------------------------- 9 -------------------------

impl<P> PowScale<-1> for Scale<P, 9>
where
    P: Prefix,
{
    type Output = Scale<P, -9>;
}

impl<P> PowScale<0> for Scale<P, 9>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> PowScale<1> for Scale<P, 9>
where
    P: Prefix,
{
    type Output = Scale<P, 9>;
}

//------------------------- 10 -------------------------

impl<P> PowScale<-1> for Scale<P, 10>
where
    P: Prefix,
{
    type Output = Scale<P, -10>;
}

impl<P> PowScale<0> for Scale<P, 10>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> PowScale<1> for Scale<P, 10>
where
    P: Prefix,
{
    type Output = Scale<P, 10>;
}
