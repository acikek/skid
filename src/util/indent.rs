pub fn indent(v: Vec<String>, spaces: usize) -> Vec<String> {
    let sp = " ".repeat(spaces);
    v.iter().map(|s| format!("{}{}", sp, s)).collect::<Vec<String>>()
}

pub fn indent_endl(v: Vec<String>, spaces: usize) -> String {
    indent(v, spaces).join("\n")
}