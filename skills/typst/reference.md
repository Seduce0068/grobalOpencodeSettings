---
type: Reference
title: "Typst Reference"
description: "Complete Typst language reference and API documentation"
tags:
  - typst
  - reference
  - typesetting
generated:
  by: Sovereign AI Agent
  at: "2026-10-13T00:00:00+09:00"
---

# Typst Language Reference

## Syntax Overview

### Markup Mode (Default)

Text is markup by default. Special characters:
- `#` — Enter code mode
- `$` — Math mode
- `*` / `_` — Bold / Italic
- `` ` `` — Inline code
- `[...]` — Content block
- `=` — Heading (1-6 `=`)

### Code Mode

Started with `#` in markup:

```typst
#let x = 1          // Variable
#set text(size: 12pt)  // Set rule
#show heading: it => [#it]  // Show rule
#if cond [then] [else]  // Conditional
#for i in range(3) [#i]  // Loop
```

### Math Mode

Started with `$` (inline) or `$$` (block):

```typst
$ x + y $           // Inline
$$ sum_{i=1}^n i $$ // Block (numbered)
```

Unicode supported: `α β γ ∑ ∫ ∂ ≠ ≤ ≥ → ← ⇒ ⇐`

---

## Built-in Functions

### Text & Styling

| Function | Description |
|----------|-------------|
| `text(...)` | Create styled text |
| `strong(body)` | Bold |
| `emph(body)` | Italic |
| `underline(body)` | Underline |
| `strike(body)` | Strikethrough |
| `raw(text, lang: none)` | Code/monospace |
| `link(url, body)` | Hyperlink |
| `label(id)` | Anchor for refs |
| `ref(label)` | Cross-reference |
| `counter(name).display(fmt)` | Counter formatting |

### Layout

| Function | Description |
|----------|-------------|
| `page(...)` | Page setup |
| `column(body)` | Column break |
| `block(...)` | Block container |
| `box(...)` | Inline box |
| `align(alignment, body)` | Horizontal align |
| `place(alignment, body)` | Float placement |
| `grid(children, columns: n)` | Grid layout |
| `stack(...)` | Layer elements |

### Structure

| Function | Description |
|----------|-------------|
| `heading(level, body)` | Section heading |
| `outline(...)` | Table of contents |
| `figure(body, caption, ...)` | Figure with caption |
| `table(cells, columns: n)` | Table |
| `list(items, ...)` | Bullet/numbered list |
| `enum(items, ...)` | Enumerated list |
| `terms(items)` | Definition list |

### Math

| Function | Description |
|----------|-------------|
| `sym(name)` | Symbol by name |
| `op(name)` | Operator |
| `frac(num, den)` | Fraction |
| `root(radicand, index)` | Root |
| `attach(body, scripts...)` | Sub/superscript |
| `lr(left, body, right)` | Delimiters |
| `matrix(rows, cols, ...)` | Matrix |
| `cases(...)` | Piecewise |

---

## Set Rules (Global Configuration)

```typst
#set page(margin: 2cm, numbering: "1")
#set text(font: "Serif", size: 11pt, fill: black)
#set heading(numbering: "1.1", outlined: false)
#set link(fill: blue, underline: false)
#set par(justify: true, leading: 0.65em)
#set list(marker: "-")
#set enum(numbering: "1.")
#set figure(placement: "top")
#set table(stroke: 0.5pt)
```

### Page Options

```typst
#set page(
  paper: "a4",           // or "us-letter", "a5", etc.
  margin: (x: 2cm, y: 2.5cm),
  flip: true,            // Two-sided
  numbering: "1",        // "1", "i", "I", "a", "A"
  header: [Content],
  footer: [Content],
  columns: 1,
  column-gutter: 1cm,
)
```

---

## Show Rules (Transform Elements)

