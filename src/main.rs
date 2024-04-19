use tch::{nn, nn::Module, Device};

fn net(vs: &nn::Path) -> impl Module {
    nn::seq()
        .add(nn::linear(vs / "layer1",4,10,Default::default(),))
        .add_fn(|xs| xs.leaky_relu())
        .add(nn::linear(vs, 10, 10, Default::default()))
        .add_fn(|xs| xs.leaky_relu())
        .add(nn::linear(vs, 10, 5, Default::default()))
        .add_fn(|xs| xs.leaky_relu())
        .add(nn::linear(vs, 5, 1, Default::default()))
        .add_fn(|xs| xs.leaky_relu())
        
}

fn main() {

}