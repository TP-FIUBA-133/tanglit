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

// check_dependencies verifica las dependencias de un bloque objetivo
// Post:
// - Si un bloque en la cadena de macros no existe, devuelve un error BlockNotFound
// - Si hay un ciclo en las dependencias, devuelve un error CycleDetected
// - Si todo está bien, devuelve Ok(())
// - Si hay un error interno, devuelve InternalError
pub fn check_dependencies(
    target_block: &str,
    blocks: &HashMap<String, CodeBlock>,
) -> Result<(), TangleError> {
    println!("target_block: {}", target_block);
    // Si detecta que falta un bloque nombrado en una macro, devuelve error BlockNotFound
    let graph = build_graph_from_target(target_block, blocks)?;

    // Si detecta ciclo, devuelve error CycleDetected
    has_cycle(&graph)?;

    // Si todo OK, devuelve unit ()
    Ok(())
}

/// build_graph_from_target Construye un grafo de dependencias a partir del bloque objetivo
/// y todos los bloques disponibles.
/// # Post:
/// - Si un bloque en la cadena de macros no existe, devuelve un error BlockNotFound
/// - Si todo está bien, devuelve el grafo de dependencias
fn build_graph_from_target(
    target_block: &str,
    all_blocks: &HashMap<String, CodeBlock>,
) -> Result<Graph, TangleError> {
    let mut graph = Graph::new();
    let mut visited = HashSet::new();

    build_dependency_graph(target_block, all_blocks, &mut graph, &mut visited)?;

    Ok(graph)
}

/// build_dependency_graph Construye el grafo de dependencias de forma recursiva.
/// # Post:
/// - Si un bloque en la cadena de macros no existe, devuelve un error BlockNotFound
/// - Si todo está bien, devuelve el grafo de dependencias
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

// has_cycle verifica si hay ciclos en el grafo de dependencias
// # Post:
// - Si hay un ciclo, devuelve un error CycleDetected
// - Si no hay ciclos, devuelve Ok(())
fn has_cycle(graph: &HashMap<String, HashSet<String>>) -> Result<(), TangleError> {
    let mut state = HashMap::new();

    for node in graph.keys() {
        if *state.get(node).unwrap_or(&State::NotVisited) == State::NotVisited
            && check_cycle_dfs(node, graph, &mut state)
        {
            return Err(TangleError::CycleDetected());
        }
    }
    Ok(())
}

/// check_cycle_dfs Realiza una búsqueda en profundidad para detectar ciclos
/// # Post:
/// - Si encuentra un ciclo, devuelve true
/// - Si no encuentra ciclos, devuelve false
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

    // Test para verificar que se construya el grafo correctamente
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

    // Test for cycle detection

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
