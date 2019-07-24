//use biodivine::u32::sequential::scc;
use biodivine::u32::parallel::parallel_scc;
use biodivine::u32::models::{hspc_model};

fn main() {
    //let mut builder = BNBuilder::new();
    /*let b = builder.make_variable("B");
    let a = builder.make_variable("A");
    builder.update_function(&a, Box::new(move |s| {
        !s.get(&a) || s.get(&b)
    }));
    builder.update_function(&b, Box::new(move |s| {
        s.get(&a) || s.get(&b)
    }));*/


    let network = hspc_model();//builder.build_network();

    let args: Vec<String> = std::env::args().collect();
    parallel_scc(&network, args[1].parse().unwrap());
}

