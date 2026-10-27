# XML tag checker

## What I changed

I implemented the tag validation logic and connected it to the command-line
application. The program reads the first argument and prints either `Valid` or
`Invalid` followed by a newline.

I kept the solution focused on the two rules in the assignment. It is not
intended to behave like a full XML parser, and it does not use regular
expressions or an XML library.

## Approach

The input is scanned from left to right. When the program finds `<`, it looks
for the next `>` and uses the text between them as the tag value.

I used a `Vec<&str>` as a stack for open tags:

- An opening tag is added to the stack.
- A closing tag must match the last tag in the stack.
- A missing `>`, an empty tag, or a closing tag that does not match immediately
  makes the input invalid.
- After the scan, the input is valid only when the stack is empty.

Using a stack also handles nesting naturally because the most recently opened
tag must be the first one closed.

## Assumptions

Some inputs are not defined explicitly in the assignment, so I used the
following rules:

- Tag contents are compared exactly. This includes capitalization, whitespace,
  and attribute-like text. For example, `<x a="1"></x a="1">` is valid, while
  `<x a="1"></x>` is not.
- Self-closing tags, comments, declarations, and CDATA do not receive special
  XML handling. For example, `<x/>` is treated as an opening tag named `x/` and
  is invalid unless it has an identical closing tag.
- Text outside tags is ignored.
- Empty input, plain text, and multiple top-level tag pairs are considered valid
  because they do not break either of the two validation rules.
- If no command-line argument is provided, the program prints `Invalid`.

## Complexity

The input is scanned once, so the time complexity is `O(n)`. The stack uses
`O(d)` additional space, where `d` is the maximum nesting depth.

## Running the project

```sh
cargo run -- '<Design><Code>hello world</Code></Design>'
```

Expected output:

```text
Valid
```

## Tests and checks

The tests cover the examples from the assignment, nested and sibling tags,
exact tag matching, malformed tags, missing arguments, and command-line output.

```sh
cargo test
cargo clippy --all-targets --all-features -- -D warnings
cargo fmt --check
```

## Build

`cargo build --release` creates `checker` on Unix-like systems and
`checker.exe` on Windows in `target/release/`.