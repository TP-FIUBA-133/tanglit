use crate::{
    errors::TangleError,
    parser::code_block::{CodeBlock, Language},
};
use regex::Regex;
use std::collections::{HashMap, HashSet};

const MACROS_REGEX: &str = r"@\[([a-zA-Z0-9_]+)\]";
type Graph = HashMap<String, HashSet<String>>;

pub fn check_dependencies( target_block: &str, blocks: HashMap<String, CodeBlock>) {
    let blocks_graph = 
        build_graph_from_target(target_block, &blocks);
}

fn build_graph_from_target(
    target_block: &str,
    all_blocks: &HashMap<String, CodeBlock>,
) -> Result<Graph, TangleError> {
    let mut graph = Graph::new();
    let mut visited = HashSet::new();

    build_dependency_graph(target_block, all_blocks, &mut graph, &mut visited)?;

    Ok(graph)
}

fn build_dependency_graph(
    current_block: &str,
    all_blocks: &HashMap<String, CodeBlock>,
    graph: &mut Graph,
    visited: &mut HashSet<String>,
) -> Result<(), TangleError> {
    if visited.contains(current_block) {
        return Ok(());
    }
    visited.insert(current_block.to_string());

    let block = all_blocks
        .get(current_block)
        .ok_or_else(|| TangleError::BlockNotFound(current_block.to_string()))?;

    let regex = Regex::new(MACROS_REGEX)
        .map_err(|e| TangleError::InternalError(format!("Regex error: {}", e)))?;

    for captures in regex.captures_iter(&block.code) {
        let macro_name = captures[1].to_string();

        if !all_blocks.contains_key(&macro_name) {
            return Err(TangleError::BlockNotFound(macro_name));
        }

        graph
            .entry(current_block.to_string())
            .or_default()
            .insert(macro_name.clone());

        build_dependency_graph(&macro_name, all_blocks, graph, visited)?;
    }

    Ok(())
}