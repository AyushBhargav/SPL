# SPL
SPL (Simple Pseudo Language) is an interpreted language which is to be used in algorithm development and stuff like that. Final design draft of language is not decided yet, so drastic changes will be made in subsequencial releases. Use it at your risk.

## Language Features
### General Syntax and semantics
Language will support dynamic typing and syntactically will be very similar to the likes of JavaScript and Python but with in-built support for data structures like trees and graphs. Language should look like the general algorithm pseudocode. N
```
x <- 0
factorial <- 1
while x < 5 {
    factorial <- factorial * x
}
print factorial
```
### In-Built Data Structures
#### Arrays
Arrays are mutable by default.
```
arr <- [1, 2, 3]
arr <- arr + [4, 5] // [1, 2, 3, 4, 5]
```
#### Hash
```
hash = Hash()
hash["key1"] = 1
```