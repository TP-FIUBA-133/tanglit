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
            if dfs(node, graph, &mut state) {
                return Err(TangleError::CycleDetected());
            }
        }
    }
    Ok(())
}

fn dfs(
    node: &String,
    graph: &HashMap<String, HashSet<String>>,
    state: &mut HashMap<String, State>,
) -> bool {
    state.insert(node.clone(), State::Visiting);

    for neighbor in graph.get(node).unwrap_or(&HashSet::new()) {
        let neighbor_state = state.get(neighbor).unwrap_or(&State::NotVisited).clone();

        match neighbor_state {
            State::Visiting => return true,
            State::NotVisited if dfs(neighbor, graph, state) => return true,
            _ => (),
        }
    }

    state.insert(node.clone(), State::Visited);
    false
}
