# e02 : Find unique items

## Challenge 

Write a function that filters duplicates within a list (Vec<i32>).

It will be a vector of 32 bit integers.

### Requirements

- Implements unique()

> It accepts a Vec<i32> as an argument, returning a Vec<i32> with no duplicates.

#### Example 1 

No duplicates in the input list.

```
  let list = vec![1,4,5];
  assert_eq!(unique(list), vec![1,4,5]);
```

#### Example 2

Duplicates in the input list

```
  let list = vec![1,1,3];
  assert_eq!(unique(list), vec![1,3]);
```

### Extra Credit

- Use Generics

> Change unique() to accept a Vec<T>, where T is a type that implements Ord.

- Retain order:

> Return elements in their original order




