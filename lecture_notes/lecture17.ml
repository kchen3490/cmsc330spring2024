(* Operational Semantics *)
(* semantics means the "what does the code mean?" *)


type id = string
type num = int
type exp = 
| Ident of id
| Num of num (* num is int *)
| Plus of exp * exp
| Let of id * exp * exp

(* from OCaml through lexer, parser, and interpreter, to its result *)

let p1 = Num 10;;
(* "10" => lexer => parser == Num 10 => interpreter yields -: 10 *)

let p2 = Plus(Num 10, Num 20);;
(* "10 + 20" => lexer => parser => Plus(Num 10, Num 20) => interpreter yields -: 30 *)

let p3 = Let("x", p2, Plus(Num 1, Num 2));;
(* "let x = 10 + 20 in 1 + 2" ==> lexer => parser => 
  Let("x", p2, Plus(Num 1, Num 2)) => interpreter yields 3 *)


let x = 10 in (let x = 20 in x) + x;; (* inside x shadows the outer scope one *)
(* same as below *)
let p5 = Let("x", Num 10, Plus(Let("x", 20, Ident "x")), x);;

(*
(* substitute x in e2 with e1 *)
let rec subst x e1 e2 =
  match e2 with
  | Ident y -> if x=y then e1 else e2
  | Num c -> Num c
  | Plus(e1, er) -> Plus(subst x e1 el, subst x e1 er)
  | Let(y, ebind, ebody) ->
    if x=y then Let(y, subst x e1 ebind, ebody)
    else 
;;
*)

(* OCaml version of eval for Num and Sum *)
let rec eval exp = 
  match exp with 
    (* Num *)
    | Num n -> num

    (* Sum *)
    | Plus(e1, e2) ->
      let n1 = eval e1 in
      let n2 = eval e2 in 
      let n3 = n1 + n2
      in n3
    | Let (x, e1, e2) ->
      let e2' = subst x e1 e2 in
      let v2 = eval e2' in v2
    | _ -> failwith "error 1"
;;

(* Operations on Environment *)
type env = (id * value) list;;

list extend env x v = (x,v)::env;; (* adds another item onto environment *)

let rec lookup env x = 
  match env with
  [] -> failwith "no var"
  | (y,v)::env ->
    if x = y then v
    else lookup env' x
;;

let rec eval env e =
  match e with
    Ident x -> lookup env x
  | Num n -> n
  | Plus (e1,e2) ->
      let n1 = eval env e1 in
      let n2 = eval env e2 in
      let n3 = n1+n2 in
     n3
  | Let (x,e1,e2) ->
      let v1 = eval env e1 in
      let env’ = extend env x v1 in
      let v2 = eval env’ e2 in v2
  | Eq0 e1 -> 
      let Int n = eval env e1 in
      if n=0 then Bool true else Bool false
  | If (e1,e2,e3) ->
      let Bool b = eval env e1 in
      if b then eval env e2
      else eval env e3
;;

type value = 
    Int of int
  | Bool of bool

type exp = 
  | Ident of id 
  | Val of value
  | Plus of exp * exp
  | Let of id * exp * exp
  | Eq0 of exp
  | If of exp * exp * exp

(* Examples w/ ops on env *)

(* env: x = 100 
20 + x
*)
eval [("x", Int 100)] (Plus);;

(* "if 4 + 2 = 0 then 100 else 200" *)
eval [] (If(Eq0 (Plus(Val(Int 4), Val(Int 2))), Val(Int 100), Val(Int 200)));;

(* "if 1 + 2 > 3 then 2 + 3 else 3 + 5" *)
let e1 = If(Gt(Plus(Val(Int 1), Val(Int 2)), Val(Int 3)), Val(Int 5), Val(Int 8));;
eval [] e1;;