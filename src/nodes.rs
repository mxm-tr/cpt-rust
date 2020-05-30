
pub mod nodes {
    use core::cmp::Ordering;
    use std::num::NonZeroUsize;
    use std::fmt::Display as FmtDisplay;
    use std::fmt::Formatter as FmtFormatter;
    use std::fmt::Result as FmtResult;

    use crate::data_types::data_types::DataTypes;
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, Debug)]
    #[derive(Copy, Clone)]
    #[derive(Eq, Hash)]
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