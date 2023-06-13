use camino::Utf8PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Canvas {
    pub nodes: Vec<Node>,
    pub edges: Vec<Edge>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum Node {
    File {
        file: Utf8PathBuf,
        id: NodeId,
    },
    Group {
        id: NodeId,
        label: String,
        #[serde(flatten)]
        residue: serde_json::Value,
    },
    Text {
        id: NodeId,
        #[serde(flatten)]
        residue: serde_json::Value,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Edge {
    pub from_node: NodeId,
    pub from_side: Side,
    pub to_node: NodeId,
    pub to_side: Side,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct NodeId(String);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Side {
    Bottom,
    Top,
    Left,
    Right,
}

impl Node {
    pub fn id(&self) -> NodeId {
        match self {
            Node::File { id, .. } | Node::Group { id, .. } | Node::Text { id, .. } => id.clone(),
        }
    }
}
