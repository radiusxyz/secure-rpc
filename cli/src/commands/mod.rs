pub mod node;

use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Starts the node
    Node(Box<node::NodeCommand>),
}
