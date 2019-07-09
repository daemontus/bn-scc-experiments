use crate::bn::BooleanNetwork;
use crate::bn::builder::BNBuilder;

/*
    Regular expressions for simplified model building
    ([a-z,A-Z,0-9,_]+) : \{0, 1}
    let $1 = builder.make_variable("$1")
    
    ([a-z,A-Z,0-9,_]+)_focal :=
    builder.update_function(&$1, Box::new(move |s| {}));
    
     & 
     &&

    ([a-z,A-Z,0-9,_]+) = 1
    s.get(&$1)

    ([a-z,A-Z,0-9,_]+) = 0
    !s.get(&$1)
*/

// 2^26 states (67 108 864)
// Optimized: 90.17 s / 3037 MB / 45b per state
pub fn t2dm_model() -> BooleanNetwork {
    let mut builder = BNBuilder::new();
    let atp = builder.make_variable("atp");
    let pp2a = builder.make_variable("pp2a");
    let p53 = builder.make_variable("p53");
    let p_rb = builder.make_variable("p_rb");
    let ampk = builder.make_variable("ampk");
    let irs_pik3ca = builder.make_variable("irs_pik3ca");
    let mapk = builder.make_variable("mapk");
    let pp1c = builder.make_variable("pp1c");
    let prc = builder.make_variable("prc");
    let pten = builder.make_variable("pten");
    let m_torc1_s6k1 = builder.make_variable("m_torc1_s6k1");
    let e2f1 = builder.make_variable("e2f1");
    let senescence = builder.make_variable("senescence");
    let tsc = builder.make_variable("tsc");
    let akt = builder.make_variable("akt");
    let cdk2 = builder.make_variable("cdk2");
    let g1_s = builder.make_variable("g1_s");
    let myc = builder.make_variable("myc");
    let metabolism = builder.make_variable("metabolism");
    let p16 = builder.make_variable("p16");
    let p21 = builder.make_variable("p21");
    let foxo = builder.make_variable("foxo");
    let mdm2 = builder.make_variable("mdm2");
    let insulin = builder.make_variable("insulin");
    let gf = builder.make_variable("gf");
    let therapy = builder.make_variable("therapy");

    // Inputs:
    builder.update_function(&insulin, Box::new(move |s| {
        s.get(&insulin)
    }));
    builder.update_function(&gf, Box::new(move |s| {
        s.get(&gf)
    }));
    builder.update_function(&therapy, Box::new(move |s| {
        s.get(&therapy)
    }));

    // Variables:
    builder.update_function(&senescence, Box::new(move |s| {
        (!s.get(&p16)) && (s.get(&p21)) && (s.get(&m_torc1_s6k1)) || (s.get(&p16))
    }));

    builder.update_function(&g1_s, Box::new(move |s| {
        (!s.get(&p21)) && (s.get(&cdk2)) && (s.get(&e2f1)) && (s.get(&metabolism))
    }));

    builder.update_function(&mapk, Box::new(move |s| {
        (s.get(&gf)) && (!s.get(&pp2a))
    }));

    builder.update_function(&p16, Box::new(move |s| {
        (s.get(&mapk)) && (!s.get(&p53)) && (!s.get(&e2f1)) && (!s.get(&prc))
    }));

    builder.update_function(&mdm2, Box::new(move |s| {
        ((!s.get(&p16)) && (!s.get(&p53)) && (s.get(&akt)) && (!s.get(&m_torc1_s6k1)) && (!s.get(&myc)) && (!s.get(&e2f1))) ||
        ((!s.get(&p16)) && (s.get(&p53)) && (!s.get(&m_torc1_s6k1)) && (!s.get(&myc)) && (!s.get(&e2f1))) ||
        ((s.get(&p16)) && (!s.get(&m_torc1_s6k1)) && (!s.get(&myc)) && (!s.get(&e2f1)))
    }));

    builder.update_function(&p53, Box::new(move |s| {
        (!s.get(&mdm2))
    }));

    builder.update_function(&p21, Box::new(move |s| {
        ((!s.get(&p53)) && (!s.get(&akt)) && (!s.get(&myc)) && (s.get(&foxo))) ||
        ((s.get(&p53)) && (!s.get(&akt)) && (!s.get(&myc)))
    }));

    builder.update_function(&akt, Box::new(move |s| {
        ((!s.get(&irs_pik3ca)) && (!s.get(&pten)) && (s.get(&cdk2)) && (!s.get(&pp2a))) ||
        ((s.get(&irs_pik3ca)) && (!s.get(&pten)) && (!s.get(&pp2a)))
    }));

    builder.update_function(&m_torc1_s6k1, Box::new(move |s| {
        (!s.get(&ampk)) && (!s.get(&tsc))
    }));

    builder.update_function(&atp, Box::new(move |s| {
        (s.get(&metabolism))
    }));

    builder.update_function(&irs_pik3ca, Box::new(move |s| {
        (s.get(&insulin)) && (!s.get(&m_torc1_s6k1))
    }));

    builder.update_function(&ampk, Box::new(move |s| {
        (s.get(&p53)) && (!s.get(&atp))
    }));

    builder.update_function(&pten, Box::new(move |s| {
        (s.get(&p53)) && (!s.get(&akt))
    }));

    builder.update_function(&tsc, Box::new(move |s| {
        (!s.get(&mapk)) && (!s.get(&akt)) && (s.get(&ampk))
    }));

    builder.update_function(&myc, Box::new(move |s| {
        (s.get(&mapk)) && (!s.get(&p53)) && (s.get(&m_torc1_s6k1)) && (s.get(&e2f1))
    }));

    builder.update_function(&cdk2, Box::new(move |s| {
        ((!s.get(&p21)) && (s.get(&m_torc1_s6k1)) && (!s.get(&myc)) && (s.get(&e2f1))) ||
        ((!s.get(&p21)) && (s.get(&m_torc1_s6k1)) && (s.get(&myc)))
    }));

    builder.update_function(&p_rb, Box::new(move |s| {
        (!s.get(&cdk2))
    }));

    builder.update_function(&e2f1, Box::new(move |s| {
        ((!s.get(&gf)) && (s.get(&myc)) && (!s.get(&p_rb)) && (s.get(&e2f1))) ||
        ((s.get(&gf)) && (!s.get(&p_rb)) && (s.get(&e2f1)))
    }));

    builder.update_function(&prc, Box::new(move |s| {
        (!s.get(&akt)) && (s.get(&myc))
    }));

    builder.update_function(&metabolism, Box::new(move |s| {
        ((!s.get(&mapk)) && (!s.get(&akt)) && (s.get(&m_torc1_s6k1)) && (s.get(&pp1c))) ||
        ((!s.get(&mapk)) && (s.get(&akt)) && (s.get(&m_torc1_s6k1))) ||
        ((s.get(&mapk)) && (!s.get(&akt)) && (s.get(&pp1c))) ||
        ((s.get(&mapk)) && (s.get(&akt)))
    }));

    builder.update_function(&pp2a, Box::new(move |s| {
        (!s.get(&m_torc1_s6k1))
    }));

    builder.update_function(&foxo, Box::new(move |s| {
        ((!s.get(&mapk)) && (!s.get(&p16)) && (!s.get(&akt)) && (!s.get(&ampk)) && (s.get(&metabolism))) ||
        ((!s.get(&mapk)) && (!s.get(&p16)) && (!s.get(&akt)) && (s.get(&ampk))) ||
        ((!s.get(&mapk)) && (s.get(&p16)) && (!s.get(&akt)))
    }));

    builder.update_function(&pp1c, Box::new(move |s| {
        ((!s.get(&mapk)) && (s.get(&akt))) || (s.get(&mapk))
    }));

    return builder.build_network();
}

