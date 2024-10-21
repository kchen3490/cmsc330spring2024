(* Review of Type Checking *)

(* lex => [1; +; 2; *; 3] //regex
    +
   / \
  1   2

two options:
  Given, if user_input > 1 then 1 else 1/"hello"
  while(true) {...} TypeError

  1) dynamic typing (type checking)
  runs until you see the error (Python, Ruby)
  - great for while loops

  
  Given, if 4 > 1 then 1 else 1/"hello"
  
  2) static typing
  type check before run (C, C++, Java, OCaml, Rust)

  sound: typecheck -> no runtime type error (i.e., if typecheck says it's correct, it's correct;
                                            e.g., if gcc is correct means your C program is correct)
    - used to define static type checking

  complete: rejects some valid programs
    - used to define dynamic type checking

  matrix
*)


(* Subtyping *)
(*
s <: t, if you want t, anything from s is okay
  student <: person,

  foo(p: person) {}

  foo(alice: student)

We use records to cover subtyping

refer to the rules on slides or handwritten notes
*)

(* Modules *)
(* Look at modules.md *)