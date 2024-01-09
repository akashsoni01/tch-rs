use tch::{kind, Tensor};
// use tch::Tensor::{new, randn, zeros, of_slice, ones_like, add, get, view, view_, double_value, size, to_device, apply, narrow, copy_, flat_view, apply_, device, double_value_, to_kind, set_requires_grad, zero_grad, backward, grad};


fn main() {
    println!("working fine!");

    // python torch.rand(5, 3) in rust 
    let x = Tensor::randn(&[5, 3], kind::FLOAT_CPU);
    println!("x: {:?}", x);

    // traverse the above tensor matrix x 
    // for i in 0..x.size() {
    //     for j in 0..x[i].size() {
    //         println!("x[{}, {}]: {:?}", i, j, x.double_value(&[i, j]));
    //     }
    // }

/*
    // python torch.zeros(5, 3, dtype=torch.long) in rust
    let x = Tensor::zeros(&[5, 3], kind::INT64_CPU);
    println!("x: {:?}", x);

    // python torch.tensor([5.5, 3]) in rust
    let x = Tensor::of_slice(&[5.5, 3.0]);
    println!("x: {:?}", x);

    // python x.new_ones(5, 3, dtype=torch.double) in rust
    let x = x.ones_like(kind::FLOAT_CPU);
    println!("x: {:?}", x);

    // python x.size() in rust
    println!("x.size(): {:?}", x.size());

    // python y = torch.rand(5, 3) in rust
    let y = Tensor::randn(&[5, 3], kind::FLOAT_CPU);
    println!("y: {:?}", y);

    // python x + y in rust
    println!("x + y: {:?}", x + y);

    // python torch.add(x, y) in rust
    println!("torch.add(x, y): {:?}", Tensor::add(&x, &y));

    // python y.add_(x) in rust
    println!("y.add_(x): {:?}", y.add_(&x));

    // python print(y[:, 1]) in rust
    println!("y[:, 1]: {:?}", y.get(1));

    // python x = torch.randn(4, 4) in rust
    let x = Tensor::randn(&[4, 4], kind::FLOAT_CPU);
    println!("x: {:?}", x);

    // python y = x.view(16) in rust
    let y = x.view(&[16]);
    println!("y: {:?}", y);

    // python z = x.view(-1, 8) in rust
    let z = x.view(&[-1, 8]);
    println!("z: {:?}", z);

    // python x = torch.randn(1) in rust
    let x = Tensor::randn(&[1], kind::FLOAT_CPU);
    println!("x: {:?}", x);

    // python x.item() in rust
    println!("x.item(): {:?}", x.double_value(&[]));

    // python x.numpy() in rust
    println!("x.numpy(): {:?}", x.double_value(&[]));

*/
    
}
