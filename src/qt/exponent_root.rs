use super::*;


//------------------------- -10 -------------------------

impl RootExp<-10> for Exp<-10> {
    type Output = Exp<1>;
}
            
impl RootExp<-5> for Exp<-10> {
    type Output = Exp<2>;
}
            
impl RootExp<-2> for Exp<-10> {
    type Output = Exp<5>;
}
            
impl RootExp<-1> for Exp<-10> {
    type Output = Exp<10>;
}
            
impl RootExp<1> for Exp<-10> {
    type Output = Exp<-10>;
}
            
impl RootExp<2> for Exp<-10> {
    type Output = Exp<-5>;
}
            
impl RootExp<5> for Exp<-10> {
    type Output = Exp<-2>;
}
            
impl RootExp<10> for Exp<-10> {
    type Output = Exp<-1>;
}
            
//------------------------- -9 -------------------------

impl RootExp<-9> for Exp<-9> {
    type Output = Exp<1>;
}
            
impl RootExp<-3> for Exp<-9> {
    type Output = Exp<3>;
}
            
impl RootExp<-1> for Exp<-9> {
    type Output = Exp<9>;
}
            
impl RootExp<1> for Exp<-9> {
    type Output = Exp<-9>;
}
            
impl RootExp<3> for Exp<-9> {
    type Output = Exp<-3>;
}
            
impl RootExp<9> for Exp<-9> {
    type Output = Exp<-1>;
}
            
//------------------------- -8 -------------------------

impl RootExp<-8> for Exp<-8> {
    type Output = Exp<1>;
}
            
impl RootExp<-4> for Exp<-8> {
    type Output = Exp<2>;
}
            
impl RootExp<-2> for Exp<-8> {
    type Output = Exp<4>;
}
            
impl RootExp<-1> for Exp<-8> {
    type Output = Exp<8>;
}
            
impl RootExp<1> for Exp<-8> {
    type Output = Exp<-8>;
}
            
impl RootExp<2> for Exp<-8> {
    type Output = Exp<-4>;
}
            
impl RootExp<4> for Exp<-8> {
    type Output = Exp<-2>;
}
            
impl RootExp<8> for Exp<-8> {
    type Output = Exp<-1>;
}
            
//------------------------- -7 -------------------------

impl RootExp<-7> for Exp<-7> {
    type Output = Exp<1>;
}
            
impl RootExp<-1> for Exp<-7> {
    type Output = Exp<7>;
}
            
impl RootExp<1> for Exp<-7> {
    type Output = Exp<-7>;
}
            
impl RootExp<7> for Exp<-7> {
    type Output = Exp<-1>;
}
            
//------------------------- -6 -------------------------

impl RootExp<-6> for Exp<-6> {
    type Output = Exp<1>;
}
            
impl RootExp<-3> for Exp<-6> {
    type Output = Exp<2>;
}
            
impl RootExp<-2> for Exp<-6> {
    type Output = Exp<3>;
}
            
impl RootExp<-1> for Exp<-6> {
    type Output = Exp<6>;
}
            
impl RootExp<1> for Exp<-6> {
    type Output = Exp<-6>;
}
            
impl RootExp<2> for Exp<-6> {
    type Output = Exp<-3>;
}
            
impl RootExp<3> for Exp<-6> {
    type Output = Exp<-2>;
}
            
impl RootExp<6> for Exp<-6> {
    type Output = Exp<-1>;
}
            
//------------------------- -5 -------------------------

impl RootExp<-5> for Exp<-5> {
    type Output = Exp<1>;
}
            
impl RootExp<-1> for Exp<-5> {
    type Output = Exp<5>;
}
            
impl RootExp<1> for Exp<-5> {
    type Output = Exp<-5>;
}
            
impl RootExp<5> for Exp<-5> {
    type Output = Exp<-1>;
}
            
//------------------------- -4 -------------------------

impl RootExp<-4> for Exp<-4> {
    type Output = Exp<1>;
}
            
impl RootExp<-2> for Exp<-4> {
    type Output = Exp<2>;
}
            
impl RootExp<-1> for Exp<-4> {
    type Output = Exp<4>;
}
            
