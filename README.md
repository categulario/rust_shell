# Rust shell

Un sencillo shell escrito en rust. El parser y la gramática están escritos a mano haciendo uso extensivo de structs y enums para definir cada componente.

La gramática se parece a esto:

```
expr: semicolon_expr

semicolon_expr: and_expr ; semicolon_expr
              | and_expr

and_expr: or_expr && and_expr
        | or_expr

or_expr: call_expr || or_expr
       | call_expr

call_expr: cmd_name [args...]
         | ( expr )
         | Empty

args: Word
    | DoubleQuotedString
    | SingleQuotedString
```

## Desarrollo

Necesitas [rust](https://rustup.rs/), clona el repo y corre `cargo run` para probar el shell o `cargo test` para probar que de hecho hace lo que dice que hace.
