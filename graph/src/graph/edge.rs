use std::collections::HashMap;
use std::fmt::{Display, Error, Formatter};

use crate::graph::placed_node::PlacedNode;
use crate::schema::property::Property;

#[derive(Debug, Clone)]
pub struct Hook {
    pub node: PlacedNode,
    pub property: Property,
}

#[derive(Debug, Clone)]
pub struct Edge {
    pub source: Hook,
    pub target: Hook,
}

#[derive(Debug, Clone)]
pub struct EdgeMap {
    edges: HashMap<String, Edge>,
}

//impl Hook {
//    pub fn new(node: &PlacedNode, property: &Property) -> Self {
//        return Hook {
//            node: node.into(),
//            property: property.clone(),
//        };
//    }
//}

impl Display for Hook {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}#{}", self.node.key, self.property.id())
    }
}

//impl Edge{
//    pub fn new(source: Hook, target: Hook)
//}

impl Display for Edge {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}>{}", self.source, self.target)
    }
}

impl EdgeMap {}

impl Default for EdgeMap {
    fn default() -> Self {
        EdgeMap {
            edges: Default::default(),
        }
    }
}
