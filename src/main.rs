
use cpt_rust::data_types::DataTypes;
use cpt_rust::cpt::CPT;

fn main() {
    // let seq1: [crate::DataTypes; 6] = [DataTypes::Integer(1), DataTypes::Integer(2), DataTypes::Integer(3), DataTypes::Integer(4), DataTypes::Integer(5), DataTypes::Integer(5)];
    // let seq2: [crate::DataTypes; 6] = [DataTypes::Integer(2), DataTypes::Integer(2), DataTypes::Integer(3), DataTypes::Integer(4), DataTypes::Integer(5), DataTypes::Integer(5)];
    
    let seq1: [crate::DataTypes; 3] = [DataTypes::Integer(2), DataTypes::Integer(2), DataTypes::Integer(3)];
    let seq2: [crate::DataTypes; 3] = [DataTypes::Integer(2), DataTypes::Integer(3), DataTypes::Integer(3)];
    
    let mut cpt = CPT::new();

    cpt.add_sequence_to_root(&seq1);
    cpt.add_sequence_to_root(&seq2);

    println!("s1 = {:?}", seq1 );
    println!("s2 = {:?}", seq2 );
    for n in cpt.nodes {
        println!("node parent_id = {:?} \t children = {:?} \t data = {:?}", n.parent, n.children,n.data);
    }
    // println!("{}", cpt.to_json_pretty())
}