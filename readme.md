# rebcc (REBASIC Compiler) 
A [BASIC](https://en.wikipedia.org/wiki/BASIC)-like programming language compiler that was made by following [Austin Z. Henley's Teeny Tiny compiler tutorial](https://austinhenley.com/blog/teenytinycompiler1.html) in Python, writing my own version in Rust.

## Why?
- For general knowledge of programming concepts.
- To learn more Rust.
- To know how does a compiler work on a deeper level.
- To learn how to properly document projects (to learn Markdown).
- To learn how to design proper tests.
- To learn the basics of git and its commands.
- To learn and practice Neovim.

## Example of REBASIC syntax
```
PRINT "How many fibonacci numbers do you want?"
INPUT nums

# This is a comment.

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

## How to use rebcc 
WIP

## Goals
- Functions.
- Variable type keywords 'INT', 'FLOAT', 'STRING', instead of the universal 'LET' keyword.
- Make an universal 'END' keyword to end any block of code.
