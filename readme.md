# REBASIC Compiler 
A [BASIC](https://en.wikipedia.org/wiki/BASIC)-like programming language compiler that was made by following [Austin Z. Henley's Teeny Tiny compiler tutorial](https://austinhenley.com/blog/teenytinycompiler1.html) in Python, writing my own version in Rust.

## Why?
- For general knowledge of programming concepts.
- To learn more Rust.
- To know how does a compiler work on a deeper level.
- To learn how to properly document projects (to learn markdown).
- To learn how to design tests for safe updates.

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

## How does it work?
### By these 3 stages of compilation:
1. **Lexical analysis (tokenization)** - the source code gets broken down into tokens;
2. **Syntax analysis (parsing)** - checking if the tokens are in correct order, rules of the language are being followed;
3. **Command emission** - passing commands to a lower-level programming language and running them.

## Features
### Compiler-wise
- A functional Lexer.

### Language-wise
No fully working compiler = no language features yet :(

## Goals
- Functions.
- Variable type keywords 'INT', 'FLOAT', 'STRING', instead of the universal 'LET' keyword.
- Make an universal 'END' keyword to end any block of code (could also do scopes with curly braces).
