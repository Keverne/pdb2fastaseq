use lib3dmol::parser;
use lib3dmol::tools;
use std::env;

fn run() {
    // Parse command line
    let args: Vec<String> = env::args().collect();
    match args.len() {
        2 => {
            let pdb = &args[1];

            let my_structure = parser::read_pdb(pdb, &pdb[..]);

            let fasta = tools::fasta_seq(&my_structure);
            println!("{}", fasta);
        }
        _ => {
            println!("Wrong command line. Usage: {} pdb_filename ", args[0]);
        }
    }
}

fn main() {
    run()
}
