use std::ops::Add;
struct Complex<T>
    where T: Add<Output = T>    
{
    r: T,
    i: T
}

impl<T:Add<Output = T>> Add for Complex<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        return Complex::<T>{
            r: self.r + rhs.r,
            i: self.i + rhs.i
        }
    }
}

fn main() {
    let a = Complex{r:1, i:2};
    let b = Complex{r:9, i:9};
    let c = a + b;
    println!("Value of c is {} {}", c.r, c.i);
}