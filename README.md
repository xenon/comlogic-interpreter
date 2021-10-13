# Combinatory logic
- Combinatory logic is a "programming language".
- It's theoretically as powerful as any other programming language.
- It is a neat mathematical construction that captures some of the essence of what a program is.
- It creates fun and puzzling programs.

## What is combinatory logic? 
Combinatory logic is a simple pure-functional model of computation similar to the λ-calculus. It's major differentiating feature is that it does not have quantified variables.
It's basic "functions" are called combinators.

### Combinators
A combinator is n-ary, taking in n following terms and using them to produce a new combinatory logic expression, which is then substituted in place for it and the arguments.
If it doesn't have the n required terms to perform the replacement it is considered fully simplified and it's sub-expressions are checked for further simplification.

#### The three basic combinators
```
I x     => x
K x y   => x
S x y z => x z (y z)
```
- I, K and S are 1-, 2- and 3-ary functions respectively

## Grammar of combinatory logic
The grammar is provided here:
```
<outer-expr> ::= ε | <terms> | <cl-expr>
<cl-expr>    ::= '(' <terms> ')'
<terms>      ::= <atom/comb> | <cl-expr> | <atom/comb> <terms> | <cl-expr> <terms>
<atom/comb>  ::= <identifier>
```
*Note:*
- Spaces between terms are mandatory.
- ε is an empty string

### Non-terminals
- ``outer-expr`` is the starting point of the parser.
- ``cl-term`` is the recursive combinatory logic expression.
- ``terms`` is a non-empty list of combinators, atoms and subexpressions.
- ``atom/comb`` is either an atom or combinator.

#### When is it an atom and when is it a combinator?
An identifier is an atom by default, unless it is defined to be a combinator
in the interpreter's environment.

### Terminals
- `identifier` is a string of characters

### Why is the outer expression different?
Ideally all combinatory logic expressions could just be fully parenthesized but
it is convention that the outer expression does not have parentheses around it.

To achieve this one should remove the ``<terms>`` from the right-hand side of ``<outer-expr>``.

## About the implementation
- Combinatory logic is turing-complete but that doesn't mean that it's particularly useful.
- This implementation is the basic vanilla language and could be extended to be more useful.

### Numbers?
- Numbers could be implemented and arithmetic combinators could be made.
- This also leads into the idea of typing combinatory logic as once you do that you now would have both ``number`` and ``string`` atoms.

### IO?
- The implementation here is pure and doesn't provide any IO.
- IO can be added by having certain combinators perform side-effects.

# Compiling
## Interpreter/REPL
This is a terminal program that one can type combinatory logic expressions into.
```shell
cargo run --bin comlogic-repl
```
## Rust Library
```shell
cargo build --lib
```
## WASM
```shell
cargo build --lib --target=wasm32-unknown-unknown
```
### Note on wasm support
The wasm support is experimental, it requires that you have the ``wasm32-unknown-unknown`` target installed for rust. It also produces a large file because currently the combinators do allocations and use the standard library which adds overhead to the wasm file.

# License
- GPL v2
- See LICENSE for more information