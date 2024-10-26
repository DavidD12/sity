use super::*;


//------------------------- -10 -------------------------

impl<P> RootScale<-10> for Scale<P, -10>
where
    P: Prefix,
{
    type Output = Scale<P, 1>;
}

impl<P> RootScale<-5> for Scale<P, -10>
where
    P: Prefix,
{
    type Output = Scale<P, 2>;
}

impl<P> RootScale<-2> for Scale<P, -10>
where
    P: Prefix,
{
    type Output = Scale<P, 5>;
}

impl<P> RootScale<-1> for Scale<P, -10>
where
    P: Prefix,
{
    type Output = Scale<P, 10>;
}

impl<P> RootScale<1> for Scale<P, -10>
where
    P: Prefix,
{
    type Output = Scale<P, -10>;
}

impl<P> RootScale<2> for Scale<P, -10>
where
    P: Prefix,
{
    type Output = Scale<P, -5>;
}

impl<P> RootScale<5> for Scale<P, -10>
where
    P: Prefix,
{
    type Output = Scale<P, -2>;
}

impl<P> RootScale<10> for Scale<P, -10>
where
    P: Prefix,
{
    type Output = Scale<P, -1>;
}

//------------------------- -9 -------------------------

impl<P> RootScale<-9> for Scale<P, -9>
where
    P: Prefix,
{
    type Output = Scale<P, 1>;
}

impl<P> RootScale<-3> for Scale<P, -9>
where
    P: Prefix,
{
    type Output = Scale<P, 3>;
}

impl<P> RootScale<-1> for Scale<P, -9>
where
    P: Prefix,
{
    type Output = Scale<P, 9>;
}

impl<P> RootScale<1> for Scale<P, -9>
where
    P: Prefix,
{
    type Output = Scale<P, -9>;
}

impl<P> RootScale<3> for Scale<P, -9>
where
    P: Prefix,
{
    type Output = Scale<P, -3>;
}

impl<P> RootScale<9> for Scale<P, -9>
where
    P: Prefix,
{
    type Output = Scale<P, -1>;
}

//------------------------- -8 -------------------------

impl<P> RootScale<-8> for Scale<P, -8>
where
    P: Prefix,
{
    type Output = Scale<P, 1>;
}

impl<P> RootScale<-4> for Scale<P, -8>
where
    P: Prefix,
{
    type Output = Scale<P, 2>;
}

impl<P> RootScale<-2> for Scale<P, -8>
where
    P: Prefix,
{
    type Output = Scale<P, 4>;
}

impl<P> RootScale<-1> for Scale<P, -8>
where
    P: Prefix,
{
    type Output = Scale<P, 8>;
}

impl<P> RootScale<1> for Scale<P, -8>
where
    P: Prefix,
{
    type Output = Scale<P, -8>;
}

impl<P> RootScale<2> for Scale<P, -8>
where
    P: Prefix,
{
    type Output = Scale<P, -4>;
}

impl<P> RootScale<4> for Scale<P, -8>
where
    P: Prefix,
{
    type Output = Scale<P, -2>;
}

impl<P> RootScale<8> for Scale<P, -8>
where
    P: Prefix,
{
    type Output = Scale<P, -1>;
}

//------------------------- -7 -------------------------

impl<P> RootScale<-7> for Scale<P, -7>
where
    P: Prefix,
{
    type Output = Scale<P, 1>;
}

impl<P> RootScale<-1> for Scale<P, -7>
where
    P: Prefix,
{
    type Output = Scale<P, 7>;
}

impl<P> RootScale<1> for Scale<P, -7>
where
    P: Prefix,
{
    type Output = Scale<P, -7>;
}

impl<P> RootScale<7> for Scale<P, -7>
where
    P: Prefix,
{
    type Output = Scale<P, -1>;
}

//------------------------- -6 -------------------------

impl<P> RootScale<-6> for Scale<P, -6>
where
    P: Prefix,
{
    type Output = Scale<P, 1>;
}

impl<P> RootScale<-3> for Scale<P, -6>
where
    P: Prefix,
{
    type Output = Scale<P, 2>;
}

impl<P> RootScale<-2> for Scale<P, -6>
where
    P: Prefix,
{
    type Output = Scale<P, 3>;
}