// 2^20 states (1 048 576)
// Optimized:  1.05 s /   63 MB / 61b per state
// ~70 components, most 192 states, a few 1536
pub fn erb_b2_model() -> BooleanNetwork {
    let mut builder = BNBuilder::new();
    let egf = builder.make_variable("egf");
    let cdk6 = builder.make_variable("cdk6");
    let cyclin_e1 = builder.make_variable("cyclin_e1");
    let erbb1 = builder.make_variable("erbb1");
    let erbb2 = builder.make_variable("erbb2");
    let erbb3 = builder.make_variable("erbb3");
    let erbb1_2 = builder.make_variable("erbb1_2");
    let erbb1_3 = builder.make_variable("erbb1_3");
    let erbb2_3 = builder.make_variable("erbb2_3");
    let eralpha = builder.make_variable("eralpha");
    let p_rb1 = builder.make_variable("p_rb1");
    let cdk2 = builder.make_variable("cdk2");
    let cdk4 = builder.make_variable("cdk4");
    let igf1r = builder.make_variable("igf1r");
    let myc = builder.make_variable("myc");
    let p21 = builder.make_variable("p21");
    let cyclin_d1 = builder.make_variable("cyclin_d1");
    let p27 = builder.make_variable("p27");
    let akt1 = builder.make_variable("akt1");
    let mek1 = builder.make_variable("mek1");

    builder.update_function(&mek1, Box::new(move |s| {
        ((!s.get(&igf1r)) && (!s.get(&erbb1)) && (!s.get(&erbb2_3)) && (!s.get(&erbb1_2)) && (s.get(&erbb1_3))) ||
            ((!s.get(&igf1r)) && (!s.get(&erbb1)) && (!s.get(&erbb2_3)) && (s.get(&erbb1_2))) ||
            ((!s.get(&igf1r)) && (!s.get(&erbb1)) && (s.get(&erbb2_3))) ||
            ((!s.get(&igf1r)) && (s.get(&erbb1))) ||
            (s.get(&igf1r))
    }));

    builder.update_function(&cdk4, Box::new(move |s| {
        (!s.get(&p21)) && (s.get(&cyclin_d1)) && (!s.get(&p27))
    }));

    builder.update_function(&p21, Box::new(move |s| {
        (!s.get(&cdk4)) && (!s.get(&akt1)) && (!s.get(&myc))
    }));

    builder.update_function(&cyclin_d1, Box::new(move |s| {
        ((!s.get(&mek1)) && (s.get(&eralpha)) && (s.get(&akt1)) && (s.get(&myc))) ||
            ((s.get(&mek1)) && (s.get(&eralpha)) && (s.get(&myc)))
    }));

    builder.update_function(&p27, Box::new(move |s| {
        (!s.get(&cdk4)) && (!s.get(&cdk2)) && (!s.get(&akt1)) && (!s.get(&myc))
    }));

    builder.update_function(&cdk2, Box::new(move |s| {
        (!s.get(&p21)) && (!s.get(&p27)) && (s.get(&cyclin_e1))
    }));

    builder.update_function(&cyclin_e1, Box::new(move |s| {
        (s.get(&myc))
    }));

    builder.update_function(&p_rb1, Box::new(move |s| {
        (s.get(&cdk4)) && (s.get(&cdk6))
    }));

    builder.update_function(&eralpha, Box::new(move |s| {
        ((!s.get(&mek1)) && (s.get(&akt1))) ||
            (s.get(&mek1))
    }));

    builder.update_function(&akt1, Box::new(move |s| {
        ((!s.get(&igf1r)) && (!s.get(&erbb1)) && (!s.get(&erbb2_3)) && (!s.get(&erbb1_2)) && (s.get(&erbb1_3))) ||
            ((!s.get(&igf1r)) && (!s.get(&erbb1)) && (!s.get(&erbb2_3)) && (s.get(&erbb1_2))) ||
            ((!s.get(&igf1r)) && (!s.get(&erbb1)) && (s.get(&erbb2_3))) ||
            ((!s.get(&igf1r)) && (s.get(&erbb1))) ||
            (s.get(&igf1r))
    }));

    builder.update_function(&cdk6, Box::new(move |s| {
        (s.get(&cyclin_d1))
    }));

    builder.update_function(&myc, Box::new(move |s| {
        ((!s.get(&mek1)) && (!s.get(&eralpha)) && (s.get(&akt1))) ||
            ((!s.get(&mek1)) && (s.get(&eralpha))) ||
            (s.get(&mek1))
    }));

    builder.update_function(&igf1r, Box::new(move |s| {
        ((!s.get(&eralpha)) && (s.get(&akt1)) && (!s.get(&erbb2_3))) ||
            ((s.get(&eralpha)) && (!s.get(&erbb2_3)))
    }));

    builder.update_function(&egf, Box::new(move |_| {
        true
    }));

    builder.update_function(&erbb1, Box::new(move |s| {
        (s.get(&egf))
    }));

    builder.update_function(&erbb2, Box::new(move |s| {
        (s.get(&egf))
    }));

    builder.update_function(&erbb3, Box::new(move |s| {
        (s.get(&egf))
    }));

    builder.update_function(&erbb2_3, Box::new(move |s| {
        (s.get(&erbb2)) && (s.get(&erbb3))
    }));

    builder.update_function(&erbb1_2, Box::new(move |s| {
        (s.get(&erbb1)) && (s.get(&erbb2))
    }));

    builder.update_function(&erbb1_3, Box::new(move |s| {
        (s.get(&erbb1)) && (s.get(&erbb3))
    }));


    return builder.build_network();
}

