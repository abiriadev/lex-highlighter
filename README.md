# Lex-highlighter

100% stream-based $\mathcal O(n)$ syntax highlighter for ANSI terminal

> [!WARNING]  
> This is proof-of-concept implementation and WON't be colorful like existing syntax highlighter solutions.

## Stream format

```bnf
<stream> ::= (<span> <newline>)*
<span> ::= <number> <whitespace>+ <number> (<whitespace>+ <color>)?
<number> ::= 0 | [1-9] [0-9]* // negative not supported
<color> ::= "#" <hex> <hex> <hex> <hex> <hex> <hex>
<hex> ::= [0-9a-fA-F]
```
