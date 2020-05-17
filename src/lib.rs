use serde::{Serialize, Deserialize};

pub mod data_types {

    use std::cmp::PartialEq;
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, Debug)]
    #[derive(Copy, Clone, Eq, PartialOrd, PartialEq, Ord)]
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

        pub fn match_sequence(&self, sequence: &[DataTypes], backwards: bool) -> Vec<NodeId> {
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

        pub fn match_sequence_backward(&self, sequence: &[DataTypes]) -> Vec<NodeId>{
            // This returns the first NodeID of the longest matched sequence
            let mut current_node_ids = Vec::<NodeId>::new();
            let mut new_node_ids = Vec::<NodeId>::new();
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
                    current_node_ids = possible_node_ids.to_vec();
                    // Get the previous item in the sequence to match,
                    // at each iteration we will filter the possible_node_ids
                    while let Some(&next_item) = sequence_iter.next(){
                        println!("Current NodeIds at item {:?}th item in sequence {:?}: {:?}", count, sequence, current_node_ids);
                        count = count + 1;
                        new_node_ids = current_node_ids.iter().filter_map(|&possible_node_id|
                            if let Some(possible_node) = self.get(possible_node_id){
                                if let Some(parent_node_id) = possible_node.parent{
                                    if let Some(parent_node_data) = self.get_data(parent_node_id){
                                        if InvertedIndex::element_matching(parent_node_data, next_item) {
                                            Some(parent_node_id)
                                        }else { None }
                                    } else { None }
                                } else { None }
                            } else { None }
                        ).collect();
                        if new_node_ids.len() == 0{
                            break
                        }
                        current_node_ids = new_node_ids;
                    }
                }
            }
            println!("Matching node_ids at the end for sequence {:?}: {:?}", sequence, current_node_ids);
            current_node_ids
        }

        pub fn match_sequence_forward(&self, sequence: &[DataTypes]) -> Vec<NodeId>{
            // This returns the last NodeID of the longest matched sequence
            let mut current_node_ids = Vec::<NodeId>::new();
            let mut new_node_ids = Vec::<NodeId>::new();
            println!("Matching sequence forward {:?}", sequence);

            // Given an input sequence, get its first item
            // Then, check the children with the next value
            let mut sequence_iter = sequence.iter();

            let mut count = 1;

            // Get the last element of the sequence
            if let Some(first_value) = sequence_iter.next() {
                // Get all nodes matching this value, this is our initial list of possible nodes
                if let Some(possible_node_ids) = self.inverted_index.get_value_ids(*first_value){
                    current_node_ids = possible_node_ids.to_vec();
                    // Get the next item in the sequence to match,
                    // at each iteration we will filter the possible_node_ids
                    while let Some(&next_item) = sequence_iter.next(){
                        println!("Current NodeIds at item {:?}th item in sequence {:?}: {:?}", count, sequence, current_node_ids);
                        count = count + 1;
                        new_node_ids = current_node_ids.iter().filter_map(|&possible_node_id|
                                if let Some(possible_node) = self.get(possible_node_id){
                                    Some(possible_node.children.iter().filter_map( |&children_node_id| {
                                        if let Some(children_node_data) = self.get_data(children_node_id){
                                            if InvertedIndex::element_matching(children_node_data, next_item){
                                                Some(children_node_id)
                                            } else { None }
                                        } else { None }
                                    }).collect::<Vec<NodeId>>())
                                } else { None }
                        ).collect::<Vec<Vec<NodeId>>>().concat();
                        if new_node_ids.len() == 0{
                            break
                        }
                        current_node_ids = new_node_ids;
                    }
                }
            }
            println!("Matching node_ids at the end for sequence {:?}: {:?}", sequence, current_node_ids);
            current_node_ids
        }
    }

}

pub mod nodes {
    use std::num::NonZeroUsize;
    use std::fmt::Display as FmtDisplay;
    use std::fmt::Formatter as FmtFormatter;
    use std::fmt::Result as FmtResult;

    use crate::data_types::DataTypes;
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, Debug)]
    #[derive(Copy, Clone)]
    pub struct NodeId {
        // Class used for NodeIDs, 1 indexing is the default
        // .index0 is implmented to get the 0 value
        pub index1: NonZeroUsize,
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
            println!("---- Accessing node data: {:?}", &self.data);
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