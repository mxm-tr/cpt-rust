
#[cfg(test)]
mod tests {

    use cpt_rust::cpt::CPT;
    use cpt_rust::data_types::DataTypes;
    use cpt_rust::nodes::{NodeId};
    
    #[test]
    fn it_works() {
        let seq1: [DataTypes; 3] = [DataTypes::Integer(2), DataTypes::Integer(2), DataTypes::Integer(3)];
        let seq2: [DataTypes; 3] = [DataTypes::Integer(2), DataTypes::Integer(3), DataTypes::Integer(3)];
    
        let mut cpt = CPT::new();

        cpt.add_sequence(&seq1, NodeId::new());
        cpt.add_sequence(&seq2, NodeId::new());

        // Test the CPT structure
        let expected_result = "{\"inverted_index\":{\"values\":[{\"Integer\":2},{\"Integer\":3}],\"node_ids\":[[{\"index1\":2},{\"index1\":3}],[{\"index1\":4},{\"index1\":5},{\"index1\":6}]]},\"nodes\":[{\"parent\":null,\"children\":[{\"index1\":2}],\"data\":null},{\"parent\":{\"index1\":1},\"children\":[{\"index1\":3},{\"index1\":5}],\"data\":{\"Integer\":2}},{\"parent\":{\"index1\":2},\"children\":[{\"index1\":4}],\"data\":{\"Integer\":2}},{\"parent\":{\"index1\":3},\"children\":[],\"data\":{\"Integer\":3}},{\"parent\":{\"index1\":2},\"children\":[{\"index1\":6}],\"data\":{\"Integer\":3}},{\"parent\":{\"index1\":5},\"children\":[],\"data\":{\"Integer\":3}}],\"sequences_lookup_table\":[{\"index1\":4},{\"index1\":6}]}";
        assert_eq!(cpt.to_json(), expected_result);
        
        // Test the inverted index lookup function
        let expected_lookup_indices = [NodeId::new_with_value(2), NodeId::new_with_value(3)].to_vec();
        assert_eq!(
            serde_json::to_string(cpt.inverted_index.get_value_ids(DataTypes::Integer(2)).unwrap()).unwrap(),
            serde_json::to_string(&expected_lookup_indices).unwrap()
        );
    }
}
