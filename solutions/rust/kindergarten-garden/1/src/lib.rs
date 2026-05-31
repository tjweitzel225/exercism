pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let student_idx = match student {
        "Alice" => 0,
        "Bob" => 2,
        "Charlie" => 4,
        "David" => 6,
        "Eve" => 8,
        "Fred" => 10,
        "Ginny" => 12,
        "Harriet" => 14,
        "Ileana" => 16,
        "Joseph" => 18,
        "Kincaid" => 20,
        "Larry" => 22,
        _ => unreachable!(),
    };
    diagram
        .lines()
        .flat_map(|row| row.get(student_idx..=student_idx + 1).unwrap().chars())
        .map(|c| match c {
            'G' => "grass",
            'C' => "clover",
            'R' => "radishes",
            'V' => "violets",
            _ => unreachable!(),
        })
        .collect()
}
