extern crate bio;
use std::io;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::str;
use std::io::BufWriter;
use clap::Parser;
use std::collections::HashMap;
use bio::io::fasta;
use bio::io::fasta::Writer;

// Arguments et Options
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// FASTA INPUT (file path or stdin)
    fasta_file : std::path::PathBuf,    // Fichier fasta
    /// FAM_FILE INPUT (file path)
    fam_file : String,    				// Fichier famille
    /// Include orphan families
    #[arg(long,default_value_t = false)]
    orphans : bool,                     // Traite les orphans,

}
// Sequence Fasta
type FastaSeqs = Vec<FastaSeq>;
#[derive(Debug)]
struct FastaSeq {
    id:String,      	// nom de la sequence
    seq:Vec<u8>,    	// sequence
    desc:Option<String> 	// description
}
// Lecture FAM_FILE
fn read_families(filename :String) -> HashMap::<String,Vec<String>>{
    let file = File::open(filename);
    let file = match file {
        Ok(file) => file,
        Err(_e) => {
		  panic!("Unable top open FAM_FILE")
    	},
    };
    let reader = BufReader::new(file);
    let mut dico =  HashMap::<String,Vec<String>>::new();
    for line in reader.lines() {
        let line = line.expect("Error in FAM_FILE.");
        let split_line: Vec<&str> = line.split('\t').collect();
        assert_eq!(split_line.len(), 2, "Wrong format in FAM_FILE.");
        let fam = split_line[0].to_string();
        let seq = split_line[1].to_string();
        if !dico.contains_key(&fam) {
            dico.insert(fam.clone(),vec![]);
        };
        if let Some(x) = dico.get_mut(&fam) {
            x.push(seq);
        };
    }
    dico
}
//  Ouverture FASTA
fn traverse_fasta_file(path : &str) -> FastaSeqs {
    println!("FILE = {}",path);
    if path == "stdin" { println!("FASTA form stdin ")};
    match path {
        "stdin" => {
            let reader = fasta::Reader::new(io::stdin());
            processing_fasta(reader)
        },
        _ => {
            let reader = fasta::Reader::from_file(path).unwrap();
            processing_fasta(reader)
        },
    }
}
// Lecture des sequences FASTA
fn processing_fasta<B>(reader: fasta::Reader<B>) -> FastaSeqs where B: BufRead{
    let mut fastaseqs = Vec::new();
    for result in reader.records() {
        let record = result.expect("Error during fasta record parsing");
        let id = record.id();
        let seq = record.seq();
        let desc = record.desc();
        fastaseqs.push(
            FastaSeq{
                id: id.to_string(),
                seq: seq.to_vec(),
                desc: match desc {
                	Some(desc)  => Some(desc.to_string()),
                	None => None,
                }
            }
        );
    }
    fastaseqs
}
fn main() {
    let args = Cli::parse();
    println!("Parsing Fasta...");
    let fastas = traverse_fasta_file(args.fasta_file.to_str().expect("FASTA file not found."));
    println!("ok");
    println!("Reading families...");
    let fam_dico = read_families(args.fam_file);
    println!("ok");
    let nb_seq_fasta = fastas.len();
    let mut dico_sequence_index =  HashMap::<String,usize>::new();
    println!("Processing families...");
    for i in 0 .. nb_seq_fasta {
        if !dico_sequence_index.contains_key(&fastas[i].id) {
            dico_sequence_index.insert(fastas[i].id.clone(),i);
        }
        else {
            panic!("Sequence {} already exists.",fastas[i].id);
        }
    }
    for (key, value) in fam_dico {
        if (value.len() > 1) || args.orphans { 
            let filename = key+".fasta";
            let file = File::create(filename).unwrap();
            let handle = BufWriter::new(file);
            let mut writer = Writer::new(handle);
            for seq in value {
                let idx = match dico_sequence_index.get(&seq){
                    Some(idx) => *idx,
                    None => panic!("Sequence {} not found.",seq),
                };
                let record = fasta::Record::with_attrs(&fastas[idx].id.clone(), fastas[idx].desc.as_deref(), &fastas[idx].seq);
                writer.write_record(&record).expect("Unable to write output file.");
            }
        }
    }
    println!("done.");
}
