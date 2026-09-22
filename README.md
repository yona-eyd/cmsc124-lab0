# Extension

## Creators

- Adrienne Nicole V. Tipon (strg-ze)
- Ryona Cassandra P. Honrado (rcssndr)

## Overview

[One paragraph: what the language is for, who would use it, what writing it
feels like.]

## Host language and build

- Host language: Rust 1.98.0
- Version metadata: [file that pins it, e.g. rust-toolchain.toml, go.mod]
- Build: `./build.sh`
- [Anything a fresh clone needs to know.]

## Running it


| Command | What it does |
|---|---|
| `./run <file>` | [Executes a program. Available from Lab 4.] |
| `./run --tokenize <file>` | [Prints the token stream.] |
| `./run --parse <file>` | [Prints the parsed tree.] |
| `./run --eval <file>` | [Evaluates each expression and prints its value.] |
| `./run` | [Starts the REPL.] |


Exit codes: 0 when file scans with no errors, 65 when scanner rejects an input, 70 [when].

## File extension

`.cord` [Must match the `ext` field in every tests/lab*/manifest.json.]

## Lexical structure

### Keywords


| Keyword | Purpose |
| [word] | [what does it do] |


### Operators


| Operator | Category |  Operands  | Associativity |             Precedence           |
|----------|----------|------------|---------------|----------------------------------|
|   `(`    | grouping |    none    |      none     | highest, overides all precedence |
|   `)`    | grouping |    none    |      none     | highest, overides all precedence |
|   `{`    |  block   |    none    |      none     |             none                 |
|   `}`    |  block   |    none    |      none     |             none                 |
|   `,`    |separator |    none    |      none     |             none                 |
|   `;`    |separator |    none    |      none     |             none                 |
|   `.`    |separator |    none    |      none     |             none                 |
|   `+`    |arithmetic|   binary   |      none     |             none                 |
|   `-`    |arithmetic|binary/unary|      none     |             none                 |
|   `*`    |arithmetic|   binary   |      none     |             none                 |
|   `/`    |arithmetic|    none    |      none     |             none                 |


### Literals


| Kind | Syntax | Produces |
|---|---|---|
| [number] | [e.g. 42, 3.14] | [what runtime value] |
| [string] | [e.g. "hello", escapes supported] | [what runtime value] |
| [boolean] | [true, false] | [what runtime value] |
| [nil] | [spelling] | [what runtime value] |


### Identifiers

- Start characters: [which]
- Continue characters: [which]
- Case-sensitive: [yes or no]
- [Reserved patterns, length limits, or other restrictions.]

### Comments

- Line comments: `//`, reads until end of line
- Block comments: `//* ... *//`
- Nesting: not supported
- Harness note: `comment_prefix` in `tests/lab*/manifest.json` is set to `//`

## Whitespace and termination

- Whitespace significant: no
- Statement terminator: semicolon (`;`)
- Block delimiters: braces (`{ }`)
- Grouping delimiters: parentheses (`( )`)

## Token output format

```
Token(type=VAR, lexeme=var, literal=null, line=1)
```

`type` is token category  (from Keywords/Operators/ Literals)
`lexeme` is raw source text
`literal` is the runtime value for literals only (`null if not applicable`)
`line` is 1-indexed

Frozen as of Lab 1; changes are recorded in the changelog.

## Grammar

```
[Your complete context-free grammar, current as of the latest activity.
Unambiguous, with precedence and associativity encoded in rule structure.]
```

## Parse output format

```
[one line of real --parse output, e.g. (+ 1.0 (* 2.0 3.0))]
```

- Groupings print as: [form]
- Numbers print as: [form]

## Semantics

### Values and types

[What runtime values exist, and how they are represented in the host
language.]

### Value printing

- Numbers: [e.g. 5 rather than 5.0]
- Nil: [spelling]
- Strings: [with or without quotes]

### Truthiness

[The complete rule. Which values are false in a condition; everything else is
true.]

### Operator semantics

- Arithmetic: [accepted operand types]
- `+` on strings: [concatenation, error, or coercion]
- Mixed types: [what happens]
- Comparison: [accepted operand types]
- Equality across types: [false, or an error]
- Division by zero: [value produced, or runtime error]

### Scope and bindings

- Redeclaration in the same scope: [allowed or an error]
- Uninitialized variable holds: [value]
- Shadowing: [behavior]
- Undefined name: [static error with exit 65, or runtime error with exit 70]

### Control flow and functions

- Logical operators return: [booleans, or the operand]
- Dangling else binds to: [which if]
- Closure capture of a loop variable: [per iteration, or shared]
- Function with no return statement produces: [value]
- Arity mismatch: [message and exit code]

## Native functions


| Name | Arguments | Returns | Notes |
|---|---|---|---|
| [name] | [count and types] | [type] | [caveats] |


## Errors and diagnostics

Message format:

```
[one real static error]
[one real runtime error]
```


| Failure | Exit code |
|---|---|
| [lexical error] | 65 |
| [syntax error] | 65 |
| [runtime error] | 70 |


## Testing conventions


| Folder | Activity | Mode | Flag |
|---|---|---|---|
| tests/lab1 | Scanner | sidecar | `--tokenize` |
| tests/lab2 | Parser | sidecar | `--parse` |
| tests/lab3 | Evaluator | inline | `--eval` |
| tests/lab4 | Context | inline | none |
| tests/lab5 | Functions | inline | none |


```
[specific tests]...
```

Run locally with:

```bash
curl -sSL https://raw.githubusercontent.com/WhiteLicorice/cmsc-124-harness/v1.1/run_tests.py -o run_tests.py
./build.sh
python3 run_tests.py tests/lab1
```

## Sample code

```
[a short program]
```

Output:

```
[its output]
```

## Design rationale

[Why the language is the way it is. Cover the choices that surprised you, the
features you cut, and the decisions you reversed. Specific reasons, not
approval of your own work.]

## Known limitations

1. String literals do not process escape sequences. They are taken literally, like any other character inside "".
2. Comments are line comments only. No block comments.
3. Empty input still emits a single Eof token at the very first line.
4. scan_tokens() collects error in the file before returning rather than stopping at the first one.
5. REPL scans one line at a time. A token split across multiple lines is read as undfinished and returns an error immediately.

## Changelog


| Activity | What changed in the language |
|---|---|
| Lab 1 | [entry] |