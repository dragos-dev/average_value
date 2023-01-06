use std::collections::HashMap;

fn main() {
    let input_vec: Vec<usize> = vec![10, 20, 30, 40, 50, 50];
    let mut input_map: HashMap<usize, usize> = HashMap::new();
    let mut max: usize = 0;
    let mut sum_of_values: usize = 0;

    for i in input_vec.iter() {
        sum_of_values += *i;
        *input_map.entry(*i).or_insert(0) += 1;
    }

    for (_, value) in input_map.iter() {
        if *value > max {
            max = *value;
        }
    }

    let average_number = sum_of_values / &input_vec.len();

}
