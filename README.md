# Exploring numbers, strings and sequences with low Kolmogorov complexity

Kolmogorov complexity is the amount of information in a given object, measured as the length of a minimal program that could generate it. This project aims to compute the Kolmogorov complexity of numbers and sequences by generating them with BrainFuck programs, Turing machines and SKI calculus.

## Generation methods

We are going to generate non-negative integer numbers, or equivalently bit strings. Several methods are used.

### Binary BrainFuck

BrainFuck program working on a binary tape. + and - are equivalent. `.` operation outputs the current bit. `,` operation is not permitted. The output bit string is interpreted as a number with the lower magnitude digits first (otherwise the number would always have to start with a 1).

### Arbitrary precision BrainFuck

BrainFuck program operating on a tape of arbitrary precision integers. Input/output operations are not permitted. The output of the program is the number in the cell 0 after the program has finished. Unlike other options, the output in this mode can be negative.

### Turing machine

A Turing machine with a binary tape. The output is the final state of the tape from the final position of the head to the right.

### 2-tape Turing machine

A Turing machine on a binary tape with an extra ouput tape. Each transition rule contains an extra value that is written to the second tape, one of 0, 1 and none.

### SK(I) combinatory logic

Applies S and K transformations from [combinatory logic](https://en.wikipedia.org/wiki/Combinatory_logic). The program is a sequence of Ss and Ks followed by terms `01`. The program is valid if after expanding the rules it leaves only 0s and 1s.

## Measuring the complexity

We can measure the complexity of the number (or bitstring) that we generate in one of a few ways. We will assume that all of the programs in a given computation system are ordered in some fixed order such that the shorter programs always come before the longer ones.

We can measure the complexity of the given output by:

1. The index of the minimal program generating the given output among all the valid programs.
2. The index of the given output among all the possible outputs in order in which they are generated.