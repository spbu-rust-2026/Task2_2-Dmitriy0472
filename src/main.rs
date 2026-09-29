fn main() {
    print!("{}", process(String::from("hello MATMEX")));
}

fn process(_text: String) -> String {
    let otv1: String = match _text.trim_start().find(char::is_whitespace) {
        Some(a) => _text.trim_start()[..a].to_string(),
        None => _text.trim_start().to_string(),
    };

    let mut otv2 = _text;
    otv2.push_str(" rust");

    let otv3 = &otv2;

    format!("{}\n{}\n{}\n", otv1, otv2, otv3)
}

#[cfg(test)]
#[path = "../tests/unit/process.rs"]
mod process_tests;
