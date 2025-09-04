use crate::doc::CodeBlock;
use crate::doc::TangleError;
use regex::Regex;
use std::collections::{HashMap, HashSet};

const MACROS_REGEX: &str = r"@\[([a-zA-Z0-9_]+)\]";
type Graph = HashMap<String, HashSet<String>>;

#[derive(PartialEq, Clone)]
enum State {
    NotVisited,
    Visiting,
    Visited,
}

// check_dependencies checks the dependencies of a target block
// Post:
// - If a block in the macro chain does not exist, it returns a BlockNotFound error
// - If there is a cycle in the dependencies, it returns a CycleDetected error
// - If everything is fine, it returns Ok(())
// - If there is an internal error, it returns InternalError
pub fn check_dependencies(
    target_block: &str,
    blocks: &HashMap<String, CodeBlock>,
) -> Result<(), TangleError> {
    // If it detects that a block named in a macro is missing, it returns a BlockNotFound error
    let graph = build_graph_from_target(target_block, blocks)?;

    // If it detects a cycle, it returns a CycleDetected error
    if has_cycle(&graph) {
        return Err(TangleError::CycleDetected());
    }

    // If everything is fine, it returns Ok(())
    Ok(())
}

/// build_graph_from_target Builds a dependency graph from the target block
/// and all available blocks.
/// # Post:
/// - If a block in the macro chain does not exist, it returns a BlockNotFound error
/// - If everything is fine, it returns the dependency graph
fn build_graph_from_target(
    target_block: &str,
    all_blocks: &HashMap<String, CodeBlock>,
) -> Result<Graph, TangleError> {
    let mut graph = Graph::new();
    let mut visited = HashSet::new();
    let regex = Regex::new(MACROS_REGEX)
        .map_err(|e| TangleError::InternalError(format!("Regex error: {}", e)))?;

    build_dependency_graph(target_block, all_blocks, &mut graph, &mut visited, &regex)?;

    Ok(graph)
}
/// build_dependency_graph Builds the dependency graph recursively.
/// # Post:
/// - If a block in the macro chain does not exist, it returns a BlockNotFound error
/// - If everything is fine, it returns the dependency graph
fn build_dependency_graph(
    current_block: &str,
    all_blocks: &HashMap<String, CodeBlock>,
    graph: &mut Graph,
    visited: &mut HashSet<String>,
    regex: &Regex,
) -> Result<(), TangleError> {
    // Check existence of the current block
    let block = all_blocks
        .get(current_block)
        .ok_or_else(|| TangleError::BlockNotFound(current_block.to_string()))?;

    // Check for multiple calls
    if visited.contains(current_block) {
        return Ok(());
    }
    visited.insert(current_block.to_string());

    // Add the node to the graph
    graph.entry(current_block.to_string()).or_default();

    for captures in regex.captures_iter(&block.code) {
        let macro_name = captures[1].to_string();

        // Insert the macro as an adjacent node
        graph
            .get_mut(current_block)
            .unwrap()
            .insert(macro_name.clone());

        build_dependency_graph(&macro_name, all_blocks, graph, visited, regex)?;
    }

    Ok(())
}

/// check_cycle_dfs Performs a depth-first search to detect cycles
/// # Post:
/// - If it finds a cycle, it returns true
/// - If it finds no cycles, it returns false
fn has_cycle(graph: &HashMap<String, HashSet<String>>) -> bool {
    let mut state = HashMap::new();

    for node in graph.keys() {
        if *state.get(node).unwrap_or(&State::NotVisited) == State::NotVisited
            && check_cycle_dfs(node, graph, &mut state)
        {
            return true;
        }
    }
    false
}

fn check_cycle_dfs(
    node: &String,
    graph: &HashMap<String, HashSet<String>>,
    state: &mut HashMap<String, State>,
) -> bool {
    state.insert(node.clone(), State::Visiting);

    for neighbor in graph.get(node).unwrap_or(&HashSet::new()) {
        let neighbor_state = state.get(neighbor).unwrap_or(&State::NotVisited).clone();

        match neighbor_state {
            State::Visiting => return true,
            State::NotVisited if check_cycle_dfs(neighbor, graph, state) => return true,
            _ => (),
        }
    }

    state.insert(node.clone(), State::Visited);
    false
}

///// Tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashMap, HashSet};

    fn set(items: &[&str]) -> HashSet<String> {
        items.iter().cloned().map(String::from).collect()
    }

    // Check Dependencies Tests
    #[test]
    fn infinite_cycle_in_dependencies_return_cycle_detected() {
        let mut blocks = HashMap::new();
        blocks.insert(
            "A".to_string(),
            CodeBlock::new_with_code("use @[B]".to_string()),
        );
        blocks.insert(
            "B".to_string(),
            CodeBlock::new_with_code("use @[A]".to_string()),
        );

        let mut one_block: HashMap<String, CodeBlock> = HashMap::new();
        one_block.insert(
            "A".to_string(),
            CodeBlock::new_with_code("use @[A]".to_string()),
        );

        let result = check_dependencies("A", &blocks);
        assert!(matches!(result, Err(TangleError::CycleDetected())));
        let result_one_block = check_dependencies("A", &one_block);
        assert!(matches!(
            result_one_block,
            Err(TangleError::CycleDetected())
        ));
    }

    #[test]
    fn missing_block_in_dependencies_return_block_not_found() {
        let mut blocks = HashMap::new();
        blocks.insert(
            "main".to_string(),
            CodeBlock::new_with_code("call @[missing]".to_string()),
        );

        let emptyblocks: HashMap<String, CodeBlock> = HashMap::new();

        let result = check_dependencies("main", &blocks);
        assert!(matches!(result, Err(TangleError::BlockNotFound(ref name)) if name == "missing"));

        let result = check_dependencies("missing", &emptyblocks);
        assert!(matches!(result, Err(TangleError::BlockNotFound(ref name)) if name == "missing"));
    }

    #[test]
    fn test_case_no_missing_dependencies_and_no_cycles() {
        let mut blocks = HashMap::new();
        blocks.insert(
            "main".to_string(),
            CodeBlock::new_with_code("start @[helper]".to_string()),
        );
        blocks.insert(
            "helper".to_string(),
            CodeBlock::new_with_code("let x = 42;".to_string()),
        );

        let result = check_dependencies("main", &blocks);
        assert!(result.is_ok());
    }

    // Test to verify that the graph is built correctly
    #[test]
    fn test_single_block_no_dependencies() {
        let mut blocks = HashMap::new();
        blocks.insert(
            "main".to_string(),
            CodeBlock::new_with_code("let x = 5;".to_string()),
        );

        let graph = build_graph_from_target("main", &blocks).unwrap();
        assert_eq!(graph.len(), 1);
        assert_eq!(graph["main"].len(), 0);
    }

    #[test]
    fn test_block_with_one_dependency() {
        let mut blocks = HashMap::new();
        blocks.insert(
            "main".to_string(),
            CodeBlock::new_with_code("start @[helper]".to_string()),
        );
        blocks.insert(
            "helper".to_string(),
            CodeBlock::new_with_code("let x = 42;".to_string()),
        );

        let graph = build_graph_from_target("main", &blocks).unwrap();

        let mut expected = HashMap::new();
        expected.insert("main".to_string(), HashSet::from(["helper".to_string()]));
        expected.insert("helper".to_string(), HashSet::new());

        assert_eq!(graph, expected);
    }

    #[test]
    fn test_nested_dependencies() {
        let mut blocks = HashMap::new();
        blocks.insert(
            "A".to_string(),
            CodeBlock::new_with_code("start @[B]".to_string()),
        );
        blocks.insert(
            "B".to_string(),
            CodeBlock::new_with_code("middle @[C]".to_string()),
        );
        blocks.insert("C".to_string(), CodeBlock::new_with_code("end".to_string()));

        let graph = build_graph_from_target("A", &blocks).unwrap();

        let mut expected = HashMap::new();
        expected.insert("A".to_string(), HashSet::from(["B".to_string()]));
        expected.insert("B".to_string(), HashSet::from(["C".to_string()]));
        expected.insert("C".to_string(), HashSet::new());

        assert_eq!(graph, expected);
    }

    #[test]
    fn test_missing_dependency_block() {
        let mut blocks = HashMap::new();
        blocks.insert(
            "main".to_string(),
            CodeBlock::new_with_code("call @[missing]".to_string()),
        );

        let result = build_graph_from_target("main", &blocks);

        assert!(matches!(result, Err(TangleError::BlockNotFound(ref name)) if name == "missing"));
    }

    #[test]
    fn test_cycle_handled_gracefully() {
        let mut blocks = HashMap::new();
        blocks.insert(
            "A".to_string(),
            CodeBlock::new_with_code("use @[B]".to_string()),
        );
        blocks.insert(
            "B".to_string(),
            CodeBlock::new_with_code("use @[A]".to_string()),
        );

        let graph = build_graph_from_target("A", &blocks).unwrap();

        assert_eq!(graph.len(), 2);
        assert_eq!(graph["A"], HashSet::from(["B".to_string()]));
        assert_eq!(graph["B"], HashSet::from(["A".to_string()]));
    }

    // Tests for cycle detection

    #[test]
    fn no_cycle_empty_graph() {
        let graph: HashMap<String, HashSet<String>> = HashMap::new();
        assert_eq!(has_cycle(&graph), false);
    }

    #[test]
    fn no_cycle_single_node() {
        let mut graph = HashMap::new();
        graph.insert("A".to_string(), HashSet::new());
        assert_eq!(has_cycle(&graph), false);
    }

    #[test]
    fn no_cycle_linear_graph() {
        let mut graph = HashMap::new();
        graph.insert("A".to_string(), set(&["B"]));
        graph.insert("B".to_string(), set(&["C"]));
        graph.insert("C".to_string(), HashSet::new());
        assert_eq!(has_cycle(&graph), false);
    }

    #[test]
    fn cycle_self_loop() {
        let mut graph = HashMap::new();
        graph.insert("A".to_string(), set(&["A"]));
        assert_eq!(has_cycle(&graph), true);
    }

    #[test]
    fn cycle_three_nodes() {
        let mut graph = HashMap::new();
        graph.insert("A".to_string(), set(&["B"]));
        graph.insert("B".to_string(), set(&["C"]));
        graph.insert("C".to_string(), set(&["A"]));
        assert_eq!(has_cycle(&graph), true);
    }

    #[test]
    fn disconnected_graph_with_one_cycle() {
        let mut graph = HashMap::new();
        graph.insert("A".to_string(), set(&["B"]));
        graph.insert("B".to_string(), set(&["C"]));
        graph.insert("C".to_string(), set(&["A"])); // cycle here

        graph.insert("X".to_string(), set(&["Y"]));
        graph.insert("Y".to_string(), HashSet::new());
        assert_eq!(has_cycle(&graph), true);
    }

    #[test]
    fn disconnected_graph_no_cycle() {
        let mut graph = HashMap::new();
        graph.insert("A".to_string(), set(&["B"]));
        graph.insert("B".to_string(), HashSet::new());

        graph.insert("X".to_string(), set(&["Y"]));
        graph.insert("Y".to_string(), HashSet::new());

        assert_eq!(has_cycle(&graph), false);
    }
}
