use crate::u32::bn::{BooleanNetwork, BooleanNetworkBuilder};

// 2^26 states (67 108 864)
// Optimized: 18.03 s / 290 MB / 4.33b per state
// 73088 components
pub fn t2dm_model() -> BooleanNetwork {
    let mut builder = BooleanNetworkBuilder::new();
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
        (s | insulin)
    }));
    builder.update_function(&gf, Box::new(move |s| {
        (s | gf)
    }));
    builder.update_function(&therapy, Box::new(move |s| {
        (s | therapy)
    }));

    // Variables:
    builder.update_function(&senescence, Box::new(move |s| {
        (!(s | p16)) && (s | p21) && (s | m_torc1_s6k1) || (s | p16)
    }));

    builder.update_function(&g1_s, Box::new(move |s| {
        (!(s | p21)) && (s | cdk2) && (s | e2f1) && (s | metabolism)
    }));

    builder.update_function(&mapk, Box::new(move |s| {
        (s | gf) && (!(s | pp2a))
    }));

    builder.update_function(&p16, Box::new(move |s| {
        (s | mapk) && (!(s | p53)) && (!(s | e2f1)) && (!(s | prc))
    }));

    builder.update_function(&mdm2, Box::new(move |s| {
        ((!(s | p16)) && (!(s | p53)) && (s | akt) && (!(s | m_torc1_s6k1)) && (!(s | myc)) && (!(s | e2f1))) ||
            ((!(s | p16)) && (s | p53) && (!(s | m_torc1_s6k1)) && (!(s | myc)) && (!(s | e2f1))) ||
            ((s | p16) && (!(s | m_torc1_s6k1)) && (!(s | myc)) && (!(s | e2f1)))
    }));

    builder.update_function(&p53, Box::new(move |s| {
        (!(s | mdm2))
    }));

    builder.update_function(&p21, Box::new(move |s| {
        ((!(s | p53)) && (!(s | akt)) && (!(s | myc)) && (s | foxo)) ||
            ((s | p53) && (!(s | akt)) && (!(s | myc)))
    }));

    builder.update_function(&akt, Box::new(move |s| {
        ((!(s | irs_pik3ca)) && (!(s | pten)) && (s | cdk2) && (!(s | pp2a))) ||
            ((s | irs_pik3ca) && (!(s | pten)) && (!(s | pp2a)))
    }));

    builder.update_function(&m_torc1_s6k1, Box::new(move |s| {
        (!(s | ampk)) && (!(s | tsc))
    }));

    builder.update_function(&atp, Box::new(move |s| {
        (s | metabolism)
    }));

    builder.update_function(&irs_pik3ca, Box::new(move |s| {
        (s | insulin) && (!(s | m_torc1_s6k1))
    }));

    builder.update_function(&ampk, Box::new(move |s| {
        (s | p53) && (!(s | atp))
    }));

    builder.update_function(&pten, Box::new(move |s| {
        (s | p53) && (!(s | akt))
    }));

    builder.update_function(&tsc, Box::new(move |s| {
        (!(s | mapk)) && (!(s | akt)) && (s | ampk)
    }));

    builder.update_function(&myc, Box::new(move |s| {
        (s | mapk) && (!(s | p53)) && (s | m_torc1_s6k1) && (s | e2f1)
    }));

    builder.update_function(&cdk2, Box::new(move |s| {
        ((!(s | p21)) && (s | m_torc1_s6k1) && (!(s | myc)) && (s | e2f1)) ||
            ((!(s | p21)) && (s | m_torc1_s6k1) && (s | myc))
    }));

    builder.update_function(&p_rb, Box::new(move |s| {
        (!(s | cdk2))
    }));

    builder.update_function(&e2f1, Box::new(move |s| {
        ((!(s | gf)) && (s | myc) && (!(s | p_rb)) && (s | e2f1)) ||
            ((s | gf) && (!(s | p_rb)) && (s | e2f1))
    }));

    builder.update_function(&prc, Box::new(move |s| {
        (!(s | akt)) && (s | myc)
    }));

    builder.update_function(&metabolism, Box::new(move |s| {
        ((!(s | mapk)) && (!(s | akt)) && (s | m_torc1_s6k1) && (s | pp1c)) ||
            ((!(s | mapk)) && (s | akt) && (s | m_torc1_s6k1)) ||
            ((s | mapk) && (!(s | akt)) && (s | pp1c)) ||
            ((s | mapk) && (s | akt))
    }));

    builder.update_function(&pp2a, Box::new(move |s| {
        (!(s | m_torc1_s6k1))
    }));

    builder.update_function(&foxo, Box::new(move |s| {
        ((!(s | mapk)) && (!(s | p16)) && (!(s | akt)) && (!(s | ampk)) && (s | metabolism)) ||
            ((!(s | mapk)) && (!(s | p16)) && (!(s | akt)) && (s | ampk)) ||
            ((!(s | mapk)) && (s | p16) && (!(s | akt)))
    }));

    builder.update_function(&pp1c, Box::new(move |s| {
        ((!(s | mapk)) && (s | akt)) || (s | mapk)
    }));

    return builder.build_network();
}

