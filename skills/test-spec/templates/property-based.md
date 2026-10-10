# Property-Based Test Specification (fast-check)

## Target: {{MODULE_NAME}}

### 概要
プロパティベーステストで「仕様として常に成り立つべき不変条件」を定義し、数千ケースで自動検証する。

### フレームワーク
| 言語 | ライブラリ | インストール |
|---|---|---|
| TypeScript | fast-check | `npm i -D fast-check` |
| Rust | proptest | `cargo add proptest --dev` |
| Python | hypothesis | `pip install hypothesis` |

---

## プロパティ定義テンプレート

### 1. 純粋関数のプロパティ
```typescript
// tests/property/{{module}}.property.ts
import * as fc from 'fast-check';
import { {{functionName}} } from '{{module}}';

describe('Property: {{FUNCTION_NAME}}', () => {
  // Property 1: 入力順序不変 (可換性)
  test('commutative: f(a, b) === f(b, a)', () => {
    fc.assert(fc.property(fc.integer(), fc.integer(), (a, b) => {
      expect({{functionName}}(a, b)).toEqual({{functionName}}(b, a));
    }), { numRuns: 10000 });
  });

  // Property 2: 結合律
  test('associative: f(f(a, b), c) === f(a, f(b, c))', () => {
    fc.assert(fc.property(fc.integer(), fc.integer(), fc.integer(), (a, b, c) => {
      expect({{functionName}}({{functionName}}(a, b), c)).toEqual({{functionName}}(a, {{functionName}}(b, c)));
    }), { numRuns: 10000 });
  });

  // Property 3: 単位元
  test('identity: f(a, identity) === a', () => {
    fc.assert(fc.property(fc.integer(), (a) => {
      expect({{functionName}}(a, {{IDENTITY_VALUE}})).toEqual(a);
    }), { numRuns: 10000 });
  });

  // Property 4: 冪等性 (該当する場合)
  test('idempotent: f(f(a)) === f(a)', () => {
    fc.assert(fc.property(fc.integer(), (a) => {
      expect({{functionName}}({{functionName}}(a))).toEqual({{functionName}}(a));
    }), { numRuns: 5000 });
  });

  // Property 5: 範囲保持
  test('range preservation: output within expected bounds', () => {
    fc.assert(fc.property(fc.integer({ min: -1000, max: 1000 }), (a) => {
      const result = {{functionName}}(a);
      expect(result).toBeGreaterThanOrEqual({{MIN_OUTPUT}});
      expect(result).toBeLessThanOrEqual({{MAX_OUTPUT}});
    }), { numRuns: 10000 });
  });
});
```

### 2. 状態を持つシステムのプロパティ (State Machine)
```typescript
// tests/property/{{module}}.state-machine.ts
import * as fc from 'fast-check';
import { {{System}} } from '{{module}}';

const commands = (system: {{System}}) => ({
  // 操作定義
  push: fc.record({ value: fc.integer() }).map(({ value }) => ({
    check: () => true,
    run: (s: {{System}}) => s.push(value),
    toString: () => `push(${value})`,
  })),
  
  pop: fc.record({}).map(() => ({
    check: (s: {{System}}) => s.size() > 0,
    run: (s: {{System}}) => { s.pop(); },
    toString: () => 'pop()',
  })),
  
  clear: fc.record({}).map(() => ({
    check: () => true,
    run: (s: {{System}}) => s.clear(),
    toString: () => 'clear()',
  })),
});

// 不変条件
const invariants = (system: {{System}}) => [
  () => expect(system.size()).toBeGreaterThanOrEqual(0),
  () => expect(system.size()).toBeLessThanOrEqual({{MAX_SIZE}}),
  () => system.isEmpty() === (system.size() === 0),
];

test('State Machine: {{SYSTEM}} maintains invariants', () => {
  fc.assert(
    fc.property(
      fc.commands(commands(new {{System}}()), { size: 'small' }),
      (cmds) => {
        const system = new {{System}}();
        cmds.forEach((cmd) => cmd.run(system));
        invariants(system).forEach((inv) => inv());
      }
    ),
    { numRuns: 1000, maxSkipsPerRun: 100 }
  );
});
```

### 3. シリアライズ/デシリアライズの可逆性
```typescript
test('roundtrip: deserialize(serialize(x)) === x', () => {
  fc.assert(fc.property(fc.{{ARBITRARY_FOR_TYPE}}(), (input) => {
    const serialized = serialize(input);
    const deserialized = deserialize(serialized);
    expect(deserialized).toEqual(input);
  }), { numRuns: 5000 });
});
```

