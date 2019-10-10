
use super::BDD;
use std::io::Write;

/// Write given BDD to the output writer as a .dot graph.
/// When naming the nodes in the graph, we use the given variable names.
/// The BDD can be "zero pruned", in which case we prune the zero node and all edges
/// leading to it.
pub fn print_bdd_as_dot(
    output: &mut dyn Write,
    bdd: &BDD,
    var_names: &Vec<String>,
    zero_pruned: bool
) -> Result<(), std::io::Error> {
    output.write_all(b"digraph G {\n")?;
    output.write_all(b"init__ [label=\"\", style=invis, height=0, width=0];\n")?;
    output.write_all(format!("init__ -> {};\n", bdd.last_index()).as_bytes())?;
    /*
        Fortunately, it seem that .dot does not care about ordering of graph elements,
        so we can just go through the BDD and print it as is.

        Note that for slices, this can output unused nodes, but we don't support slices yet anyway.
     */
    for node_index in (2..bdd.size()).rev() {
        // write the node itself
        output.write_all(format!("{}[label=\"{}\"];\n", node_index, var_names[bdd.var(node_index)]).as_bytes())?;
        let high_link = bdd.high_link(node_index);
        if !zero_pruned || high_link != 0 { // write "high" link
            output.write_all(format!("{} -> {} [style=filled];\n", node_index, high_link).as_bytes())?;
        }
        let low_link = bdd.low_link(node_index);
        if !zero_pruned || low_link != 0 { // write "low" link
            output.write_all(format!("{} -> {} [style=dotted];\n", node_index, low_link).as_bytes())?;
        }
    }
    if !zero_pruned {
        output.write_all(b"0 [shape=box, label=\"0\", style=filled, shape=box, height=0.3, width=0.3];\n")?;
    }
    output.write_all(b"1 [shape=box, label=\"1\", style=filled, shape=box, height=0.3, width=0.3];\n")?;
    output.write_all(b"}\n")?;
    return Result::Ok(());
}