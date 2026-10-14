---
name: okf
description: Generate, validate, and convert Open Knowledge Format (OKF) frontmatter and documents. Use when creating RFC 3339-compliant metadata, markdownlint-compliant documents, or when the user mentions OKF, Open Knowledge Format, frontmatter generation, or RFC 3339 timestamps.
---

# Open Knowledge Format (OKF) Skill

## Overview

This skill handles OKF-compliant document generation with strict adherence to:
- **OKF Specification**: YAML frontmatter with required/optional fields
- **RFC 3339**: All timestamps with explicit UTC offset (e.g., `2026-09-19T09:00:00+09:00`)
- **markdownlint**: Output must pass all markdownlint rules (MD013, MD025, MD041, etc.)
- **CommonMark/GFM**: Primary/secondary markup standards

## Quick Start

### Generate OKF Frontmatter

```bash
rust-script scripts/okf_generate.rs \
  --type "Reference" \
  --title "API仕様書" \
  --description "REST APIのエンドポイント定義" \
  --tags "api,spec,rest" \
  --resource "https://api.example.com/openapi.json" \
  --status "draft"
```

### Validate OKF Document

```bash
rust-script scripts/okf_validate.rs document.md
```

### Convert Legacy Frontmatter to OKF

```bash
rust-script scripts/okf_convert.rs legacy.md --output okf.md
```

## Frontmatter Schema

### Required Fields

| Field | Type | Description |
|-------|------|-------------|
| `type` | string | Document type (e.g., `Reference`, `Playbook`, `ADR`, `Runbook`, `Spec`) |
| `title` | string | Human-readable title |
| `description` | string | One-sentence summary |
| `tags` | array[string] | Lowercase kebab-case tags |
| `generated.by` | string | Generator identifier (e.g., `okf-generator`, `human`, `AI-model-name`) |
| `generated.at` | string | **RFC 3339 with explicit offset** (e.g., `2026-09-19T09:00:00+09:00`) |

### Optional Fields

| Field | Type | Description |
|-------|------|-------------|
| `resource` | string | Canonical URL or URI for the primary resource |
| `verified.by` | string | Validator identifier |
| `verified.at` | string | RFC 3339 timestamp of validation |
| `status` | string | `draft` \| `review` \| `approved` \| `deprecated` \| `archived` |
| `stale_after` | string | RFC 3339 timestamp when content may be outdated |

### Full Example

```yaml
---
type: Reference
title: "認証API仕様"
description: "OAuth 2.0 / OIDC エンドポイント定義とトークンフロー"
tags:
  - auth
  - oauth2
  - oidc
  - api-spec
resource: "https://auth.example.com/.well-known/openid-configuration"
generated:
  by: "okf-generator"
  at: "2026-09-19T09:00:00+09:00"
verified:
  by: "security-team"
  at: "2026-09-20T14:30:00+09:00"
status: "approved"
stale_after: "2027-03-19T09:00:00+09:00"
---
```

## RFC 3339 Timestamp Rules

**ALWAYS** use explicit UTC offset. Never use:
- `Z` suffix (use `+00:00` instead)
- Date-only (`2026-09-19`)
- Timezone names (`JST`, `UTC`)

**Correct formats:**
- `2026-09-19T09:00:00+09:00` (JST)
- `2026-09-19T00:00:00+00:00` (UTC)
- `2026-09-19T15:30:45-07:00` (PDT)

**Get current timestamp:**
```bash
date -u +"%Y-%m-%dT%H:%M:%S%z" | sed 's/\([+-]\)\([0-9]\{2\}\)\([0-9]\{2\}\)/\1\2:\3/'
# Output: 2026-09-19T00:00:00+00:00

date +"%Y-%m-%dT%H:%M:%S%z" | sed 's/\([+-]\)\([0-9]\{2\}\)\([0-9]\{2\}\)/\1\2:\3/'
# Output: 2026-09-19T09:00:00+09:00
```

## Markdownlint Compliance

Output documents MUST pass these rules:

| Rule | Requirement |
|------|-------------|
| MD013 | Line length ≤ 120 chars (code blocks excluded) |
| MD025 | Single H1 (`#`) only in frontmatter `title`; body starts at `##` |
| MD041 | First heading in body must be `##` |
| MD022 | Blank lines around headings |
| MD032 | Blank lines around lists |
| MD031 | Fenced code blocks |
| MD040 | Code blocks must have language |

### Heading Structure

```markdown
---
title: "Document Title"  # H1 equivalent
---

## Section          # H2 - first body heading
### Subsection      # H3
#### Detail         # H4
```

## Document Types

