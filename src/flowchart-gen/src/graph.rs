pub struct Node {
    name: String,
    code: String,
    id: String
}

pub struct Graph {
    nodes: Vec<Node>,
    current_id: usize,
    connection: Vec<(String, String)>,
}

impl Graph {
    pub fn new() -> Self {
        Self {nodes: Vec::new(), current_id: 0, connection: Vec::new()}
    }

    pub fn add_node(&mut self, name: &str, code: &str) -> String {
        let node = Node{name: name.to_string(), code: code.to_string(), id: Graph::node_id(self.current_id)};
        self.nodes.push(node);
        self.current_id += 1;
        self.nodes.last().unwrap().id.clone()
    }

    pub fn build(&self) -> String {
        let mut out = String::from("digraph {\n\trankdir=LR\n\tsplines=ortho\n\tnode [shape=none margin=0 fontname=monospace]\n\n");

        for node in &self.nodes {
            out.push_str(self.format_node(node).as_str());
        }

        for (from, to) in &self.connection {
            out.push_str(format!("\t{} -> {}\n", from, to).as_str());
        }

        out.push('}');
        out
    }

    pub fn connect_nodes(&mut self, a: &String, b: &String) {
        if !self.connection.contains(&(a.clone(), b.clone())) {
            self.connection.push((a.clone(), b.clone()));
        }
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    // ------

    fn format_node(&self, node: &Node) -> String {
        format!("\t{} [label=<
\t\t<table border=\"1\" cellborder=\"0\" cellspacing=\"0\" bgcolor=\"#2d2d2d\">
\t\t\t<tr><td bgcolor=\"#4a90d9\" align=\"left\"><font color=\"white\"><b>{}</b></font></td></tr>
\t\t\t<tr><td align=\"left\"><font color=\"white\">{}<br ALIGN=\"LEFT\"/></font></td></tr>
\t\t</table>
\t>]\n\n",
                node.id, node.name,
                node.code
                    .replace("&", "&amp;")
                    .replace("\n", "<br ALIGN=\"LEFT\"/>")
                    .replace("\t", "")
                    .replace("  ", "")
                    .replace("[", "&#91;")
                    .replace("]", "&#93;"))
    }

    pub fn node_id(n: usize) -> String {
        let mut n = n;
        let mut result = String::new();
        loop {
            result.push((b'A' + (n % 26) as u8) as char);
            n /= 26;
            if n == 0 { break; }
            n -= 1; // offset because there's no "zero" digit
        }
        result.chars().rev().collect()
    }

    pub fn id_to_node(s: &str) -> usize {
        s.chars().fold(0, |acc, c| {
            acc * 26 + (c as usize - 'A' as usize + 1)
        }) - 1
    }
}