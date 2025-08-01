use crate::{
    errors::TangleError,
    parser::code_block::{CodeBlock},
};
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

// check_dependencies verifica las dependencias de un bloque objetivo
// y devuelve un error si falta algún bloque nombrado en una macro o si hay un ciclo 
// en las dependencias.
// Si todo está bien, devuelve Ok(()).
pub fn check_dependencies(
    target_block: &str,
    blocks: &HashMap<String, CodeBlock>,
) -> Result<(), TangleError> {
    // Si detecta que falta un bloque nombrado en una macro, devuelve error BlockNotFound
    let graph = build_graph_from_target(target_block, blocks)?;

    // Si detecta ciclo, devuelve error CycleDetected
    has_cycle(&graph)?;

    // Si todo OK, devuelve unit ()
    Ok(())
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
    // 1. Verificar existencia del bloque actual
    let block = all_blocks
        .get(current_block)
        .ok_or_else(|| TangleError::BlockNotFound(current_block.to_string()))?;

    // 2. Evitar ciclos
    if visited.contains(current_block) {
        return Ok(());
    }
    visited.insert(current_block.to_string());

    // 3. Agregar el nodo al grafo, incluso si no tiene macros
    graph.entry(current_block.to_string()).or_default();

    // 4. Regex para extraer macros
    let regex = Regex::new(MACROS_REGEX)
        .map_err(|e| TangleError::InternalError(format!("Regex error: {}", e)))?;

    for captures in regex.captures_iter(&block.code) {
        let macro_name = captures[1].to_string();

        // 5. Insertar macro como dependencia
        graph
            .get_mut(current_block)
            .unwrap()
            .insert(macro_name.clone());

        // 6. Llamada recursiva (valida existencia)
        build_dependency_graph(&macro_name, all_blocks, graph, visited)?;
    }

    Ok(())
}





fn has_cycle(graph: &HashMap<String, HashSet<String>>) -> Result<(), TangleError> {
    let mut state = HashMap::new();

    for node in graph.keys() {
        if *state.get(node).unwrap_or(&State::NotVisited) == State::NotVisited {
            if check_cycle_dfs(node, graph, &mut state) {
                return Err(TangleError::CycleDetected());
            }
        }
    }
    Ok(())
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
/// 
/// #[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashMap, HashSet};

    fn set(items: &[&str]) -> HashSet<String> {
        items.iter().cloned().map(String::from).collect()
    }

    #[test]
    fn no_cycle_empty_graph() {
        let graph: HashMap<String, HashSet<String>> = HashMap::new();
        assert_eq!(has_cycle(&graph), Ok(()));
    }

    #[test]
    fn no_cycle_single_node() {
        let mut graph = HashMap::new();
        graph.insert("A".to_string(), HashSet::new());
        assert_eq!(has_cycle(&graph), Ok(()));
    }

    #[test]
    fn no_cycle_linear_graph() {
        let mut graph = HashMap::new();
        graph.insert("A".to_string(), set(&["B"]));
        graph.insert("B".to_string(), set(&["C"]));
        graph.insert("C".to_string(), HashSet::new());
        assert_eq!(has_cycle(&graph), Ok(()));
    }

    #[test]
    fn cycle_self_loop() {
        let mut graph = HashMap::new();
        graph.insert("A".to_string(), set(&["A"]));
        assert_eq!(has_cycle(&graph), Err(TangleError::CycleDetected()));
    }

    #[test]
    fn cycle_three_nodes() {
        let mut graph = HashMap::new();
        graph.insert("A".to_string(), set(&["B"]));
        graph.insert("B".to_string(), set(&["C"]));
        graph.insert("C".to_string(), set(&["A"]));
        assert_eq!(has_cycle(&graph), Err(TangleError::CycleDetected()));
    }

    #[test]
    fn disconnected_graph_with_one_cycle() {
        let mut graph = HashMap::new();
        graph.insert("A".to_string(), set(&["B"]));
        graph.insert("B".to_string(), set(&["C"]));
        graph.insert("C".to_string(), set(&["A"])); // ciclo aquí

        graph.insert("X".to_string(), set(&["Y"]));
        graph.insert("Y".to_string(), HashSet::new()); // sin ciclo

        assert_eq!(has_cycle(&graph), Err(TangleError::CycleDetected()));
    }

    #[test]
    fn disconnected_graph_no_cycle() {
        let mut graph = HashMap::new();
        graph.insert("A".to_string(), set(&["B"]));
        graph.insert("B".to_string(), HashSet::new());

        graph.insert("X".to_string(), set(&["Y"]));
        graph.insert("Y".to_string(), HashSet::new());

        assert_eq!(has_cycle(&graph), Ok(()));
    }
}