```typst
// Transform all headings
#show heading: it => [
  #set text(weight: "bold")
  #it
]

// Transform specific level
#show heading.where(level: 1): it => [
  #pagebreak()
  #set text(size: 24pt)
  #it
]

// Transform with metadata
#show figure: it => {
  set block(spacing: 0.5em)
  it
}

// Wrap all paragraphs
#show par: it => block(inset: 0, it)
```

---

## Standard Library Modules

### `sys` — System Info

```typst
#sys.inputs          // CLI inputs
#sys.cursor          // Current location
#sys.fonts           // Available fonts
```

### `datetime` — Date/Time

```typst
#datetime.today()                    // Current date
#datetime.date(year: 2026, month: 10, day: 13)
#datetime.time(hour: 14, minute: 30)
#dt.display("[year]/[month]/[day]")  // Format
#dt.year(), dt.month(), dt.day()
```

### `calc` — Calculations

```typst
#calc.add(1, 2)
#calc.mod(10, 3)
#calc.pow(2, 8)
#calc.sqrt(16)
#calc.sin(90deg)
```

### `array` / `dictionary` / `string`

```typst
#array.join(["a", "b"], ", ")
#dictionary.keys(dict)
#string.split("a,b,c", ",")
#string.replace("hello", "l", "x")
```

---

## Package Registry

### Official Preview Registry

Browse: <https://typst.app/universe>

### Common Packages

| Package | Purpose | Import |
|---------|---------|--------|
| `biblatex` | Citations/bibliography | `#import "@preview/biblatex:0.3.0"` |
| `cetz` | Diagrams/drawings | `#import "@preview/cetz:0.3.0"` |
| `touying` | Presentation slides | `#import "@preview/touying:0.3.0"` |
| `resume` | CV/resume templates | `#import "@preview/resume:0.2.0"` |
| `numbly` | Number formatting | `#import "@preview/numbly:0.1.0"` |
| `typst-assets` | Asset management | `#import "@preview/typst-assets:0.1.0"` |

### Using Packages

```typst
// In typst.toml
[dependencies]
biblatex = { version = "0.3.0", registry = "preview" }

// In .typ file
#import "@preview/biblatex:0.3.0": *

#show: cite.with(style: "apa", bibliography: bibliography("refs.bib"))

#cite("smith2024")
```

---

## CLI Reference

```bash
# Compile
typst compile [OPTIONS] INPUT [OUTPUT]
  --root DIR           # Project root
  --format pdf|png|svg|html
  --pages 1,3,5-10     # Page range
  --font-path DIR      # Additional fonts

# Watch (live reload)
typst watch INPUT [OUTPUT]

# Fonts
typst fonts            # List all
typst fonts --search "Noto"

# Package management
typst init [--package] # Initialize project

# Query
typst query INPUT SELECTOR
  # Example: typst query main.typ "heading"
```

---

## Debugging

### Reveal AST

```typst
#show: it => {
  reveal(it)  // Prints to stderr
  it
}
```

### Inspect Values

```typst
#let x = [Hello]
#show: it => {
  // In code mode, use `repr`
  #repr(x)  // Shows internal representation
  it
}
```

### Common Errors

| Error | Cause | Fix |
|-------|-------|-----|
| `unknown variable` | Typo, scope issue | Check spelling, `#let` before use |
| `expected content, found` | Type mismatch | Wrap in `[]` or use correct function |
| `cycle detected` | Recursive show rule | Avoid self-referential transforms |
| `font not found` | Missing font | Install font or use fallback |

---

## Performance Tips

1. **Use `#set` over `#show`** for simple styling
2. **Avoid deep nesting** in show rules
3. **Split large docs** with `#include`
4. **Cache images** — Typst caches automatically
5. **Use `typst watch`** for development

---

## Resources

- **Official docs**: <https://typst.app/docs/>
- **Reference**: <https://typst.app/docs/reference/>
- **Forum**: <https://github.com/typst/typst/discussions>
- **Packages**: <https://typst.app/universe/>
- **Web app**: <https://typst.app/app>
- **GitHub**: <https://github.com/typst/typst>
