#![feature(const_fn)]
#[macro_use]
extern crate dimensioned;
extern crate vector3;

use vector3::vector3a::Vector3;
use std::ops::{Add, Mul};

use dimensioned::si::{one, m, kg, s, Meter};
use dimensioned::{Dim, Dimension, KeepDim, MulDim};

// #[macro_export]
// macro_rules! test { ($Trait:ident, $fun:ident, $op:ident, $In:ty => $Out:ty) => (
//     pub trait $Trait {
//         type Output;
//         fn $fun(self) -> Self::Output;
//     }
//     impl<D> $Trait for Dim<D, $In> where D: Dimension + $op, <D as $op>::Output: Dimension {
//         type Output = Dim<<D as $op>::Output, $Out>;
//         fn $fun(self) -> Self::Output { Dim::new( (self.0).$fun() ) }
//     }
//     );
// }

dim_impl_unary!(Norm, norm, KeepDim, Vector3 => f64);
dim_impl_unary!(Norm2, norm2, Mul, Vector3 => f64);

dim_impl_binary!(Dot, dot, Mul, Vector3 => f64);
dim_impl_binary!(Cross, cross, MulDim, Vector3 => Vector3);

// fn area<V: Mul>(w: Meter<V>, l: Meter<V>) -> <Meter<V> as Mul>::Output {
//     w * l
// }

// fn area<V: Mul>(w: Dim<Meter, V>, l: Dim<Meter, V>) -> <Dim<Meter, V> as Mul>::Output {
//     w * l
// }

fn a() {}

fn main() {
    let xhat = one * Vector3::new(1.0, 0.0, 0.0);
    let yhat = one * Vector3::new(0.0, 1.0, 0.0);
    let zhat = one * Vector3::new(0.0, 0.0, 1.0);


    let start = -22.0*xhat*m + 5.0*yhat*m + 6.0*zhat*m;
    println!("A physicist was standing on a hill at position {}.", start);

    let end = 26.0*xhat*m - 19.0*yhat*m;
    println!("Then she walked down the hill to {}.", end);

    let displace = end - start;
    println!("So, her displacement vector was {}.", displace);

    let time = 30.0*s;
    println!("The walk took her {}.", time);

    let velocity = displace/time;
    println!("She must have had an average velocity of {}.", velocity);

    let speed = velocity.norm();
    println!("So, her average speed was {}.", speed);

    let center = 28.0*xhat*m - 21.0*yhat*m;
    println!("Now, she's standing next to a merry-go-round, centered at {}.", center);

    let force = 500.0*xhat*kg*m/s/s;
    println!("She decides to spin it, pushing with a force of {}", force);

    let r = end - center;
    let torque = r.cross(force);
    println!("That's a torque of {}!", torque);
}

