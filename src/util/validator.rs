pub fn validated_keyword(input: String, prefix: &str) -> Result<Vec<String>, anyhow::Error> {
    let result: Vec<String> = input
        .split(',')
        .map(|s| s.trim())
        .filter(|&s| s.starts_with(prefix)) // ← Regex使わずにシンプルに
        .map(|s| s.to_string())
        .collect();

    Ok(result)
}
