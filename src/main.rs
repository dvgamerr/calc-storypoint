fn calculate_priority(effort: &str, complexity: &str, risk: &str) -> i32 {
    let effort_values: f64 = match effort {
        "minimum" => 0.35,
        "mid" => 1.6,
        "moderate" => 2.85,
        "serve" => 4.1,
        "maximum" => 5.35,
        _ => 0.0,
    };

    let complexity_values: f64 = match complexity {
        "low" => 0.35,
        "medium" => 1.85,
        "high" => 3.35,
        _ => 0.0,
    };

    let risk_values: f64 = match risk {
        "none" => 0.35,
        "low" => 1.68,
        "moderate" => 3.0,
        "high" => 4.33,
        _ => 0.0,
    };

    let priority_score: f64 = effort_values + complexity_values + risk_values;

    let result_group: [i32; 6] = [1, 2, 3, 5, 8, 13];

    let closest_result: &i32 = result_group
        .iter()
        .min_by_key(|&&x| (x - priority_score as i32).abs())
        .unwrap_or(&1);

    *closest_result
}

fn main() {
    println!("ACP-1: {}", calculate_priority("minimum", "medium", "moderate"));
    println!("ACP-2: {}", calculate_priority("minimum", "medium", "low"));
    println!("ACP-4: {}", calculate_priority("minimum", "medium", "moderate"));
}