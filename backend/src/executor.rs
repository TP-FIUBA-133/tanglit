use crate::parser::{code_block::CodeBlock, parse_input};
use std::process::Command;
use std::fs;

struct Executor {
    // Fields for the executor
}

impl Executor {
    pub fn execute_c(&self, blocks: &[&CodeBlock]) -> Result<String, String> {
        let code = blocks.iter()
            .map(|b| b.code.as_str())
            .collect::<Vec<&str>>()
            .join("\n");

        // Verificar si el código contiene una función main
        let has_main = code.contains("int main");

        // Si no contiene main, envolver el código en una función main
        let final_code = if has_main {
            code
        } else {
            format!(
                "#include <stdio.h>\n\nint main() {{\n{}\n    return 0;\n}}",
                code
            )
        };

        fs::write("temp.c", final_code).map_err(|e| e.to_string())?;
        // Crear un archivo para capturar la salida

        let compile = Command::new("gcc")
            .args(&["temp.c", "-o", "temp_run"])
            .status()
            .map_err(|e| e.to_string())?;
        if !compile.success() {
            return Err(format!("gcc falló con {:?}", compile.code()));
        }

        let output = Command::new("./temp_run")
            .output()
            .map_err(|e| e.to_string())?;
        
        fs::remove_file("temp.c").map_err(|e| e.to_string())?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        if output.status.success() {
            Ok(stdout)
        } else {
            Err(format!("Programa falló:\nSTDERR:\n{}", stderr))
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_executor_c_printf() {
        let executor = Executor {};
        let input = r#"
        ```c
        #include <stdio.h>
        int main() {
            printf("Hello World\n");
            return 0;
        }
        ```
        "#;
        let blocks = parse_input(input.to_string()).unwrap();
        let block = blocks.first().expect("No blocks found");
        let result = executor.execute_c(&[block]).expect("La ejecución falló");
        assert_eq!(result.trim(), "Hello World");
    }

    #[test]
    fn test_executor_c_without_main() {
        let executor = Executor {};
        let input = r#"
        ```c
        #include <stdio.h>
        printf("Hello World\n");
        ```
        "#;
        let blocks = parse_input(input.to_string()).unwrap();
        let block = blocks.first().expect("No blocks found");
        let result = executor.execute_c(&[block]).expect("La ejecución falló");
        assert_eq!(result.trim(), "Hello World");
    }
}