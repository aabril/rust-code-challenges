fn median(a: Vec<f32>) -> Option<f32> {
  todo!();
}

fn main() {
    let answer: Option<f32> = median(vec![1.0, 2.0, 5.0]);

    println!("median([1.0, 2.0, 5.0]) = {:?}", answer);
}

#[test]
fn empty_list() {
    let input: Vec<f32> = vec![];
    let expected_output: Option<f32> = None;
    let actual_output: Option<f32> = median(input);
    assert_eq!(actual_output, expected_output);
}


