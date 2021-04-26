mod core_model {
    /// Main Graph Model
    pub struct Graph {
        name: String,
        nodes_collection: Vec<Node>,
        bonds_collection: Vec<Bond>
    }

    /// Main Node(Vertex) document collection element 
    pub struct Node {
        id: u32,
        label: String
        // TODO Create properties as JSON document
    }

    /// Main Bond(Relation) document collection element
    pub struct Bond {
        id: u32,
        label: String
    }


    //  Main Graph action Methods
    impl Graph {

        /// Creates new empty Graph
        fn new_graph(name: &str) -> Box<Self> {
            Box::new(Graph {name: String::from(name), 
                     nodes_collection: Vec::new(), 
                     bonds_collection: Vec::new()})
        }

        /// Creates Node, adding to nodes collection
        fn create_node(&mut self, node: Node) -> Result<(), ()> {
            self.nodes_collection.push(node);
            Ok(())
        }

        /// Creates Bond, adding to bonds collection
        fn create_bond(&mut self, bond: Bond) -> Result<(), ()> {
            self.bonds_collection.push(bond);
            Ok(())
        }

        // Drops Whole Graph
        fn delete_graph(self){
            drop(self);
        }
    }
}