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

    use std::num::NonZeroUsize;
    use crate::data_types::DataTypes as DataTypes;
    use crate::nodes::{Node, NodeId};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct InvertedIndex<T>{
        values: Vec<T>,
        node_ids: Vec<Vec<NodeId>>,
    }
    impl<T> InvertedIndex<T>{
        pub fn new() -> InvertedIndex<T> {
            InvertedIndex { 
                values: Vec::<T>::new(),
                node_ids: Vec::<Vec<NodeId>>::new(),
            }
        }
    }
    impl InvertedIndex<DataTypes>{
        pub fn get_value_ids(&self, value: DataTypes) -> Option<&Vec<NodeId>> {
            if let Ok(value_id) = self.values.binary_search(&value){
                self.node_ids.get(value_id)
            }else{
                None
            }
        }

        fn insert_value(&mut self, value: DataTypes, node_id: NodeId){
            // Check whether the value exists in the index
            // If the value already exists, just add the node_id
            // in the list of node_ids associated to this value
            if let Ok(value_id) = self.values.binary_search(&value){
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
        pub inverted_index: InvertedIndex<T>,
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
            let mut dot_string = String::from("digraph  Result { \n");
            dot_string.push_str("subgraph cluster_cpt {");

            for (id, node) in self.nodes.as_slice().iter().enumerate(){
                // Declare a node using
                // N0[label="Node 0"];
                // Declare an edge using
                // N0 -> N1[label=""];
                
                dot_string.push_str(&format!("{}[label=\"{:?}\"];\n", id, node.data));
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
            println!("Getting node {:?}", id);
            println!("-- Got node {:?}", self.nodes.get(id.index0()));
            self.nodes.get(id.index0())
        }

        pub fn update_node(& mut self, id: NodeId, new_node: Node<DataTypes>) {
            println!("Updating node {:?}:", id);
            println!("-- Before: {:?}", self.nodes.get(id.index0()));
            self.nodes[id.index0()] = new_node;
            println!("-- After: {:?}", self.nodes.get(id.index0()));
        }

        pub fn get_data(&self, id: NodeId) -> Option<DataTypes> {
            *self.get(id).expect("No node found").get()
        }

        pub fn child_exists(&self, new_data: DataTypes, node_id: NodeId) -> Option<NodeId>
            where DataTypes: PartialEq<DataTypes> + Copy {

            let mut matched_node_id = None;
            println!("Does child with value {:?} exists for node {:?}", new_data, node_id);
            if let Some(parent_node) = self.get(node_id){
                // exists = parent_node.children.as_slice().iter().any(|&id| self.get_data(id) == Some(&new_data))
                // exists = parent_node.children.as_slice().iter().map(|&id| self.get(id)).any(|node| node.expect("").data == Some(new_data))
                for child_id in parent_node.children.as_slice(){
                    println!("-- Looking for data in child {:?}", child_id);
                    if self.get_data(*child_id) == Some(new_data){
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
        
            let mut current_node_id = node_id;
            for item in sequence{
                current_node_id = self.add_child(*item, current_node_id);
            }
            self.sequences_lookup_table.push(current_node_id);
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