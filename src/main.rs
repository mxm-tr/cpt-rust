mod cpt;//https://github.com/rust-lang/rls-vscode/issues/686#issuecomment-558368010

use std::io::{ Read, Write };
use cpt::data_types::data_types::{DataTypes, SimilarityScores, SequenceAttributes};
use cpt::data_types::data_types::SequenceMatchFunction;
use cpt::cpt::CPT;


use std::fs::File;

fn main() -> std::io::Result<()> {
    // let seq1: [crate::DataTypes; 6] = [DataTypes::Integer(1), DataTypes::Integer(2), DataTypes::Integer(3), DataTypes::Integer(4), DataTypes::Integer(5), DataTypes::Integer(5)];
    // let seq2: [crate::DataTypes; 6] = [DataTypes::Integer(2), DataTypes::Integer(2), DataTypes::Integer(3), DataTypes::Integer(4), DataTypes::Integer(5), DataTypes::Integer(5)];
    
    let seq1: [crate::DataTypes; 3] = [DataTypes::Integer(1), DataTypes::Integer(2), DataTypes::Integer(3)];
    let seq2: [crate::DataTypes; 3] = [DataTypes::Integer(4), DataTypes::Integer(5), DataTypes::Integer(6)];
    let seq3: [crate::DataTypes; 3] = [DataTypes::Integer(1), DataTypes::Integer(5), DataTypes::Integer(6)];
    let seq4: [crate::DataTypes; 3] = [DataTypes::Integer(1), DataTypes::Integer(5), DataTypes::Integer(4)];
    
    // let seq1: [DataTypes; 3] = [DataTypes::Integer(2), DataTypes::Integer(2), DataTypes::Integer(3)];
    // let seq2: [DataTypes; 3] = [DataTypes::Integer(2), DataTypes::Integer(3), DataTypes::Integer(3)];
    
    let mut cpt = CPT::new();

    cpt.add_sequence_to_root(seq1.to_vec(), None);
    cpt.add_sequence_to_root(seq2.to_vec(),
        Some(vec![
            SequenceAttributes::ClassStr("Test".to_string()),
            SequenceAttributes::ClassStr("Test2".to_string())
    ]));
    cpt.add_sequence_to_root(seq3.to_vec(), None);
    cpt.add_sequence_to_root(seq4.to_vec(), None);

    let seq_find: [crate::DataTypes; 3] = [DataTypes::Integer(3), DataTypes::Integer(7), DataTypes::Integer(1)];

    println!("---------");

    cpt.match_sequence(&seq_find,
        true,
        &[SequenceMatchFunction::StrictEqual],
    );

    println!("---------");

    cpt.match_sequence(&seq_find,
        false,
        &[SequenceMatchFunction::SequenceLength, SequenceMatchFunction::AlgebraicDistance],
    );

    println!("---------");

    cpt.predict(&seq_find, 3);

    // println!("Inverted index = {:?}", cpt.inverted_index );
    // println!("s2 = {:?}", seq2 );
    // for n in cpt.nodes {
    //     println!("node parent_id = {:?} \t children = {:?} \t data = {:?}", n.parent, n.children,n.data);
    // }
    // println!("{}", cpt.to_json_pretty())
    
    // let mut cpt = CPT::new();
    // let mut file = File::open("tests/inputs/toilet.wav")
    //     .expect("file should open read only");
    // let mut buffer = Vec::new();

    // // read the whole file
    // file.read_to_end(&mut buffer)?;
    // cpt.add_sequence_to_root( buffer.iter().map(|&byte| DataTypes::U8(byte)).collect(), None );

    let mut file = File::create("test.dot")?;
    file.write(cpt.to_dot().as_bytes())?;
    Ok(())
}