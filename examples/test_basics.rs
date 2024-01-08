use tch::{kind, Tensor};

fn grad_example() {
    let x = Tensor::from(2.0).set_requires_grad(true);
    let y = &x * &x + &x + 36;
    println!("{}", y);
}

fn main() {
    grad_example();
}
