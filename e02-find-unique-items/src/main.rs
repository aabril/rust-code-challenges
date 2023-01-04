fn unique(list: Vec<i32>) -> Vec<i32>{
    list
}


fn main() {
    let answer: Vec<i32> = unique(vec![1,4,5]);
    println!("Hello, world!");
    println!("unique([1,4,5]) = {:?}", answer);
}

#[test]
fn no_duplicates() {
    let input = vec![1,4,5];
    let expected_output = vec![1,4,5];
    let actual_output = unique(input);
    assert_eq!(actual_output, expected_output);
}
