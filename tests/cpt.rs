
#[cfg(test)]
mod tests {

    use serde_json::Value;
    use cpt_rust::cpt::cpt::CPT;
    use cpt_rust::data_types::data_types::DataTypes;
    use cpt_rust::nodes::nodes::{NodeId};
    use std::io::Write;
    use std::fs::File;

    #[test]
    fn it_works() {
        let seq1: [DataTypes; 3] = [DataTypes::Integer(2), DataTypes::Integer(2), DataTypes::Integer(3)];
        let seq2: [DataTypes; 3] = [DataTypes::Integer(2), DataTypes::Integer(3), DataTypes::Integer(3)];
    
        let mut cpt = CPT::new();

        cpt.add_sequence(seq1.to_vec(), 0, None);
        cpt.add_sequence(seq2.to_vec(), 0, None);

        // Test the CPT structure
        let file = File::open("tests/results/cpt-train.json")
            .expect("file should open read only");
        let expected_result: serde_json::Value = serde_json::from_reader(file)
            .expect("file should be proper JSON");

        assert_eq!(serde_json::from_str::<Value>(&cpt.to_json()).unwrap(), expected_result);
        
        // Test the inverted index lookup function
        let expected_lookup_indices = [1, 2].to_vec();
        assert_eq!(
            serde_json::to_string(cpt.inverted_index.get_value_ids(DataTypes::Integer(2)).unwrap()).unwrap(),
            serde_json::to_string(&expected_lookup_indices).unwrap()
        );

        // Test the paper implementation of predict()
        let file = File::open("tests/results/cpt-predict.json")
        .expect("file should open read only");
        let expected_result: serde_json::Value = serde_json::from_reader(file)
        .expect("file should be proper JSON");
        let seq1: [DataTypes; 3] = [DataTypes::Integer(1), DataTypes::Integer(2), DataTypes::Integer(3)];
        let seq2: [DataTypes; 3] = [DataTypes::Integer(3), DataTypes::Integer(5), DataTypes::Integer(6)];
        let seq3: [DataTypes; 3] = [DataTypes::Integer(1), DataTypes::Integer(5), DataTypes::Integer(6)];
        let seq4: [DataTypes; 3] = [DataTypes::Integer(1), DataTypes::Integer(5), DataTypes::Integer(4)];
        let seq_find: [DataTypes; 3] = [DataTypes::Integer(3), DataTypes::Integer(7), DataTypes::Integer(1)];

        let mut cpt = CPT::new();
        cpt.add_sequence_to_root(seq1.to_vec(), None);
        cpt.add_sequence_to_root(seq2.to_vec(), None);
        cpt.add_sequence_to_root(seq3.to_vec(), None);
        cpt.add_sequence_to_root(seq4.to_vec(), None);

        // let mut file = File::create("tests/results/test.dot").unwrap();
        // file.write(cpt.to_dot().as_bytes()).unwrap();

        cpt.predict(&seq_find, 3);
        println!("{:?}", serde_json::to_string(&cpt.predict(&seq_find, 3)).unwrap());
        assert_eq!(
            serde_json::from_str::<Value>(&serde_json::to_string(&cpt.predict(&seq_find, 3)).unwrap()).unwrap(),
            expected_result
        );
    }
}
