# Lambda lang


## Ways to note a lambda expression

- Simple identity function
```
\x. x
\x y. y x
```

- Syntax with parentheses
- Note the dot after the parentheses
```
(x y). y x
```

- Optional commas between arguments
```
(x, y). y x
```

- Named lambda expression
- No space before opening parenthesis
```
foo(x y). y x
```


## Goals


- (.) Reverse application function: `(.) :: a -> (a -> b) -> b
