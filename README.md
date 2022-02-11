# The Ram programming language
A stack based programming language created to experiment my crappy lang-dev only capable of making some mathematical formulation and logic. Written in the rust programming language.

## How to run
1. Download the [release binary](https://github.com/ujjwal-kr/ram/releases/download/v1.6/ram).
2. Make it executable using `chmod +x ./ram`
3. Make a .ram file with the following contents at the same directory as the binary:

```as
ram rv 1
add lx rv
ram lx prev
print lx
jmp 1

ram lx
ram 10
cmp
je 2
cmp
jne 0

printc >> loop end
halt
```

4. run `./ram ./example.ram` and press enter.

## Documentation
Official documentation for the RAM programming language. Please open an issue if you find any bugs, or want some features to be chaged or added.

The language has 6 variables `lx`, `rv` (for storing numbers), `string` and `lxstring` (for storing a string), `vec int` for storing a float vector and `vec str` for storing a string vector. All of these are global variavles. The data stored in them is preserved when a code block is switched. Details about how to use them is further in the documentation.

## Contents
- General commands:
  - [Comments](#comments)
  - [Print](#print)
  - [printc](#printc--characters)
  - [halt](#halt)
- std commands:
  - [stdin](#stdin)
  - [stdfs](#stdfs)
  - [random](#rand--num1num2)
- stack based commands:
  - [ram](#ram)
  - [vectors](#vectors)
  - [split](#split)
  - [str](#str)
  - [pop](#pop)
  - [popall](#popall)
- operations:  
  - [add](#add)
  - [subtract](#sub)
  - [multiply](#mul)
  - [division](#div)
  - [sqr](#sqr)
  - [sqrt](#sqrt)
  - [average](#avg)
  - [round](#round)

- [cmp and jump statements](#cmp-and-jump-statements)

## General Commands
### comments
`// this is a one line comment`

### print
- `print` prints the last number on the screen present in the stack.
- `print lx/rv/string/lxstring/vec str/vec int` prints the value of `lx/rv/string/lxstring/vec str/vec int` variables on the screen.

### printc >> characters
Prints the characters specified on the screen

### halt
`halt` stops the program

## std commands

### stdin
`stdin lx/rv/string` sets the value of the standard input provided in the next line to the specified variable.

### stdfs
- `stdfs open ./path/to/file` opens the file and stores the contents as strings in the  `strings` variable.

### rand >> num1,num2
`rand >> num1,num2/lx,rv/rv,lx` generates a random decimal between num1 and num2 (including both) and pushes it to stack. May `round` be used after this statement to round it to an integer.

## stack based commands

### ram
- `ram <number>` adds the number to the stack.
- `ram lx/rv <number>` assigns the number to the variable
- `ram lx/rv` adds the variable value to the stack for further calculations
- `ram lx/rv prev` assigns the last stack value to the specified variable.
- `ram string >> <strings>` assigns value to the strings global variable.

### vectors
- `ram vec int >> [1,2,3,4,ints...]` creates a vector and assigns it to the global `vec int` variable.
- `ram vec str >> [1,2,3,4,strs...]` creates a vector and assigns it to the global `vec str` variable.
- `vec str push` pushes the value of `string` to `vec str` variable.
- `vec int push lx/rv` pushes the value of `lx/rv` to `vec int` variable.
- `vec str >> [<index>/lx/rv]` extracts value of the index position of `vec str` and stores it in the `string` variable.
- `vec int lx/rv >> [<index>/lx/rv]` extracts the value of the index position of `vec int` and stores it in `lx/rv` variable.
- `vec str len` pushes the length of the `vec str` variable to the stack.
- `vec int len` pushes the length of the `vec int` variable to the stack.

### split
`split >> "<arg>"` splits the `string` variable into args and stores the values into the `vec str`

### str
- `str lxstring string` moves the value of the `string` variable to `lxstring`.
- `str string lxstring` moves the value of the `lxstring` variable to `string`.
- `str cmp` compares the variables `lxstring & strings` and pushes 0 to the stack if they are equal, and -1 if they are not.

### pop
Removes the last value from the stack

### popall
Empties the entire stack. Useful for `avg` implementation.

## Operation Commands

### add
- adds last two numbers in the stack and pushes the result to the stack
- `add lx rv` adds the two variables and pushes the result to the stack

### sub 
subtracts the last number from the number added prior to it and pushes to stack

### mul
- `mul` multiplies the last two numbers in the stack and adds the result to the stack
- `mul rv lx` multiplies lx and rv and pushes the result to the stack.

### div
the number added prior to the last number / last number and pushes to stack.

### sqr
- `sqr` squares the last number in the stack and adds the result to the stack.
- `sqr lx/rv` squares `lx/rv` and assigns the result to the variable squared.

### sqrt
- `sqrt` squares the last number in the stack and adds the result to the stack.
- `sqrt lx/rv` squares `lx/rv` and assigns the result to the variable squared.

### avg
Takes out the average of all the numbers present in the stack. `popall` should be used somewhere in the code to remove the vars which are redundant.

### round
- `round` rounds the last number present in the stack to decimals and pushes it to the stack.
- `round lx/rv` rounds the specified variable and assigns the result to the var itself.

### `cmp` and Jump statements
The program is devided into blocks separated by empty lines. And each block has an index. For example-
```as
ram 100
ram 300
cmp
jne 1
cmp
je 2

printc >> Not Equal

printc >> Equal
```

This code consists of three codeblocks, indexed 0,1,2 respectively and can be accessed by the jump statements.

- `jmp <index>` - jumps to the codeblock at the index position.
- `cmp` pushes -1, 0, 1 to the stack if the last two numbers are smaller, equal or greater than each other. (Important before jne and je commands).
- `jne <index>` - jumps to a block by its index position if the previous cmp statement is not 0 (equal)

- `je <index>` - jumps to a block by its index position if the previous cmp statement is 0 (equal)

- `jgr <index>` - jumps to a block by its index position if the prev cmp statement is 1 (greater)

- `jsm <index>` - jumps to a block by its index position if the prev cmp statement is -1 (smaller)

### Example Coin Flip

```as
printc >> Coin Flip
rand >> 0,1
round
ram 1
cmp
je 1
pop
ram 0
cmp
je 2

printc >> Heads

printc >> Tails
```
