# Exercise 01

Write a function to calculate the median of a list of numbers
(Vec<f32>)

We'll go through 3 examples:

### Example 1:
Input has an odd number of elements

Let's assume we have an variable list that is a vector of three numbers and we want to calculate its median.

```
let list = vec![1.0, 4.0, 5.0];
assert_eq!(median(list), ?);
```

To do so, we sort the list, in case it is not sorted, and take it's middle value.
It's four, and then so our result is four.

```
assert_eq!(median(list), Some(4));
```

The word some in the return value is significant.
This will enable us to account for the case of an empty list.


### Example 2:

On our example 2 is similar, but it's when we have an even number of elements.
In this case, our vector has four element in it.

```
let list = vec![3.0, 1.5, 8.8, 5.0];
assert_eq!(median(list), ?);
```

We first sort them, then take the average of the middle two values. ( 3.0 and 5.0 , which is 4.0 ).
That's four, and so our return value is some of four.


```
assert_eq!(median(list), Some(4.0));
```

### Example 3: 

The empty case is a littlbe bit simpler, 
```
let list = vec![1.0, 4.0, 5.0];
assert_eq!(median(list), ?);
```

and then we return None

```
assert_eq!(median(list), None);
```

---



