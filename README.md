


## Declaration

### python
```python
number: int = 10
text: str = "Text"
(first, second) = (1, 2)
```

### rust
```rust
let number: i32 = 10;
let text: &str = "Text";
let (first, second) = (1, 2);
```

## Mutability

`Python` has mutable values by default, but in `rust`, we must specify that this variable is mutable using the `mut` keyword.

### python
```python
number: int = 10
number += 1
```

```rust
let mut number: i32 = 10;
number += 1;
```

## Common types

```python
number: int = 10
string: str = "Text"
boolean: bool = True
tup: tuple[int, int] = (10, 10)
```

```rust
let number: int = 10;
let string: &str = "Text";
let boolean: bool = true;
let tup: (i32, i32) = (10, 10);
```

## Functions

```python
def fib(n: int) -> int:
    if n < 2: 
        return n
    return fib(n-1) + fib(n-2)
```

```rust
fn fib(n: i32) -> i32 {
    if n < 2 {
        return n;
    }
    return fib(n-1) + fib(n-2);
}
```

## if

``` python
if True:
    print("True")
else:
    print("False")
```

``` rust
if true {
    print!("True\n");
}
else {
    print!("False\n");
}
```

## For

``` python
for i in range(5, 10):
    print("In loop")
```

``` rust
for i in 5..10 {
    print!("In loop")
}
```

or
``` rust
for i in (std::ops::Range{start: 3, end:5}) {
    print!("Test\n");
}
```

## While

``` python
while x < 10:
    x+=1
```

``` rust
while x < 10 {
    x+=1;
}
```
