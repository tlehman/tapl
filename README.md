# code for Pierce's TAPL (Types and Programming Languages)

# Chapter 3
Chapter 3 introduces a simple language called `arith` with nothing but booleans, zero and some simple conditional expressions.

The grammar for `arith` is:
```
t ::= 
  true
  false
  if t then t else t
  0
  succ t
  pred t
  iszero t
```

The set of all terms of `arith` is called `T`

The rules for constructing compound terms are:
1. {true, false, 0} &subseteq; `T`
1. if t<sub>1</sub> &in; `T`, then {succ t<sub>1</sub>, pred t<sub>1</sub>, iszero t<sub>1</sub>} &subseteq; `T`
1. if t<sub>1</sub>, t<sub>2</sub>, t<sub>3</sub> &in; `T` then 
'if t<sub>1</sub> then t<sub>2</sub> else t<sub>3</sub>' &in; `T`


# Chapter 4
In chapter 4, the author gives a concrete implementation of the `arith` language in OCaml. I am reading this book for two reasons, 1. to learn more about type systems in general, and 2. to learn Rust specifically by exploring its type system.

The author gives this OCaml type definition:
``` ocaml
type term = 
  TmTrue of info
| TmFalse of info
| TmIf of info * term * term * term
| TmZero of info
| TmSucc of info * term
| TmPred of info * term
| TmIsZero of info * term
```
Which I tried converting into Rust using this `enum`:

``` rust
#[derive(Debug)]  // equivalent to OCaml's "of info" annotation
enum Term {
  TmTrue,
  TmFalse,
  TmIf(Term, Term, Term),
  TmZero,
  TmSucc(Term),
  TmPred(Term),
  TmIsZero(Term)
}
```

Then when I compiled it, I got this _extremely_ helpful error message with an explainer page telling me that in order for the recursive type `Term` to compile, I need to use `&Term` with an explicit [lifetime](https://doc.rust-lang.org/1.9.0/book/lifetimes.html) or use a `Box<Term>` type, which represents a pointer to an object on the heap.

