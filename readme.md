# REBASIC Compiler 
Made by following [Austin Z. Henley's Teeny Tiny compiler tutorial (web)](https://austinhenley.com/blog/teenytinycompiler1.html)  

## Why?
The purpose of this project was to learn more Rust, data structures, algorithms and of course - get to know how does a compiler work on a deeper level.

## Example of REBASIC syntax
```
print "How many fibonacci numbers do you want?"
scan nums

var a = 0
var b = 1
while nums > 0 do 
    print a
    var c = a + b
    var a = b
    var b = c
    var nums = nums - 1
end
```

## How does it work?
### Stages of compiling 
REBASIC compiler processes REBASIC source code in 3 stages:
1. Lexical analysis (tokenization) - the source code gets broken down into tokens;
2. Syntax analysis (parsing) - checking if the tokens are in correct order, rules of REBASIC are being followed;
3. Command emission - emitting code in a lower-level programming language.

### Key words
WIP

### Syntax rules
WIP

## What did I learn?
WIP