// 2^20 states (1 048 576)
// Optimized:  1.05 s /   63 MB / 61b per state
// 72 components, most are 192 states, a few 1536
pub fn erb_b2_model() -> BooleanNetwork {
    let mut builder = BooleanNetworkBuilder::new();
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
        ((!(s | igf1r)) && (!(s | erbb1)) && (!(s | erbb2_3)) && (!(s | erbb1_2)) && (s | erbb1_3)) ||
            ((!(s | igf1r)) && (!(s | erbb1)) && (!(s | erbb2_3)) && (s | erbb1_2)) ||
            ((!(s | igf1r)) && (!(s | erbb1)) && (s | erbb2_3)) ||
            ((!(s | igf1r)) && (s | erbb1)) ||
            (s | igf1r)
    }));

    builder.update_function(&cdk4, Box::new(move |s| {
        (!(s | p21)) && (s | cyclin_d1) && (!(s | p27))
    }));

    builder.update_function(&p21, Box::new(move |s| {
        (!(s | cdk4)) && (!(s | akt1)) && (!(s | myc))
    }));

    builder.update_function(&cyclin_d1, Box::new(move |s| {
        ((!(s | mek1)) && (s | eralpha) && (s | akt1) && (s | myc)) ||
            ((s | mek1) && (s | eralpha) && (s | myc))
    }));

    builder.update_function(&p27, Box::new(move |s| {
        (!(s | cdk4)) && (!(s | cdk2)) && (!(s | akt1)) && (!(s | myc))
    }));

    builder.update_function(&cdk2, Box::new(move |s| {
        (!(s | p21)) && (!(s | p27)) && (s | cyclin_e1)
    }));

    builder.update_function(&cyclin_e1, Box::new(move |s| {
        (s | myc)
    }));

    builder.update_function(&p_rb1, Box::new(move |s| {
        (s | cdk4) && (s | cdk6)
    }));

    builder.update_function(&eralpha, Box::new(move |s| {
        ((!(s | mek1)) && (s | akt1)) ||
            (s | mek1)
    }));

    builder.update_function(&akt1, Box::new(move |s| {
        ((!(s | igf1r)) && (!(s | erbb1)) && (!(s | erbb2_3)) && (!(s | erbb1_2)) && (s | erbb1_3)) ||
            ((!(s | igf1r)) && (!(s | erbb1)) && (!(s | erbb2_3)) && (s | erbb1_2)) ||
            ((!(s | igf1r)) && (!(s | erbb1)) && (s | erbb2_3)) ||
            ((!(s | igf1r)) && (s | erbb1)) ||
            (s | igf1r)
    }));

    builder.update_function(&cdk6, Box::new(move |s| {
        (s | cyclin_d1)
    }));

    builder.update_function(&myc, Box::new(move |s| {
        ((!(s | mek1)) && (!(s | eralpha)) && (s | akt1)) ||
            ((!(s | mek1)) && (s | eralpha)) ||
            (s | mek1)
    }));

    builder.update_function(&igf1r, Box::new(move |s| {
        ((!(s | eralpha)) && (s | akt1) && (!(s | erbb2_3))) ||
            ((s | eralpha) && (!(s | erbb2_3)))
    }));

    builder.update_function(&egf, Box::new(move |_| {
        true
    }));

    builder.update_function(&erbb1, Box::new(move |s| {
        (s | egf)
    }));

    builder.update_function(&erbb2, Box::new(move |s| {
        (s | egf)
    }));

    builder.update_function(&erbb3, Box::new(move |s| {
        (s | egf)
    }));

    builder.update_function(&erbb2_3, Box::new(move |s| {
        (s | erbb2) && (s | erbb3)
    }));

    builder.update_function(&erbb1_2, Box::new(move |s| {
        (s | erbb1) && (s | erbb2)
    }));

    builder.update_function(&erbb1_3, Box::new(move |s| {
        (s | erbb1) && (s | erbb3)
    }));


    return builder.build_network();
}