// A small demo model with three different types of behaviour.
pub fn demo_model() -> BooleanNetwork {
    let mut builder = BNBuilder::new();
    let m2n = builder.make_variable("M2N");
    let m2c = builder.make_variable("M2C");
    let dna = builder.make_variable("DNA");
    let p53 = builder.make_variable("P53");
    builder.update_function(&p53, Box::new(move |s| {
        !s.get(&m2n)
    }));
    builder.update_function(&m2c, Box::new(move |s| {
        s.get(&p53)
    }));
    builder.update_function(&dna, Box::new(move |s| {
        // !s.get(&p53) || s.get(&dna)
        //Disorder:
        !s.get(&p53)
    }));
    builder.update_function(&m2n, Box::new(move |s| {
        //Stable: (s.get(&m2c) && (!s.get(&dna) || !s.get(&p53))) || (!s.get(&m2c) && !s.get(&dna) && !s.get(&p53))
        //Bi-stable: (s.get(&m2c) && (!s.get(&dna) || !s.get(&p53))) || (!s.get(&m2c) && !s.get(&p53))
        //Oscillating: s.get(&m2c) || (!s.get(&m2c) && !s.get(&dna) && !s.get(&p53))
        //Disorder:
        s.get(&m2c) || (!s.get(&m2c) && !s.get(&dna) && !s.get(&p53))
    }));

    return builder.build_network();
}