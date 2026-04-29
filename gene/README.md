# Gene

Gene generates Rust lexer source code from a compact lexer definition.

## Definition syntax

The lexer definition is line-oriented. Each rule ends with `;`.

- `let Name = r"...";` defines a reusable regex fragment.
- `rule Name = r"...";` defines a token rule.
- `skip rule Name = r"...";` defines a token rule that is skipped.
- Optional action blocks use a raw Rust string literal: `c#"..."#`.

Fragments can be referenced inside regexes as `{FragmentName}`.

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
skip rule Whitespace = r"[ \t\n\r]+";
let IdentStart = r"[a-zA-Z_]";
let IdentRest = r"[a-zA-Z0-9_]*";

rule LetKw = r"let";
rule Identifier = r"{IdentStart}{IdentRest}";
rule Number = r"[0-9]+";
rule Assign = r"=";
rule Semi = r";";
```

## API

```rust
use gene::generate_lexer_source;

let spec = r#"rule Number = r"[0-9]+";"#;
let source = generate_lexer_source(spec, None).expect("codegen failed");
```
