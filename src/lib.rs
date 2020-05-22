use serde::{Serialize, Deserialize};

pub mod data_types {

    use std::cmp::PartialEq;
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, Debug)]
    #[derive(Copy, Clone, Eq, PartialOrd, PartialEq, Ord, Hash)]
    pub enum DataTypes{
        Integer(i32),
        None
    }

    // impl PartialEq for DataTypes {
    //     fn eq(&self, other: &Self) -> bool {
    //         println!("Comparing {:?} with {:?}", self, other);
    //         match self {
    //             Self::Integer(num1) => match other { Self::Integer(num2) => num1 == num2, _ => false },
    //             _ => false
    //         }
    //     }
    // }
}

pub mod cpt{

    use std::cmp::Ordering;
    use std::num::NonZeroUsize;
    use crate::data_types::DataTypes as DataTypes;
    use crate::nodes::{Node, NodeId};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct InvertedIndex<T>{
        values: Vec<T>,
        node_ids: Vec<Vec<NodeId>>,
    }
    impl<T> InvertedIndex<T>{
        // Basically two lists: one of possible values,
        // The other a list of lists of IDs having this value
        // This class helps us perform a search based on value
        // In the entire tree
        pub fn new() -> InvertedIndex<T> {
            InvertedIndex { 
                values: Vec::<T>::new(),
                node_ids: Vec::<Vec<NodeId>>::new(),
            }
        }
    }
    impl InvertedIndex<DataTypes>{

        pub fn element_ordering(a: DataTypes, b: DataTypes) -> Ordering {
            // Used for binary_search functions below
            a.cmp(&b)
        }

        pub fn element_matching(a: DataTypes, b: DataTypes) -> bool {
            // Function to match two values, used to search a value in the graph.
            // Currently set up to test equality, this could potentially
            // Be changed to another heuristic
            InvertedIndex::element_ordering(a, b).eq(&Ordering::Equal)
        }

        pub fn get_value_ids(&self, value: DataTypes) -> Option<&Vec<NodeId>> {
            // Return node ids whose nodes match a specific value
            if let Ok(value_id) = self.values.binary_search_by(|&probe| InvertedIndex::element_ordering(probe, value) ){
                self.node_ids.get(value_id)
            }else{
                None
            }
        }

        fn insert_value(&mut self, value: DataTypes, node_id: NodeId){
            // Check whether the value exists in the index
            // If the value already exists, just add the node_id
            // in the list of node_ids associated to this value
            if let Ok(value_id) = self.values.binary_search_by(|&probe| InvertedIndex::element_ordering(probe, value) ){
                self.node_ids[value_id].push(node_id)
            }else{
            // If it doesn't exist create a new entry for this value
                self.values.push(value);
                self.node_ids.push([node_id].to_vec());
            }
        }
    }