impl RootExp<1> for Exp<-4> {
    type Output = Exp<-4>;
}
            
impl RootExp<2> for Exp<-4> {
    type Output = Exp<-2>;
}
            
impl RootExp<4> for Exp<-4> {
    type Output = Exp<-1>;
}
            
//------------------------- -3 -------------------------

impl RootExp<-3> for Exp<-3> {
    type Output = Exp<1>;
}
            
impl RootExp<-1> for Exp<-3> {
    type Output = Exp<3>;
}
            
impl RootExp<1> for Exp<-3> {
    type Output = Exp<-3>;
}
            
impl RootExp<3> for Exp<-3> {
    type Output = Exp<-1>;
}
            
//------------------------- -2 -------------------------

impl RootExp<-2> for Exp<-2> {
    type Output = Exp<1>;
}
            
impl RootExp<-1> for Exp<-2> {
    type Output = Exp<2>;
}
            
impl RootExp<1> for Exp<-2> {
    type Output = Exp<-2>;
}
            
impl RootExp<2> for Exp<-2> {
    type Output = Exp<-1>;
}
            
//------------------------- -1 -------------------------

impl RootExp<-1> for Exp<-1> {
    type Output = Exp<1>;
}
            
impl RootExp<1> for Exp<-1> {
    type Output = Exp<-1>;
}
            
//------------------------- 0 -------------------------

impl RootExp<-10> for Exp<0> {
    type Output = Exp<0>;
}
            
impl RootExp<-9> for Exp<0> {
    type Output = Exp<0>;
}
            
impl RootExp<-8> for Exp<0> {
    type Output = Exp<0>;
}
            
impl RootExp<-7> for Exp<0> {
    type Output = Exp<0>;
}
            
impl RootExp<-6> for Exp<0> {
    type Output = Exp<0>;
}
            
impl RootExp<-5> for Exp<0> {
    type Output = Exp<0>;
}
            
impl RootExp<-4> for Exp<0> {
    type Output = Exp<0>;
}
            
impl RootExp<-3> for Exp<0> {
    type Output = Exp<0>;
}
            
impl RootExp<-2> for Exp<0> {
    type Output = Exp<0>;
}
            
impl RootExp<-1> for Exp<0> {
    type Output = Exp<0>;
}
            
impl RootExp<1> for Exp<0> {
    type Output = Exp<0>;
}
            
impl RootExp<2> for Exp<0> {
    type Output = Exp<0>;
}
            
impl RootExp<3> for Exp<0> {
    type Output = Exp<0>;
}
            
impl RootExp<4> for Exp<0> {
    type Output = Exp<0>;
}
            
impl RootExp<5> for Exp<0> {
    type Output = Exp<0>;
}
            
impl RootExp<6> for Exp<0> {
    type Output = Exp<0>;
}
            
impl RootExp<7> for Exp<0> {
    type Output = Exp<0>;
}
            
impl RootExp<8> for Exp<0> {
    type Output = Exp<0>;
}
            
impl RootExp<9> for Exp<0> {
    type Output = Exp<0>;
}
            
impl RootExp<10> for Exp<0> {
    type Output = Exp<0>;
}
            
//------------------------- 1 -------------------------

impl RootExp<-1> for Exp<1> {
    type Output = Exp<-1>;
}
            
impl RootExp<1> for Exp<1> {
    type Output = Exp<1>;
}
            
//------------------------- 2 -------------------------

impl RootExp<-2> for Exp<2> {
    type Output = Exp<-1>;
}
            
impl RootExp<-1> for Exp<2> {
    type Output = Exp<-2>;
}
            
impl RootExp<1> for Exp<2> {
    type Output = Exp<2>;
}
            
impl RootExp<2> for Exp<2> {
    type Output = Exp<1>;
}
            
//------------------------- 3 -------------------------

impl RootExp<-3> for Exp<3> {
    type Output = Exp<-1>;
}
            
impl RootExp<-1> for Exp<3> {
    type Output = Exp<-3>;
}
            
impl RootExp<1> for Exp<3> {
    type Output = Exp<3>;
}
            
impl RootExp<3> for Exp<3> {
    type Output = Exp<1>;
}
            
//------------------------- 4 -------------------------

