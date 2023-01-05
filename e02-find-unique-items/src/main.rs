fn unique(mut list: Vec<i32>) -> Vec<i32>{
    list.sort();
    list.dedup();
    list
}


fn main() {
    let input: Vec<i32> = vec![2,1,1];
    let answer: Vec<i32> = unique(input);
    println!("unique -> {:?}", answer);
}

#[test]
fn empty_list() {
    let input = vec![];
    let expected_output = vec![];
    let actual_output = unique(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list() {
    let input = vec![1,4,5];
    let expected_output = vec![1,4,5];
    let actual_output = unique(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list_with_duplicates() {
    let input = vec![1,4,4,5];
    let expected_output = vec![1,4,5];
    let actual_output = unique(input);
    assert_eq!(actual_output, expected_output);
}


#[test]
fn unsorted_list() {
    let input = vec![1,5,4];
    let expected_output = vec![1,4,5];
    let actual_output = unique(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list_with_duplicates() {
    let input = vec![5,5,4,1];
    let expected_output = vec![1,4,5];
    let actual_output = unique(input);
    assert_eq!(actual_output, expected_output);
}
