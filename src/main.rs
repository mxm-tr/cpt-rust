
use std::io::Write;
use cpt_rust::data_types::data_types::DataTypes;
use cpt_rust::cpt::cpt::CPT;
use cpt_rust::cpt::cpt::SequenceMatchFunction as SequenceMatchFunction;

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

    cpt.add_sequence_to_root(&seq1);
    cpt.add_sequence_to_root(&seq2);
    cpt.add_sequence_to_root(&seq3);
    cpt.add_sequence_to_root(&seq4);

    let seq_find: [crate::DataTypes; 3] = [DataTypes::Integer(3), DataTypes::Integer(7), DataTypes::Integer(1)];

    println!("---------");

    cpt.match_sequence(&seq_find, true, &[SequenceMatchFunction::StrictEqual]);

    println!("---------");

    cpt.match_sequence(&seq_find, false, &[SequenceMatchFunction::SequenceLength, SequenceMatchFunction::AlgebraicDistance]);

    println!("---------");

    cpt.predict(&seq_find, 3);

    // println!("Inverted index = {:?}", cpt.inverted_index );
    // println!("s2 = {:?}", seq2 );
    // for n in cpt.nodes {
    //     println!("node parent_id = {:?} \t children = {:?} \t data = {:?}", n.parent, n.children,n.data);
    // }
    // println!("{}", cpt.to_json_pretty())
    let mut file = File::create("test.dot")?;
    file.write(cpt.to_dot().as_bytes())?;
    Ok(())
}