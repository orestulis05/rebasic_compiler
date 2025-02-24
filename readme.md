# REBASIC Compiler 
Made by following [Austin Z. Henley's Teeny Tiny compiler tutorial (web)](https://austinhenley.com/blog/teenytinycompiler1.html) in Python, writing my own version in Rust.

## Why?
The purpose of this project was to learn more of Rust, and of course - get to know how does a compiler work on a deeper level.

## Example of REBASIC syntax
```
PRINT "How many fibonacci numbers do you want?"
INPUT nums

LET a = 0
LET b = 1
LET nums > 0 DO 
    PRINT a
    LET c = a + b
    LET a = b
    LET b = c
    LET nums = nums - 1
ENDWHILE
```

## Features
WIP

## How does it work?
### By these 3 stages of compilation:
1. Lexical analysis (tokenization) - the source code gets broken down into tokens;
2. Syntax analysis (parsing) - checking if the tokens are in correct order, rules of the language are being followed;
3. Command emission - passing commands to a lower-level programming language and running them.

## What did I learn?
- How Lexer works and that making a working Lexer is an annoying part of building a compiler (at least for me, a poor junior programmer).