impl RootExp<-4> for Exp<4> {
    type Output = Exp<-1>;
}
            
impl RootExp<-2> for Exp<4> {
    type Output = Exp<-2>;
}
            
impl RootExp<-1> for Exp<4> {
    type Output = Exp<-4>;
}
            
impl RootExp<1> for Exp<4> {
    type Output = Exp<4>;
}
            
impl RootExp<2> for Exp<4> {
    type Output = Exp<2>;
}
            
impl RootExp<4> for Exp<4> {
    type Output = Exp<1>;
}
            
//------------------------- 5 -------------------------

impl RootExp<-5> for Exp<5> {
    type Output = Exp<-1>;
}
            
impl RootExp<-1> for Exp<5> {
    type Output = Exp<-5>;
}
            
impl RootExp<1> for Exp<5> {
    type Output = Exp<5>;
}
            
impl RootExp<5> for Exp<5> {
    type Output = Exp<1>;
}
            
//------------------------- 6 -------------------------

impl RootExp<-6> for Exp<6> {
    type Output = Exp<-1>;
}
            
impl RootExp<-3> for Exp<6> {
    type Output = Exp<-2>;
}
            
impl RootExp<-2> for Exp<6> {
    type Output = Exp<-3>;
}
            
impl RootExp<-1> for Exp<6> {
    type Output = Exp<-6>;
}
            
impl RootExp<1> for Exp<6> {
    type Output = Exp<6>;
}
            
impl RootExp<2> for Exp<6> {
    type Output = Exp<3>;
}
            
impl RootExp<3> for Exp<6> {
    type Output = Exp<2>;
}
            
impl RootExp<6> for Exp<6> {
    type Output = Exp<1>;
}
            
//------------------------- 7 -------------------------

impl RootExp<-7> for Exp<7> {
    type Output = Exp<-1>;
}
            
impl RootExp<-1> for Exp<7> {
    type Output = Exp<-7>;
}
            
impl RootExp<1> for Exp<7> {
    type Output = Exp<7>;
}
            
impl RootExp<7> for Exp<7> {
    type Output = Exp<1>;
}
            
//------------------------- 8 -------------------------

impl RootExp<-8> for Exp<8> {
    type Output = Exp<-1>;
}
            
impl RootExp<-4> for Exp<8> {
    type Output = Exp<-2>;
}
            
impl RootExp<-2> for Exp<8> {
    type Output = Exp<-4>;
}
            
impl RootExp<-1> for Exp<8> {
    type Output = Exp<-8>;
}
            
impl RootExp<1> for Exp<8> {
    type Output = Exp<8>;
}
            
impl RootExp<2> for Exp<8> {
    type Output = Exp<4>;
}
            
impl RootExp<4> for Exp<8> {
    type Output = Exp<2>;
}
            
impl RootExp<8> for Exp<8> {
    type Output = Exp<1>;
}
            
//------------------------- 9 -------------------------

impl RootExp<-9> for Exp<9> {
    type Output = Exp<-1>;
}
            
impl RootExp<-3> for Exp<9> {
    type Output = Exp<-3>;
}
            
impl RootExp<-1> for Exp<9> {
    type Output = Exp<-9>;
}
            
impl RootExp<1> for Exp<9> {
    type Output = Exp<9>;
}
            
impl RootExp<3> for Exp<9> {
    type Output = Exp<3>;
}
            
impl RootExp<9> for Exp<9> {
    type Output = Exp<1>;
}
            
//------------------------- 10 -------------------------

impl RootExp<-10> for Exp<10> {
    type Output = Exp<-1>;
}
            
impl RootExp<-5> for Exp<10> {
    type Output = Exp<-2>;
}
            
impl RootExp<-2> for Exp<10> {
    type Output = Exp<-5>;
}
            
impl RootExp<-1> for Exp<10> {
    type Output = Exp<-10>;
}
            
impl RootExp<1> for Exp<10> {
    type Output = Exp<10>;
}
            
impl RootExp<2> for Exp<10> {
    type Output = Exp<5>;
}
            
impl RootExp<5> for Exp<10> {
    type Output = Exp<2>;
}
            
impl RootExp<10> for Exp<10> {
    type Output = Exp<1>;
}
            