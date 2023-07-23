# Maple

The most epic programming language ever even tho it might be slow

## Design

Maple is an interpreted language bc I don't want to deal with compiling and all that stuff

It has strong typing (kinda) and might have "Metaprogramming" in the future

## Well Tested

Maple has been tested not only on ubuntu wsl x64, but also on windows 11 x64. This clearly means that it will run on any other system. 

It compiles on the latest version of clang (built from the source, it's like clang 15 or something) and msvc; maybe it will also compile with gcc

## Syntax

Maple's syntax is very nice, being inspied by what was easiest when i wrote each component into the lexer and parser

### Variables

Variables are declared with the syntax *type* *name* [= *value*]

very simple, here's 2 examples:

```
int x = 0

char y
```

as you can see maple also has no semicolons

### Functions

Functions are very easily declared with the syntax: fn *name* ([args[, ]...]) *return_type* { [code] }

like this:

```
fn returnAnInt(int a) int {
    return a + 2
}
```

### if

If statements need no parentheses after the if:

```
if x == 0 {
    // do stuff
}
```

same with while

else if is done with the keyword elseif (ie no space), and else is done with else:

```
if x == 1 {
    // do stuff
} elseif x > 1 {
    // do other stuff
} else {
    // do other other stuff
}
```

### Operators

the currently used operators and their precedence can be found in Maple/AST.cpp (aka not all operators are actually implemented yet)

## Future plans

### "Metaprogramming"

"Metaprogramming" will be done with the '@' token prefixing an expression

What this will do is force the codeblock to not create a new scope, thus enabling conditional code creation:

```
@if x == 0 {
    fn doYeet(char x) char {
        return x + 1
    }
else {
    fn doYeet(char x) char {
        return x - 1
    }
}
```

### mega speed boost

1. maybe have all operators have their own ast which also means more customizability but more work for me
2. idk make things faster
3. custom token thing
4. mega super mega speed boost

### idk other stuff
  
