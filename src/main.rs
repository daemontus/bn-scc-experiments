use biodivine::u32::sequential::scc;
use biodivine::u32::models::t2dm_model;

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


    let network = t2dm_model();//builder.build_network();

    scc(&network);
}

