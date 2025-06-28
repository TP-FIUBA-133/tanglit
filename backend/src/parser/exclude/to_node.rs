use markdown::mdast::{Code, List, ListItem, Node, Paragraph, Text};

pub trait ToNode {
    fn to_node(self) -> Node;
}

impl ToNode for Paragraph {
    fn to_node(self) -> Node {
        Node::Paragraph(self)
    }
}
impl ToNode for Text {
    fn to_node(self) -> Node {
        Node::Text(self)
    }
}

impl ToNode for ListItem {
    fn to_node(self) -> Node {
        Node::ListItem(self)
    }
}

impl ToNode for List {
    fn to_node(self) -> Node {
        Node::List(self)
    }
}
impl ToNode for Code {
    fn to_node(self) -> Node {
        Node::Code(self)
    }
}