### 4. エンコード/デコードの可逆性
```typescript
test('encoding roundtrip', () => {
  fc.assert(fc.property(fc.string(), (input) => {
    const encoded = encode(input);
    const decoded = decode(encoded);
    expect(decoded).toBe(input);
  }), { numRuns: 10000 });
});
```

### 5. ソート・順序の不変性
```typescript
test('sort stability: equal elements maintain relative order', () => {
  fc.assert(fc.property(fc.array(fc.record({ 
    key: fc.integer({ min: 0, max: 10 }),
    value: fc.string()
  })), (arr) => {
    const sorted = stableSort(arr, (a) => a.key);
    
    // 同一 key の要素順序が保持される
    for (let i = 1; i < sorted.length; i++) {
      if (sorted[i].key === sorted[i-1].key) {
        const origIdx1 = arr.findIndex(x => x === sorted[i-1]);
        const origIdx2 = arr.findIndex(x => x === sorted[i]);
        expect(origIdx1).toBeLessThan(origIdx2);
      }
    }
  }), { numRuns: 5000 });
});
```

---

## 実行設定

```json
// package.json
{
  "scripts": {
    "test:property": "vitest run tests/property/**/*.property.ts --reporter=json",
    "test:property:ci": "vitest run tests/property/**/*.property.ts --reporter=json --numRuns=50000"
  },
  "vitest": {
    "testTimeout": 120000
  }
}
```

### CI 実行オプション
```bash
# 開発時: 高速
npm run test:property -- --numRuns=1000

# CI: 徹底
npm run test:property:ci -- --numRuns=50000

# デバッグ: 特定プロパティのみ
npm run test:property -- --testNamePattern="commutative"
```

---

## 失敗時の最小再現ケース取得

fast-check は失敗時に **最小化** を自動実行します。

```typescript
// 出力例
// ✘ Property failed after 234 tests
// ✘ Counterexample: [42, -1]  (最小化済み)
// ✘ Shrunk 3 time(s)
// Original: [12451, -9876, 0, 42, -1]
```

---

## カスタム Arbitrary (任意値生成器)

```typescript
// tests/property/arbitraries.ts
import * as fc from 'fast-check';
import { User, UserStatus } from '{{domain}}';

// ドメイン固有の任意値
export const userArb = fc.record({
  id: fc.uuidV4(),
  email: fc.email(),
  name: fc.string({ minLength: 1, maxLength: 100 }).filter(n => !n.includes('\n')),
  status: fc.constantFrom(...Object.values(UserStatus)),
  createdAt: fc.date({ min: new Date('2020-01-01'), max: new Date() }).map(d => d.toISOString()),
});

// 使用例
test('user validation property', () => {
  fc.assert(fc.property(userArb, (user) => {
    expect(validateUser(user)).toBe(true);
  }), { numRuns: 5000 });
});
```

---

## エビデンス出力

```json
{
  "spec_hash": "sha256:...",
  "timestamp": "2026-10-08T21:30:00+09:00",
  "results": {
    "properties": [
      { "name": "commutative", "passed": true, "runs": 10000, "shrinks": 0 },
      { "name": "associative", "passed": true, "runs": 10000, "shrinks": 0 },
      { "name": "identity", "passed": true, "runs": 10000, "shrinks": 0 },
      { "name": "range_preservation", "passed": true, "runs": 10000, "shrinks": 0 }
    ],
    "total_runs": 40000,
    "total_shrinks": 0,
    "duration_ms": 12500
  }
}
```

---

## よくあるプロパティパターン

| カテゴリ | プロパティ | 適用例 |
|---|---|---|
| **代数的** | 可換・結合・単位元・冪等 | 数学関数、集合操作、CRDT |
| **順序** | 単調性・反射・対称・推移 | ソート、比較、優先度キュー |
| **可逆性** | ラウンドトリップ | シリアライズ、暗号、圧縮 |
| **不変条件** | 範囲・整合性・整合性 | データ構造、状態機械、DB制約 |
| **等価性** | リファクタリング前後同値 | 最適化、並列化、キャッシュ |

---

## 導入チェックリスト

- [ ] 対象関数が純粋（副作用なし）または状態機械でモデル化可能
- [ ] 入力ドメインを Arbitrary で表現可能
- [ ] 期待される不変条件を数学的に定義可能
- [ ] CI で `numRuns ≥ 10000` 確保
- [ ] 失敗時の最小再現ケースを Issue に自動記録