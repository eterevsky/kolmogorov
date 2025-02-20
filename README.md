# Exploring Kolmogorov complexity

Kolmogorov complexity is the amount of information in a given object, measured as the length of a minimal program that could generate it. This project aims to compute the Kolmogorov complexity of numbers and sequences by generating them with BrainFuck programs, Turing machines and Lambda calculus.

Two kinds of output are generated: natural numbers and finite bit sequences. The bit sequences could be interpreted as numbers in a binary representation, however different outputs can have different number of leading zeros which can be significant.

## Generating a number

In the following methods generate a single number as an output.

### Binary Brainfuck with a counter

Brainfuck program on a binary tape. The permitted operations include `+`, `<`, `>`, `[]` and `.`. The output is a counter that starts at 0 and is incremented by 1 upon every `.` operation (regardless of the currently observed bit). The values upon program completion is the output.

Possible errors:

* `<` when the program is at the leftmost cell of the tape.
* Exceeding the step limit.

### Arbitrary precision BrainFuck

BrainFuck program operating on a tape of arbitrary precision non-negative integers. The permitted operations include `+`, `-`, `<`, `>` and `[]`. The output of the program is the number in the cell 0 after the program has finished.

Possible errors:

* `<` when the program is at the leftmost cell of the tape.
* `-` on the cell containing 0.
* Exceeding the step limit.

### Turing machine with a counter

A Turing machine on a binary tape infinite in both directions. Each transition rule has an extra boolean specifying whether to increment the output counter.

Possible errors:

* Exceeding the step limit.

### Lambda calculus

The program is a lambda term without free variables. To evaluate, we append a single `1` to it and reduce it to full normal form. The valid program should output a string of 1s. The number of 1s is the output.

Possible errors:

* The output is not a pure string of 1s.
* The limit to the number of reduction steps is exceeded.

### SK calculus

The program is a binary tree with [S and K operators](https://en.wikipedia.org/wiki/SKI_combinator_calculus) in the leaves (specified using parentheses). To evaluate, we apply the expression to a single `1` and fully reduce the expression. The order of operations in the output should be trivial, i.e. (((1 1) 1) 1) ...

Possible errors:

* The output is not a pure string of 1s.
* The limit to the number of reduction steps is exceeded.

## Generation methods: a sequence of bits

The following methods output a finite string of bits.

### Binary Brainfuck without output

Brainfuck program on a binary tape. The permitted operations include `+`, `<`, `>` and `[]`. The output is the state of the tape between the beginning and the final position of the head.

Possible errors:

* `<` when the program is at the leftmost cell of the tape.
* Exceeding the step limit.

### Binary Brainfuck with output

Brainfuck program on a binary tape. The permitted operations include `+`, `<`, `>`, `[]` and `.`. The `.` operation outputs the current bit to the output sequence.

Possible errors:

* `<` when the program is at the leftmost cell of the tape.
* Exceeding the step limit.

### Turing machine

A Turing machine is run on a binary tape, infinite in one direction. The output is the final state of the tape from the final position of the head to the right.

Possible errors:

* Moving to the left of the leftmost position in the tape.
* Exceeding the step limit.

### 2-tape Turing machine

A Turing machine on a binary tape infinite in both directions, with an extra ouput tape. Each transition rule contains an extra value that is written to the second tape, which could be 0, 1 or none (no output value).

Possible errors:
* Exceeding the step limit.

### Lambda calculus

The program is a lambda term without free variables. To evaluate, we append two free variables `10` to it and reduce it to full normal form. The valid program should output a string of 1s and 0s which is the output.

Possible errors:

* The output is not a pure string of 0s and 1s.
* The limit to the number of reduction steps is exceeded.

### SK calculus

Applies S and K transformations from [combinatory logic](https://en.wikipedia.org/wiki/Combinatory_logic). The program is a sequence of Ss and Ks followed by terms `01`. The program is valid if after expanding the rules it leaves only 0s and 1s. The order of operations in the output should be trivial, i.e. (((x1 x2) x3) x4) ...

Possible errors:

* The output is not a pure string of 0s and 1s.
* The limit to the number of reduction steps is exceeded.

## What are we measuring?

For each computing system we can measure a few things:

1. How many of the valid programs result in a valid output? We can measure it as `log2(number of valid programs) - log2(number of programs with valid output)` for programs below certain length.

2. How many of the program with valid output result in _unique_ output. This can be calculated as `log2(number of programs with valid output) - log2(number of different outputs)` for programs below certain length.

If we arrange all possible outputs of a given computation system in the order in which they appear when we iterate through possible programs, we'll define the index of a certain output in this sequence as the index of this output w.r.t. this system. `log2` of this index can be considered its Kolmogorov complexity. We are interested in the following questions:

1. How far apart can be indices of a single output w.r.t. different computation systems?

2. For an arbitrary number/sequence output, how much higher will `log2(index)` be than `log2(number)` or the length of sequence?

3. What are the outputs that have unusually low `log2(index)` compared to the `log2(number)` or sequence length?