impl<P> RootScale<-1> for Scale<P, -6>
where
    P: Prefix,
{
    type Output = Scale<P, 6>;
}

impl<P> RootScale<1> for Scale<P, -6>
where
    P: Prefix,
{
    type Output = Scale<P, -6>;
}

impl<P> RootScale<2> for Scale<P, -6>
where
    P: Prefix,
{
    type Output = Scale<P, -3>;
}

impl<P> RootScale<3> for Scale<P, -6>
where
    P: Prefix,
{
    type Output = Scale<P, -2>;
}

impl<P> RootScale<6> for Scale<P, -6>
where
    P: Prefix,
{
    type Output = Scale<P, -1>;
}

//------------------------- -5 -------------------------

impl<P> RootScale<-5> for Scale<P, -5>
where
    P: Prefix,
{
    type Output = Scale<P, 1>;
}

impl<P> RootScale<-1> for Scale<P, -5>
where
    P: Prefix,
{
    type Output = Scale<P, 5>;
}

impl<P> RootScale<1> for Scale<P, -5>
where
    P: Prefix,
{
    type Output = Scale<P, -5>;
}

impl<P> RootScale<5> for Scale<P, -5>
where
    P: Prefix,
{
    type Output = Scale<P, -1>;
}

//------------------------- -4 -------------------------

impl<P> RootScale<-4> for Scale<P, -4>
where
    P: Prefix,
{
    type Output = Scale<P, 1>;
}

impl<P> RootScale<-2> for Scale<P, -4>
where
    P: Prefix,
{
    type Output = Scale<P, 2>;
}

impl<P> RootScale<-1> for Scale<P, -4>
where
    P: Prefix,
{
    type Output = Scale<P, 4>;
}

impl<P> RootScale<1> for Scale<P, -4>
where
    P: Prefix,
{
    type Output = Scale<P, -4>;
}

impl<P> RootScale<2> for Scale<P, -4>
where
    P: Prefix,
{
    type Output = Scale<P, -2>;
}

impl<P> RootScale<4> for Scale<P, -4>
where
    P: Prefix,
{
    type Output = Scale<P, -1>;
}

//------------------------- -3 -------------------------

impl<P> RootScale<-3> for Scale<P, -3>
where
    P: Prefix,
{
    type Output = Scale<P, 1>;
}

impl<P> RootScale<-1> for Scale<P, -3>
where
    P: Prefix,
{
    type Output = Scale<P, 3>;
}

impl<P> RootScale<1> for Scale<P, -3>
where
    P: Prefix,
{
    type Output = Scale<P, -3>;
}

impl<P> RootScale<3> for Scale<P, -3>
where
    P: Prefix,
{
    type Output = Scale<P, -1>;
}

//------------------------- -2 -------------------------

impl<P> RootScale<-2> for Scale<P, -2>
where
    P: Prefix,
{
    type Output = Scale<P, 1>;
}

impl<P> RootScale<-1> for Scale<P, -2>
where
    P: Prefix,
{
    type Output = Scale<P, 2>;
}

impl<P> RootScale<1> for Scale<P, -2>
where
    P: Prefix,
{
    type Output = Scale<P, -2>;
}

impl<P> RootScale<2> for Scale<P, -2>
where
    P: Prefix,
{
    type Output = Scale<P, -1>;
}

//------------------------- -1 -------------------------

impl<P> RootScale<-1> for Scale<P, -1>
where
    P: Prefix,
{
    type Output = Scale<P, 1>;
}

impl<P> RootScale<1> for Scale<P, -1>
where
    P: Prefix,
{
    type Output = Scale<P, -1>;
}

//------------------------- 0 -------------------------

impl<P> RootScale<-10> for Scale<P, 0>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> RootScale<-9> for Scale<P, 0>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> RootScale<-8> for Scale<P, 0>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> RootScale<-7> for Scale<P, 0>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> RootScale<-6> for Scale<P, 0>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> RootScale<-5> for Scale<P, 0>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> RootScale<-4> for Scale<P, 0>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> RootScale<-3> for Scale<P, 0>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> RootScale<-2> for Scale<P, 0>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> RootScale<-1> for Scale<P, 0>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> RootScale<1> for Scale<P, 0>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> RootScale<2> for Scale<P, 0>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> RootScale<3> for Scale<P, 0>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> RootScale<4> for Scale<P, 0>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> RootScale<5> for Scale<P, 0>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> RootScale<6> for Scale<P, 0>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> RootScale<7> for Scale<P, 0>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> RootScale<8> for Scale<P, 0>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> RootScale<9> for Scale<P, 0>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

