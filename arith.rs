#[derive(Debug)]  // equivalent to OCaml's "of info" annotation
enum Term {
  TmTrue,
  TmFalse,
  TmIf(Box<Term>, Box<Term>, Box<Term>),
  TmZero,
  TmSucc(Box<Term>),
  TmPred(Box<Term>),
  TmIsZero(Box<Term>)
}

fn main() {
  let tm_zero = Term::TmZero;
  let tm_true = Term::TmTrue;
  let tm_false = Term::TmFalse;
  let tm_if = Term::TmIf(Box::new(Term::TmTrue),
                         Box::new(Term::TmTrue),
                         Box::new(Term::TmFalse));

  let tm_iszero = Term::TmIsZero(Box::new(Term::TmZero));

  println!("{:?}", tm_if);
}