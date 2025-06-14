pub fn tangle_blocks(blocks: Vec<String>) -> String {
    let mut tangle = String::new();
    for block in blocks {
        tangle.push_str(&block);
        tangle.push('\n');
    }
    tangle
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tangle_blocks() {
        let blocks = vec![
            "print('Hello, world!')".to_string(),
            "console.log('Hello, world!')".to_string(),
        ];
        let tangle = tangle_blocks(blocks);
        assert_eq!(
            tangle,
            "print('Hello, world!')\nconsole.log('Hello, world!')\n"
        );
    }

    #[test]
    fn test_tangle_blocks_empty() {
        let blocks: Vec<String> = vec![];
        let tangle = tangle_blocks(blocks);
        assert_eq!(tangle, "");
    }

    #[test]
    fn test_tangle_blocks_single() {
        let blocks = vec!["print('Hello, world!')".to_string()];
        let tangle = tangle_blocks(blocks);
        assert_eq!(tangle, "print('Hello, world!')\n");
    }
}