    use serde::{Serialize, Deserialize};
    use std::collections::HashMap;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct CPT<T> {
        // This class is the CPT: it consists of three data structures:
        // The inverted index is used to match node data values to node ids
        pub inverted_index: InvertedIndex<T>,
        // List of nodes
        pub nodes: Vec<Node<T>>,
        // The lookup table references the last node of each sequence
        pub sequences_lookup_table: Vec<NodeId>
    }
    impl<'a, T> Default for CPT<T> {
        fn default() -> Self {
            let mut nodes = Vec::new();
            nodes.push(Node {children: Vec::new(), parent: None, data: None});
            Self { nodes: nodes, inverted_index: InvertedIndex::new(), sequences_lookup_table: Vec::new() }
        }
    }
    impl<'a> CPT<DataTypes>{
        pub fn new() -> CPT<DataTypes> {
            Self::default()
        }

        pub fn to_json(&self) -> String {
            serde_json::to_string(self).unwrap()
        }

        pub fn to_json_pretty(&self) -> String {
            serde_json::to_string_pretty(self).unwrap()
        }

        pub fn to_dot(&self) -> String{
            // Write the CPT as a .dot file, Graphviz renders it pretty well
            let mut dot_string = String::from("digraph  Result { \n");
            dot_string.push_str("subgraph cluster_cpt {");

            for (id, node) in self.nodes.as_slice().iter().enumerate(){
                // Declare a node using
                // N0[label="Node 0"];
                // Declare an edge using
                // N0 -> N1[label=""];
                dot_string.push_str(&format!("{}[label=\"ID={:?}, {:?}\"];\n", id, id + 1, node.data));
                for child_id in node.children.as_slice(){
                    dot_string.push_str(&format!("{} -> {:?};\n", id, &child_id.index0()));
                }
            }
            dot_string.push_str("}\n");
            dot_string.push_str("subgraph cluster_seq {");

            for (id, last_node_id) in self.sequences_lookup_table.as_slice().iter().enumerate(){
                dot_string.push_str(&format!("seq{}[label=\"Seq {:?}\"; shape=\"rectangle\"];\n", id, id));
                dot_string.push_str(&format!("seq{} -> {:?};\n", id, last_node_id.index0()));
            }
            dot_string.push_str("}");
            dot_string.push_str("}");
            dot_string
        }

        pub fn get_root_id() -> NodeId{
            NodeId::new()
        }

        pub fn new_node(&mut self, new_node: Node<DataTypes>) -> NodeId{
            let next_index1 = NonZeroUsize::new(self.nodes.len().wrapping_add(1)).expect("Cannot access latest index");
            self.nodes.push(new_node);
            let new_node_id = NodeId::from_non_zero_usize(next_index1);
            self.inverted_index.insert_value(self.get_data(new_node_id).expect("Cannot insert data in inverted index"), new_node_id);
            new_node_id
        }

        pub fn get(&self, id: NodeId) -> Option<&Node<DataTypes>> {
            // println!("Getting node {:?}", id);
            // println!("-- Got node {:?}", self.nodes.get(id.index0()));
            self.nodes.get(id.index0())
        }

        pub fn update_node(& mut self, id: NodeId, new_node: Node<DataTypes>) {
            println!("Updating node {:?}:", id);
            println!("-- Before: {:?}", self.nodes.get(id.index0()));
            self.nodes[id.index0()] = new_node;
            println!("-- After: {:?}", self.nodes.get(id.index0()));
        }

        pub fn get_data(&self, id: NodeId) -> Option<DataTypes> {
            *self.get(id).expect(&format!("No node found for NodeId {:?}", id)).get()
        }

        pub fn child_exists(&self, new_data: DataTypes, node_id: NodeId) -> Option<NodeId>
            where DataTypes: PartialEq<DataTypes> + Copy {

            let mut matched_node_id = None;
            // println!("Does child with value {:?} exists for node {:?}", new_data, node_id);
            if let Some(parent_node) = self.get(node_id){
                // exists = parent_node.children.as_slice().iter().any(|&id| self.get_data(id) == Some(&new_data))
                // exists = parent_node.children.as_slice().iter().map(|&id| self.get(id)).any(|node| node.expect("").data == Some(new_data))
                for child_id in parent_node.children.as_slice(){
                    // println!("-- Looking for data in child {:?}", child_id);
                    if InvertedIndex::element_matching(self.get_data(*child_id).unwrap(), new_data){
                        matched_node_id = Some(child_id.clone());
                    }
                }
            }
            matched_node_id
        }

        pub fn add_child(&mut self, new_data: DataTypes, node_id: NodeId)-> NodeId where DataTypes: PartialEq<DataTypes> + Copy {
            println!("Adding value {:?} to CPT at node {:?}", new_data, node_id);
            match self.child_exists(new_data, node_id)  {
                // If no child exists with the current new data, create a new node
                None => {
                    let new_node = Node { data: Some(new_data), parent: Some(node_id), children: Vec::new() };
                    let new_node_id = self.new_node(new_node);

                    // Now update the parent node with a new child
                    if let Some(parent_node) = self.get(node_id){
                        let mut new_parent_node = parent_node.clone();
                        new_parent_node.children.push(new_node_id);
                        self.update_node(node_id, new_parent_node);
                    }
                    new_node_id
                },
                Some(matched_id) => {
                    // If a child already exists with the current new data,
                    // Don't create a new node: return the id of this child
                    matched_id
                }
            }
        }

        pub fn add_sequence_to_root(&mut self, sequence: &[DataTypes]) {
            self.add_sequence(sequence, CPT::get_root_id())
        }

        pub fn add_sequence(&mut self, sequence: &[DataTypes], node_id: NodeId) where DataTypes: PartialEq<DataTypes> + Copy {
            // "Training" of the tree: it adds each item of a sequence to the tree,
            // starting from the Node at node_id, 
            let mut current_node_id = node_id;
            for item in sequence{
                current_node_id = self.add_child(*item, current_node_id);
            }
            self.sequences_lookup_table.push(current_node_id);
            println!("Added sequence {:?} to node {:?}", sequence, node_id);
        }

        pub fn match_sequence(&self, sequence: &[DataTypes], backwards: bool) -> Vec<Vec<NodeId>> {
            // Given an input sequence, match the longest possible sequences in the CPT.
            // This is can be implemented in two ways:
            //  - starting from the first item of the sequence,
            //    and search forward in the tree,
            //  - start from the last item of the sequence and
            //    search backwards in the tree

            if backwards{
                self.match_sequence_backward(sequence)
            }else{
                self.match_sequence_forward(sequence)
            }
        }

        pub fn match_sequence_backward(&self, sequence: &[DataTypes]) -> Vec<Vec<NodeId>>{
            // This returns lists of NodeIDs for matched sequences
            let mut current_node_ids = Vec::<Vec<NodeId>>::new();
            println!("Matching sequence backwards {:?}", sequence);

            // Given an input sequence, get its last item
            // Then, check the parents with the previous value
            let mut new_sequence = sequence.to_vec();
            new_sequence.reverse();

            let mut sequence_iter = new_sequence.iter();

            let mut count = 1;

            // Get the last element of the sequence
            if let Some(last_value) = sequence_iter.next() {
                // Get all nodes matching this value, this is our initial list of possible nodes
                if let Some(possible_node_ids) = self.inverted_index.get_value_ids(*last_value){
                    current_node_ids = possible_node_ids.iter().map(|&e| vec![e]).collect();
                    // Get the previous item in the sequence to match,
                    // at each iteration we will filter the possible_node_ids
                    while let Some(&next_item) = sequence_iter.next(){
                        println!("Current NodeIds at item {:?}th item in sequence {:?}: {:?}", count, sequence, current_node_ids);
                        count = count + 1;
                        current_node_ids = current_node_ids.into_iter().map(|possible_node_ids|
                            if let Some(possible_node) = self.get(*possible_node_ids.first().unwrap()){
                                if let Some(parent_node_id) = possible_node.parent{
                                    if let Some(parent_node_data) = self.get_data(parent_node_id){
                                        if InvertedIndex::element_matching(parent_node_data, next_item) {
                                            [vec![parent_node_id], possible_node_ids].concat()
                                        } else { possible_node_ids }
                                    } else { possible_node_ids }
                                } else { possible_node_ids }
                            } else { possible_node_ids }
                        ).collect();
                    }
                }
            }
            println!("Matching node_ids at the end for sequence {:?}: {:?}", sequence, current_node_ids);
            current_node_ids
        }

        pub fn match_sequence_forward(&self, sequence: &[DataTypes]) -> Vec<Vec<NodeId>>{
            
            // This returns the last NodeID of the longest matched sequence
            let mut current_node_ids = Vec::<Vec<NodeId>>::new();
            println!("Matching sequence forward {:?}", sequence);

            // Given an input sequence, get its first item
            // Then, check the children with the next value
            let mut sequence_iter = sequence.iter();

            let mut count = 1;

            // Get the last element of the sequence
            if let Some(first_value) = sequence_iter.next() {
                // Get all nodes matching this value, this is our initial list of possible nodes
                if let Some(possible_node_ids) = self.inverted_index.get_value_ids(*first_value){
                    current_node_ids = possible_node_ids.iter().map(|&e| vec![e]).collect();

                    // Get the next item in the sequence to match,
                    // at each iteration we will filter the possible_node_ids
                    while let Some(&next_item) = sequence_iter.next(){
                        println!("Current NodeIds at item {:?}th item in sequence {:?}: {:?}", count, sequence, current_node_ids);
                        count = count + 1;
                        current_node_ids = current_node_ids.into_iter().filter_map(|possible_node_ids|
                                if let Some(possible_node) = self.get(*possible_node_ids.last().unwrap()){
                                    Some(possible_node.children.clone().into_iter().filter_map( |child_node_id| {
                                        if let Some(child_node_data) = self.get_data(child_node_id){
                                            if InvertedIndex::element_matching(child_node_data, next_item){
                                                Some([possible_node_ids.clone(), vec![child_node_id]].concat())
                                            } else { Some(possible_node_ids.clone()) }
                                        } else { None }
                                    }).collect::<Vec<Vec<NodeId>>>())
                                } else { None }
                        ).collect::<Vec<Vec<Vec<NodeId>>>>().concat();
                    }
                }
            }
            println!("Matching node_ids at the end for sequence {:?}: {:?}", sequence, current_node_ids);
            current_node_ids
        }

        pub fn predict(&self, sequence: &[DataTypes], prefix_length: usize) -> Vec<(DataTypes, usize, f32)>{
            // This is an implementation of the prediction algorithm implemented in
            // ADMA2013_Compact_Prediction_tree
            // The goal is to predict the next values of an input sequence.

            // Given an input sequence: xxyyy
            // And a predicted sequence: yyyzzz,
            // yyy is the "prefix", that will be used to predict the "consequent" zzz

            // The output is a sorted list of potential next items, with their two prediction score:
            // E.g [(Integer(1), 3, 3.0), ...]
            // The first step is to identify the unique value in our prefix,
            let prefix = &sequence[(sequence.len() - prefix_length)..sequence.len()].to_vec();
            let mut prefix_set = prefix.clone();
            prefix_set.sort();
            prefix_set.dedup();
            println!("Looking for sequences in the training set with the last {:?} values in {:?}", prefix_length, sequence);
            println!("Prediction prefix unique values: {:?}", prefix_set);

            // For each of the unique item in the prefix, get the ids of the sequence that contain them
            let all_match_sequence_ids: Vec<Vec<usize>> = prefix_set.iter().map(|prefix_value| {
                // We get the nodes that match these values
                if let Some(node_ids) = self.inverted_index.get_value_ids(*prefix_value){
                    // For each node matching a given value, we have to find
                    // What training sequences match these nodes
                    // We will descend the nodes children until we reach an end
                    let mut current_node_ids: Vec<NodeId> = node_ids.clone();
                    let mut leaf_node_ids: Vec<NodeId> =  Vec::<NodeId>::new();
                    println!("Prefix unique value {:?} matched with {:?} nodes", prefix_value, node_ids.len());
                    while current_node_ids.len() > 0{
                        leaf_node_ids.extend(current_node_ids.iter().filter(|&node_id| self.get(*node_id).unwrap().children.len() == 0));
                        current_node_ids = current_node_ids.clone().into_iter()
                                .filter(|&node_id| self.get(node_id).unwrap().children.len() > 0)
                                .map(|node_id| self.get(node_id).unwrap().children.clone() ).collect::<Vec<Vec<NodeId>>>().concat();
                    }
                    // Now let's lookup the training sequences that point to the leaf nodes
                    let match_sequence_ids: Vec<usize> = leaf_node_ids.iter().map(|leaf_node_id| self.sequences_lookup_table.binary_search(leaf_node_id).unwrap_or_else(|x| x) ).collect();
                    println!("Matching sequences for prefix value {:?}: {:?}", prefix_value, match_sequence_ids);
                    match_sequence_ids
                } else { vec![] }
            }).collect();

            // Now we have the list of sequences matching each element in the prefix, let's look at the unique sequence ids
            let mut unique_matched_sequence_ids = all_match_sequence_ids.concat();
            unique_matched_sequence_ids.sort();
            unique_matched_sequence_ids.dedup();

            // These unique sequence ids will be used to find the "consequent":
            // For each sequence, let's look at the last occurence of an item the sequence:
            // Given the input sequence xxyy with yy being the prefix, If the training Sequence aabbxxyz exists, the consequent returned is yz
            let mut count_consequent_values_support = HashMap::<DataTypes,usize>::new();
            unique_matched_sequence_ids.iter().for_each(|sequence_id| {
                if let Some(&last_node_id) = self.sequences_lookup_table.get(*sequence_id){
                    let mut current_node_id = last_node_id;
                    // let mut consequent: Vec<&Node<DataTypes>> = Vec::<&Node<DataTypes>>::new();
                    let mut consequent: Vec<NodeId> = Vec::<NodeId>::new();
                    while let Some(current_node) = self.get(current_node_id) {
                        // consequent.push(current_node);
                        if let Some(node_data) = current_node.data {

                            // Now check whether the current node data belongs to the input sequence
                            if sequence.iter().any(|&sequence_value| node_data == sequence_value ) {
                                break;
                            }
                            else{
                                consequent.push(current_node_id);
                                
                                // This will count the amount of each value in the consequents
                                *count_consequent_values_support.entry(node_data).or_insert(0) += 1;

                                // If not, retry the current node's parent
                                if let Some(current_node_parent) = current_node.parent{
                                    current_node_id = current_node_parent;
                                }
                                else{
                                    break;
                                }
                            }
                        }
                    }
                    // Consequent items have been pushed, need to reverse the list
                    consequent.reverse();
                    // The consequent given a training sequence is now:
                    // xyyyyy: x being the item that the training sequence and the input sequence have in common
                    println!("Consequent for input sequence {:?} given training sequence {:?}: {:?}", sequence, sequence_id, consequent.iter().map(|node_id| format!("{:?}, {:?}", node_id, self.get_data(*node_id))).collect::<Vec<String>>().join(" -> "));
                }
            });

            // The final step is to calculate the score of each consequent, using the following metrics:
            // Support:
            // The support is calculated for each individual value in our consequents.
            // It is the number of times a value appears in training sequences that matches our input sequence
            // In our case, it will be the unique count of values in the consequents, counted in the unique_matched_sequence_ids.
            
            // let mut count_matched_sequences_values_support = HashMap::<DataTypes,usize>::new();
            // unique_matched_sequence_ids.iter().for_each(|sequence_id| {
            //     if let Some(&last_node_id) = self.sequences_lookup_table.get(*sequence_id){
            //         let mut current_node_id = last_node_id;
            //         while let Some(current_node) = self.get(current_node_id) {
            //             if let Some(node_data) = current_node.data {
            //                 // Insert the value in the counting table
            //                 *count_matched_sequences_values_support.entry(node_data).or_insert(0) += 1;

            //                 // Set the current node to the current node's parent
            //                 if let Some(current_node_parent) = current_node.parent{
            //                     current_node_id = current_node_parent;
            //                 }
            //                 else{
            //                     break;
            //                 }
            //             }
            //             else{
            //                 break;
            //             }
            //         }
            //     }
            // });
            // We now have the count of consequent's unique values among matched training sequences
            println!("Count of consequent's unique values among consequents: {:?}: {:?}", unique_matched_sequence_ids, count_consequent_values_support);
        
        
            // The secondary metric is the confidence: for each item in the support counting hashmap,
            // we divide the support value by the number of time this item appears in the tree
            let mut count_consequent_values_support_confidence:  Vec<(DataTypes, usize, f32)> = count_consequent_values_support.iter().map(|(item, support)|{
                    (*item, *support, (*support as f32) / (self.inverted_index.get_value_ids(*item).unwrap().len() as f32))
                }).collect();

            // We now have the confidence value for each indivual item        
            // Finally we sort the values using the support and the confidence:
            count_consequent_values_support_confidence.sort_by(|a,b| {
                    if a.1 < b.1 {
                        Ordering::Less
                    }else {
                        if a.1 > b.1 {
                            Ordering::Greater
                        }
                        else{
                            if a.2 < b.2 {
                                Ordering::Less
                            }else{
                                Ordering::Greater
                            }
                        }
                    }
            });
            count_consequent_values_support_confidence.reverse();
            println!("Sorted list of predicted items and their scores: {:?}", count_consequent_values_support_confidence);
            count_consequent_values_support_confidence
        }
    }
}