impl<P> RootScale<10> for Scale<P, 0>
where
    P: Prefix,
{
    type Output = Scale<One, 0>;
}

//------------------------- 1 -------------------------

impl<P> RootScale<-1> for Scale<P, 1>
where
    P: Prefix,
{
    type Output = Scale<P, -1>;
}

impl<P> RootScale<1> for Scale<P, 1>
where
    P: Prefix,
{
    type Output = Scale<P, 1>;
}

//------------------------- 2 -------------------------

impl<P> RootScale<-2> for Scale<P, 2>
where
    P: Prefix,
{
    type Output = Scale<P, -1>;
}

impl<P> RootScale<-1> for Scale<P, 2>
where
    P: Prefix,
{
    type Output = Scale<P, -2>;
}

impl<P> RootScale<1> for Scale<P, 2>
where
    P: Prefix,
{
    type Output = Scale<P, 2>;
}

impl<P> RootScale<2> for Scale<P, 2>
where
    P: Prefix,
{
    type Output = Scale<P, 1>;
}

//------------------------- 3 -------------------------

impl<P> RootScale<-3> for Scale<P, 3>
where
    P: Prefix,
{
    type Output = Scale<P, -1>;
}

impl<P> RootScale<-1> for Scale<P, 3>
where
    P: Prefix,
{
    type Output = Scale<P, -3>;
}

impl<P> RootScale<1> for Scale<P, 3>
where
    P: Prefix,
{
    type Output = Scale<P, 3>;
}

impl<P> RootScale<3> for Scale<P, 3>
where
    P: Prefix,
{
    type Output = Scale<P, 1>;
}

//------------------------- 4 -------------------------

impl<P> RootScale<-4> for Scale<P, 4>
where
    P: Prefix,
{
    type Output = Scale<P, -1>;
}

impl<P> RootScale<-2> for Scale<P, 4>
where
    P: Prefix,
{
    type Output = Scale<P, -2>;
}

impl<P> RootScale<-1> for Scale<P, 4>
where
    P: Prefix,
{
    type Output = Scale<P, -4>;
}

impl<P> RootScale<1> for Scale<P, 4>
where
    P: Prefix,
{
    type Output = Scale<P, 4>;
}

impl<P> RootScale<2> for Scale<P, 4>
where
    P: Prefix,
{
    type Output = Scale<P, 2>;
}

impl<P> RootScale<4> for Scale<P, 4>
where
    P: Prefix,
{
    type Output = Scale<P, 1>;
}

//------------------------- 5 -------------------------

impl<P> RootScale<-5> for Scale<P, 5>
where
    P: Prefix,
{
    type Output = Scale<P, -1>;
}

impl<P> RootScale<-1> for Scale<P, 5>
where
    P: Prefix,
{
    type Output = Scale<P, -5>;
}

impl<P> RootScale<1> for Scale<P, 5>
where
    P: Prefix,
{
    type Output = Scale<P, 5>;
}

impl<P> RootScale<5> for Scale<P, 5>
where
    P: Prefix,
{
    type Output = Scale<P, 1>;
}

//------------------------- 6 -------------------------

impl<P> RootScale<-6> for Scale<P, 6>
where
    P: Prefix,
{
    type Output = Scale<P, -1>;
}

impl<P> RootScale<-3> for Scale<P, 6>
where
    P: Prefix,
{
    type Output = Scale<P, -2>;
}

impl<P> RootScale<-2> for Scale<P, 6>
where
    P: Prefix,
{
    type Output = Scale<P, -3>;
}

impl<P> RootScale<-1> for Scale<P, 6>
where
    P: Prefix,
{
    type Output = Scale<P, -6>;
}

impl<P> RootScale<1> for Scale<P, 6>
where
    P: Prefix,
{
    type Output = Scale<P, 6>;
}

