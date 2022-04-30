extern crate lalrpop;

// Build the LALRPOP grammars...
fn main() {
    lalrpop::process_root().unwrap();
}