pub mod nodes {
    use core::cmp::Ordering;
    use std::num::NonZeroUsize;
    use std::fmt::Display as FmtDisplay;
    use std::fmt::Formatter as FmtFormatter;
    use std::fmt::Result as FmtResult;

    use crate::data_types::DataTypes;
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, Debug)]
    #[derive(Copy, Clone)]
    #[derive(Eq)]
    pub struct NodeId {
        // Class used for NodeIDs, 1 indexing is the default
        // .index0 is implmented to get the 0 value
        pub index1: NonZeroUsize,
    }

   impl Ord for NodeId {
        fn cmp(&self, other: &Self) -> Ordering {
            self.index1.cmp(&other.index1)
        }
    }

    impl PartialOrd for NodeId {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }
    
    impl PartialEq for NodeId {
        fn eq(&self, other: &Self) -> bool {
            self.index1 == other.index1
        }
    }

    impl NodeId {

        pub fn new() -> NodeId {
            Self::default()
        }

        pub fn new_with_value(value: usize) -> NodeId {
            NodeId {index1: NonZeroUsize::new(value)
                .expect(&format!("Cannot create index with value {}", value))
            }
        }

        pub(crate) fn index0(self) -> usize {
            // This is totally safe because `self.index1 >= 1` is guaranteed by
            // `NonZeroUsize` type.
            self.index1.get() - 1
        }

        /// Creates a new `NodeId` from the given one-based index.
        pub(crate) fn from_non_zero_usize(index1: NonZeroUsize) -> Self {
            NodeId { index1 }
        }
    }

    impl Default for NodeId {
        fn default() -> Self {
            Self { index1: NonZeroUsize::new(1).expect("Cannot allocate uint for nodes indexing.")  }
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    #[derive(Clone)]
    pub struct Node<T> {
        // The node, that contains the id of its parent,
        // Th IDs of its children, 
        // And a node data
        pub parent: Option<NodeId>,
        pub children: Vec<NodeId>,
        pub data: Option<T>
    }

    impl Node<DataTypes> {
        /// Returns a reference to the node data.
        pub fn get(&self) -> &Option<DataTypes> {
            // println!("---- Accessing node data: {:?}", &self.data);
            &self.data
        }

        /// Returns a mutable reference to the node data.
        pub fn get_mut(&mut self) -> &mut Option<DataTypes> {
            &mut self.data
        }

        /// Creates a new `Node` with the default state and the given data.
        pub(crate) fn new(data: DataTypes) -> Self {
            Self {
                parent: None,
                children: Vec::new(),
                data: Some(data),
            }
        }

    }
    impl FmtDisplay for Node<i32> {
        fn fmt(&self, f: &mut FmtFormatter<'_>) -> FmtResult {
            if let Some(parent) = &self.parent {
                write!(f, "parent: {:?}; ", parent)?;
            } else {
                write!(f, "no parent; ")?;
            }
            Ok(())
        }
    }
}