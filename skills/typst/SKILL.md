---
name: typst
description: Create, compile, and manage Typst documents. Use when writing technical documents, reports, slides, papers, or any typeset content. Typst is a modern Rust-based typesetting system that replaces LaTeX with faster compilation, markdown-like syntax, and programmable control flow.
---

# Typst Skill

## Quick Start

### Minimal Document

```typst
#set page(margin: 2cm)
#set text(font: "DejaVu Serif", size: 11pt)

= Title

Content here.

$ E = mc^2 $
```

### Compile

```bash
typst compile main.typ output.pdf      # Single build
typst watch main.typ                   # Live reload on save
typst fonts                            # List available fonts
```

---

## Project Structure

```
project/
├── main.typ           # Entry point
├── template.typ       # Shared styles (import as #import "template.typ" as t)
├── chapters/          # Per-chapter files
│   └── 01_intro.typ
├── assets/            # Images, data files
├── bibliography.bib   # BibTeX references (optional)
└── typst.toml         # Package dependencies (optional)
```

### main.typ Template

```typst
#import "template.typ" as t

#let meta = (
  title: "Document Title",
  author: "Author Name",
  date: datetime.today().display("[year]/[month]/[day]"),
  version: "0.1.0",
)

#show: t.document.with(meta)

= Introduction
#include "chapters/01_intro.typ"

= Main Content
#include "chapters/02_body.typ"

#t.appendix()
```

### template.typ Template

```typst
#let page-margin = 2.5cm
#let font-main = "Noto Serif JP"
#let font-mono = "Noto Sans Mono JP"
#let color-primary = rgb("#1a1a2e")
#let color-link = rgb("#0066cc")

#set page(margin: page-margin, numbering: "1 / {total}")
#set text(font: font-main, size: 11pt, fill: color-primary)
#set heading(numbering: "1.1")
#set link(fill: color-link, underline: false)

#let document(meta, body) = {
  // Cover page
  {
    set align(center)
    set text(size: 28pt, weight: "bold")
    v(4cm)
    meta.title
    v(1cm)
    set text(size: 14pt)
    meta.author
    v(0.5cm)
    meta.date
    pagebreak()
  }
  // TOC
  { outline(title: [目次]); pagebreak() }
  // Body
  body
}

#let appendix() = {}
#let note(body) = block(fill: rgb("#f0f4ff"), inset: 1em, radius: 4pt, width: 100%, body)
#let warn(body) = block(fill: rgb("#fff4e6"), inset: 1em, radius: 4pt, width: 100%, [⚠ #body])
#let code-block(lang, body) = raw(block: true, lang: lang, body)
```

---

## Common Patterns

### Headings & Structure

```typst
= Level 1 (Chapter)
== Level 2 (Section)
=== Level 3 (Subsection)
```

### Lists

```typst
- Bullet item
- Another item

1. Numbered
2. Items

+ Term
  Definition
```

### Math

```typst
// Inline
$ x = 1 $

// Block (numbered)
$$ sum_{i=1}^n i = n(n+1)/2 $$

// Unicode direct input
$ α + β = γ $
```

### Figures & Tables

```typst
// Image
#figure(
  image("assets/diagram.png", width: 80%),
  caption: [Architecture diagram],
)

// Table
#table(
  columns: 3,
  [Header 1], [Header 2], [Header 3],
  [Cell], [Cell], [Cell],
)
```

### Code Blocks

```typst
// Inline
`code`

// Block with syntax highlighting
```rust
fn main() {
    println!("Hello");
}
```

// Using helper from template
#code-block("rust", "
fn main() {
    println!(\"Hello\");
}
")
```

### Citations (with biblatex)

```typst
#import "@preview/biblatex:0.3.0": *

#show: cite.with(
  style: "apa",
  bibliography: bibliography("bibliography.bib"),
)

#cite("key")          // Parenthetical
#cite(key, form: "narrative")  // Narrative
```

---

## Functions & Control Flow

```typst
// Define function
#let boxed(title, body) = block(
  fill: rgb("#f5f5f5"),
  stroke: 1pt + rgb("#ccc"),
  radius: 4pt,
  inset: 1em,
  [#strong(title)] #par(body)
)

// Conditional
#if condition [true branch] [false branch]

// Loop
#for i in range(3) [
  Item #(i + 1)
]

// Variable assignment
#let x = 10
#show: set text(size: x * 1.2pt)
```

---

## Package Management

```toml
# typst.toml
[package]
name = "my-doc"
version = "0.1.0"

[dependencies]
biblatex = { version = "0.3.0", registry = "preview" }
cetz = { version = "0.3.0", registry = "preview" }  # Diagrams
```

```bash
typst init --package  # Download deps
```

Popular packages: `biblatex` (citations), `cetz` (diagrams), `touying` (slides), `resume` (CVs)

---

## VS Code Setup

Install **Tinymist Typst** extension.

```json
// .vscode/settings.json
{
  "tinymist.enableDocumentCodeLens": true,
  "tinymist.exportPdf": "onSave",
  "[typst]": { "editor.defaultFormatter": "myriad-dreamin.tinymist" }
}
```

---

## CI/CD (GitHub Actions)

```yaml
# .github/workflows/typst.yml
name: Build PDF
on: [push, pull_request]
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: typst-community/setup-typst@v2
      - run: typst compile main.typ output.pdf
      - uses: actions/upload-artifact@v4
        with:
          name: pdf
          path: output.pdf
```

---

## Font Setup (Linux)

```bash
# Install Noto CJK fonts
sudo apt install fonts-noto-cjk fonts-noto-mono

# Verify
typst fonts | grep -i noto
```

---

## Migration from LaTeX

| LaTeX | Typst |
|-------|-------|
| `\section{Title}` | `= Title` |
| `\textbf{bold}` | `*bold*` or `#strong[bold]` |
| `\textit{italic}` | `_italic_` or `#emph[italic]` |
| `\begin{itemize}` | `- item` |
| `$...$` / `$$...$$` | `$...$` / `$$...$$` |
| `\cite{key}` | `#cite("key")` |
| `\ref{fig:1}` | `@fig-1` (auto labels) |
| `\input{file}` | `#include "file.typ"` |
| `\newcommand` | `#let name = ...` |
| Preamble packages | `#import` or `typst.toml` |

---

## Tips

- **Incremental compile**: `typst watch` is near-instant
- **Web editor**: <https://typst.app/app> for quick tests
- **Unicode math**: Type `α`, `∑`, `∫` directly
- **Debug**: `#show: it => { reveal(it); it }` shows AST
- **Page break**: `#pagebreak()` or `#place(top)[content]`

---

## When to Use This Skill

- Creating new documents, reports, papers, slides
- Converting LaTeX projects to Typst
- Setting up Typst project structure
- Configuring CI for PDF generation
- Troubleshooting Typst compilation

**Do not use LaTeX** for new projects — Typst is the default typesetting system.
