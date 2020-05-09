
#[cfg(test)]
mod tests {
    use std::num::NonZeroUsize;
    use serde::{Serialize, Deserialize};

    use cpt_rust::cpt::CPT;
    use cpt_rust::data_types::DataTypes;
    use cpt_rust::nodes::{Node, NodeId};
    
    #[test]
    fn it_works() {
        let seq1: [DataTypes; 3] = [DataTypes::Integer(2), DataTypes::Integer(2), DataTypes::Integer(3)];
        let seq2: [DataTypes; 3] = [DataTypes::Integer(2), DataTypes::Integer(3), DataTypes::Integer(3)];
    
        let mut cpt = CPT::new();

        cpt.add_sequence(&seq1, NodeId::new());
        cpt.add_sequence(&seq2, NodeId::new());

        let expected_result = "{\"alphabet\":[],\"nodes\":[{\"parent\":null,\"children\":[{\"index1\":2}],\"data\":null},{\"parent\":{\"index1\":1},\"children\":[{\"index1\":3},{\"index1\":5}],\"data\":{\"Integer\":2}},{\"parent\":{\"index1\":2},\"children\":[{\"index1\":4}],\"data\":{\"Integer\":2}},{\"parent\":{\"index1\":3},\"children\":[],\"data\":{\"Integer\":3}},{\"parent\":{\"index1\":2},\"children\":[{\"index1\":6}],\"data\":{\"Integer\":3}},{\"parent\":{\"index1\":5},\"children\":[],\"data\":{\"Integer\":3}}]}";

        assert_eq!(cpt.to_json(), expected_result);
    }
}