| Type | Use Case | Typical Sections |
|------|----------|------------------|
| `Reference` | API specs, config references, lookup tables | Overview, Parameters, Examples |
| `Playbook` | Operational procedures, runbooks | Prerequisites, Steps, Verification, Rollback |
| `ADR` | Architecture Decision Records | Context, Decision, Consequences, Alternatives |
| `Runbook` | Incident response, deployment procedures | Detection, Diagnosis, Resolution, Postmortem |
| `Spec` | Technical specifications | Requirements, Design, Interfaces, Tests |
| `Report` | Analysis, audit, investigation results | Executive Summary, Findings, Evidence, Recommendations |

## Templates

### Reference Template

```markdown
---
type: Reference
title: "{{タイトル}}"
description: "{{1文要約}}"
tags:
  - {{tag1}}
  - {{tag2}}
generated:
  by: "{{generator}}"
  at: "{{RFC3339タイムスタンプ}}"
---

## 概要
{{概要説明}}

## 詳細
{{詳細内容}}

## 例
{{使用例}}

## 参照
- [参照名]({{URL}})
```

### Playbook Template

```markdown
---
type: Playbook
title: "{{手順名}}"
description: "{{目的の1文要約}}"
tags:
  - {{tag1}}
  - {{tag2}}
generated:
  by: "{{generator}}"
  at: "{{RFC3339タイムスタンプ}}"
status: "draft"
---

## 前提条件
- {{前提1}}
- {{前提2}}

## 手順
### Step 1: {{ステップ名}}
```bash
{{コマンド}}
```

### Step 2: {{ステップ名}}
{{説明}}

## 検証
- [ ] {{検証項目1}}
- [ ] {{検証項目2}}

## ロールバック
{{ロールバック手順}}

## 参照
- [参照名]({{URL}})
```

## Validation Checklist

Run these checks on every OKF document:

- [ ] Frontmatter parses as valid YAML
- [ ] All required fields present
- [ ] `generated.at` is valid RFC 3339 with offset
- [ ] `verified.at` (if present) is valid RFC 3339 with offset
- [ ] `stale_after` (if present) is valid RFC 3339 with offset
- [ ] `tags` are lowercase kebab-case
- [ ] `type` is one of the defined types
- [ ] `status` (if present) is valid enum value
- [ ] Body passes markdownlint (run `markdownlint-cli2 document.md`)
- [ ] First body heading is `##`
- [ ] No `Z` suffix in timestamps
- [ ] No date-only timestamps

## Utility Scripts

| Script | Purpose |
|--------|---------|
| `okf_generate.rs` | Generate OKF frontmatter from CLI args (rust-script) |
| `okf_validate.rs` | Validate OKF document (frontmatter + markdownlint) (rust-script) |
| `okf_convert.rs` | Convert legacy frontmatter to OKF (rust-script) |
| `okf_timestamp.rs` | Output current RFC 3339 timestamp with offset (rust-script) |

## Integration with AGENTS.md

This skill enforces the OKF rules defined in `/home/koppe/.config/opencode/AGENTS.md`:
- RFC 3339 timestamps with explicit offset
- markdownlint compliance
- YAML frontmatter structure
- Tag formatting conventions

## Examples

### Example 1: Generate Reference Document

**Input:**
```bash
rust-script scripts/okf_generate.rs \
  --type Reference \
  --title "Kubernetesリソース定義" \
  --description "Deployment/Service/Ingressのマニフェスト例" \
  --tags "k8s,manifest,deployment" \
  --status "review"
```

**Output:**
```yaml
---
type: Reference
title: "Kubernetesリソース定義"
description: "Deployment/Service/Ingressのマニフェスト例"
tags:
  - k8s
  - manifest
  - deployment
generated:
  by: "okf-generator"
  at: "2026-09-19T09:00:00+09:00"
status: "review"
---
```

### Example 2: Validation Output

```bash
$ rust-script scripts/okf_validate.rs doc.md
✅ Frontmatter: Valid YAML
✅ Required fields: All present
✅ RFC 3339: generated.at=2026-09-19T09:00:00+09:00
✅ Tags: lowercase kebab-case
✅ Markdownlint: 0 errors, 0 warnings
✅ Heading structure: Valid (starts with ##)
```

## Anti-Patterns

| ❌ Avoid | ✅ Use |
|----------|--------|
| `date: "2026-09-19"` | `generated.at: "2026-09-19T09:00:00+09:00"` |
| `timestamp: "2026-09-19T00:00:00Z"` | `generated.at: "2026-09-19T00:00:00+00:00"` |
| `tags: ["API Spec", "Auth"]` | `tags: ["api-spec", "auth"]` |
| `# Title` in body | `## Title` as first body heading |
| No blank line after frontmatter | Blank line after `---` |