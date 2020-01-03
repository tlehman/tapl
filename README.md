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
In chapter 4, the author gives a concrete implementation of the `arith` language.