impl<P> RootScale<2> for Scale<P, 6>
where
    P: Prefix,
{
    type Output = Scale<P, 3>;
}

impl<P> RootScale<3> for Scale<P, 6>
where
    P: Prefix,
{
    type Output = Scale<P, 2>;
}

impl<P> RootScale<6> for Scale<P, 6>
where
    P: Prefix,
{
    type Output = Scale<P, 1>;
}

//------------------------- 7 -------------------------

impl<P> RootScale<-7> for Scale<P, 7>
where
    P: Prefix,
{
    type Output = Scale<P, -1>;
}

impl<P> RootScale<-1> for Scale<P, 7>
where
    P: Prefix,
{
    type Output = Scale<P, -7>;
}

impl<P> RootScale<1> for Scale<P, 7>
where
    P: Prefix,
{
    type Output = Scale<P, 7>;
}

impl<P> RootScale<7> for Scale<P, 7>
where
    P: Prefix,
{
    type Output = Scale<P, 1>;
}

//------------------------- 8 -------------------------

impl<P> RootScale<-8> for Scale<P, 8>
where
    P: Prefix,
{
    type Output = Scale<P, -1>;
}

impl<P> RootScale<-4> for Scale<P, 8>
where
    P: Prefix,
{
    type Output = Scale<P, -2>;
}

impl<P> RootScale<-2> for Scale<P, 8>
where
    P: Prefix,
{
    type Output = Scale<P, -4>;
}

impl<P> RootScale<-1> for Scale<P, 8>
where
    P: Prefix,
{
    type Output = Scale<P, -8>;
}

impl<P> RootScale<1> for Scale<P, 8>
where
    P: Prefix,
{
    type Output = Scale<P, 8>;
}

impl<P> RootScale<2> for Scale<P, 8>
where
    P: Prefix,
{
    type Output = Scale<P, 4>;
}

impl<P> RootScale<4> for Scale<P, 8>
where
    P: Prefix,
{
    type Output = Scale<P, 2>;
}

impl<P> RootScale<8> for Scale<P, 8>
where
    P: Prefix,
{
    type Output = Scale<P, 1>;
}

//------------------------- 9 -------------------------

impl<P> RootScale<-9> for Scale<P, 9>
where
    P: Prefix,
{
    type Output = Scale<P, -1>;
}

impl<P> RootScale<-3> for Scale<P, 9>
where
    P: Prefix,
{
    type Output = Scale<P, -3>;
}

impl<P> RootScale<-1> for Scale<P, 9>
where
    P: Prefix,
{
    type Output = Scale<P, -9>;
}

impl<P> RootScale<1> for Scale<P, 9>
where
    P: Prefix,
{
    type Output = Scale<P, 9>;
}

impl<P> RootScale<3> for Scale<P, 9>
where
    P: Prefix,
{
    type Output = Scale<P, 3>;
}

impl<P> RootScale<9> for Scale<P, 9>
where
    P: Prefix,
{
    type Output = Scale<P, 1>;
}

//------------------------- 10 -------------------------

impl<P> RootScale<-10> for Scale<P, 10>
where
    P: Prefix,
{
    type Output = Scale<P, -1>;
}

impl<P> RootScale<-5> for Scale<P, 10>
where
    P: Prefix,
{
    type Output = Scale<P, -2>;
}

impl<P> RootScale<-2> for Scale<P, 10>
where
    P: Prefix,
{
    type Output = Scale<P, -5>;
}

impl<P> RootScale<-1> for Scale<P, 10>
where
    P: Prefix,
{
    type Output = Scale<P, -10>;
}

impl<P> RootScale<1> for Scale<P, 10>
where
    P: Prefix,
{
    type Output = Scale<P, 10>;
}

impl<P> RootScale<2> for Scale<P, 10>
where
    P: Prefix,
{
    type Output = Scale<P, 5>;
}

impl<P> RootScale<5> for Scale<P, 10>
where
    P: Prefix,
{
    type Output = Scale<P, 2>;
}

impl<P> RootScale<10> for Scale<P, 10>
where
    P: Prefix,
{
    type Output = Scale<P, 1>;
}