// A small demo model with three different types of behaviour.
pub fn demo_model_disorder() -> BooleanNetwork {
    let mut builder = BooleanNetworkBuilder::new();
    let p53 = builder.make_variable("P53");
    let dna = builder.make_variable("DNA");
    let m2c = builder.make_variable("M2C");
    let m2n = builder.make_variable("M2N");
    builder.update_function(&p53, Box::new(move |s| {
        !(s | m2n)
    }));
    builder.update_function(&m2c, Box::new(move |s| {
        (s | p53)
    }));
    builder.update_function(&dna, Box::new(move |s| {
        !(s | p53)
    }));
    builder.update_function(&m2n, Box::new(move |s| {
        (s | m2c) || (!(s | m2c) && !(s | dna) && !(s | p53))
    }));

    return builder.build_network();
}

pub fn demo_model_oscillation() -> BooleanNetwork {
    let mut builder = BooleanNetworkBuilder::new();
    let p53 = builder.make_variable("P53");
    let dna = builder.make_variable("DNA");
    let m2c = builder.make_variable("M2C");
    let m2n = builder.make_variable("M2N");
    builder.update_function(&p53, Box::new(move |s| {
        !(s | m2n)
    }));
    builder.update_function(&m2c, Box::new(move |s| {
        (s | p53)
    }));
    builder.update_function(&dna, Box::new(move |s| {
        !(s | p53) || (s | dna)
    }));
    builder.update_function(&m2n, Box::new(move |s| {
        (s | m2c) || (!(s | m2c) && !(s |  dna) && !(s | p53))
    }));

    return builder.build_network();
}

pub fn demo_model_bistable() -> BooleanNetwork {
    let mut builder = BooleanNetworkBuilder::new();
    let p53 = builder.make_variable("P53");
    let dna = builder.make_variable("DNA");
    let m2c = builder.make_variable("M2C");
    let m2n = builder.make_variable("M2N");
    builder.update_function(&p53, Box::new(move |s| {
        !(s | m2n)
    }));
    builder.update_function(&m2c, Box::new(move |s| {
        (s | p53)
    }));
    builder.update_function(&dna, Box::new(move |s| {
        !(s | p53) || (s | dna)
    }));
    builder.update_function(&m2n, Box::new(move |s| {
        ((s | m2c) && (!(s | dna) || !(s | p53))) || (!(s | m2c) && !(s | p53))
    }));

    return builder.build_network();
}

pub fn demo_model_stable() -> BooleanNetwork {
    let mut builder = BooleanNetworkBuilder::new();
    let p53 = builder.make_variable("P53");
    let dna = builder.make_variable("DNA");
    let m2c = builder.make_variable("M2C");
    let m2n = builder.make_variable("M2N");
    builder.update_function(&p53, Box::new(move |s| {
        !(s | m2n)
    }));
    builder.update_function(&m2c, Box::new(move |s| {
        (s | p53)
    }));
    builder.update_function(&dna, Box::new(move |s| {
        !(s | p53) || (s | dna)
    }));
    builder.update_function(&m2n, Box::new(move |s| {
        ((s | m2c) && (!(s | dna) || !(s | p53))) || (!(s | m2c) && !(s | dna) && !(s | p53))
    }));

    return builder.build_network();
}