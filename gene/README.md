# Gene

Gene generates Rust lexer source code from a compact lexer definition.

## Definition syntax

The lexer definition is line-oriented. Each rule ends with `;`.

- `%state path::To::State;` declares the lexer state type (optional, must appear first).
- `let Name = r"...";` defines a reusable regex fragment.
- `rule Name = r"...";` defines a token rule.
- `skip rule Name = r"...";` defines a token rule that is skipped.
- Optional callbacks use `=> path::to::handler` to call a Rust function.

Fragments can be referenced inside regexes as `{FragmentName}`.

Callback paths and `%state` must be absolute module paths in the form
`ident(::ident)+`.

## Regex grammar (summary)

- Literals match themselves; metacharacters must be escaped.
- `.` matches any Unicode scalar value except newline.
- `[...]` character classes support ranges, with `\-` and `\]` escaped.
- `^` at the start of a class negates it.
- `*`, `+`, `?` apply to the previous atom.
- `|` is alternation.
- `(...)` groups expressions.

## Example

```text
%state crate::lexer::State;
skip rule Whitespace = r"[ \t\n\r]+";
let IdentStart = r"[a-zA-Z_]";
let IdentRest = r"[a-zA-Z0-9_]*";

rule LetKw = r"let";
rule Identifier = r"{IdentStart}{IdentRest}" => crate::lexer::handle_ident;
rule Number = r"[0-9]+";
rule Assign = r"=";
rule Semi = r";";
```

## Callback signature

Callbacks receive the current slice, the remaining input, a mutable state, and
the token buffer. They return the token to emit and the number of extra bytes
consumed from `remainder`.

```rust
use std::collections::VecDeque;

fn handle(slice: &str, remainder: &str, state: &mut MyState, buffer: &mut VecDeque<Token>)
 -> Result<(Token, usize), ()> {
 let _ = (slice, remainder, state, buffer);
 Ok((Token::MyToken, 0))
}
```

## API

```rust
use gene::generate_lexer_source;

let spec = r#"rule Number = r"[0-9]+";"#;
let source = generate_lexer_source(spec, None).expect("codegen failed");
```

If `%state` is declared, the generated lexer exposes `Lexer::with_state` and
stores the provided state in `lexer.state`.
