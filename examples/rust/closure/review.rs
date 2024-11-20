/* 11/20/2024 
In OCaml,

let free_var t = ...

let n = |x:u32|->u32 {
    body
    free var: copy, move, ref
}

fn n2(x:u32)->u32 {
    body
    cannot reference free varaible
}
*/