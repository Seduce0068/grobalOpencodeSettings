# Integration Test Specification (API)

## Target: {{SERVICE_NAME}} API

## Test basis

| 項目 | 内容 |
| --- | --- |
| 対象 (test item) | {{SERVICE_NAME}} API |
| 根拠 (test basis) | {{OPENAPI_SPEC}} / {{REQUIREMENT_DOC}} |
| 判定基準 (test oracle) | {{STATUS_AND_BODY_SPEC}} |
| プロファイル | {{PROFILE_NAME}} |
| 対象スコープ | {{ASVS_CHAPTERS}} (ASVS 5.0.0 の章) |

## Risk & tier

| risk_id | リスク | tier | 完了基準 (カバレッジ) |
| --- | --- | --- | --- |
| R-001 | {{RISK_DESCRIPTION}} | {{T1\|T2\|T3}} | integration {{COV}} |

## Design technique

| 技法 | 29119-4 節 | 適用箇所 |
| --- | --- | --- |
| {{EQUIVALENCE_PARTITIONING}} | 5.2.1 | 正常系・異常系 |
| {{BOUNDARY_VALUE_ANALYSIS}} | 5.2.3 | 境界値 |
| {{DECISION_TABLE_TESTING}} | 5.2.6 | 認可・条件分岐 |
| {{STATE_TRANSITION_TESTING}} | 5.2.8 | セッション・状態 |
| {{REQUIREMENTS_BASED_TESTING}} | 5.2.12 | OpenAPI 定義との対応 |

## Document map

| 29119-3 情報項目 | 該当セクション |
| --- | --- |
| Test case specification | Test Suites |
| Test procedure specification | CI/CD Integration |
| Test data requirements | Test Data Management |
| Test environment requirements | Base Configuration |
| Test execution log / result | `evidence/test-run-*.json` |

### Base Configuration

```yaml
base_url: "{{BASE_URL}}"  # 環境変数で注入
timeout_ms: 10000
headers:
  Content-Type: application/json
  Authorization: "Bearer {{TOKEN}}"  # テスト用トークン
```

### Test Suites

#### 1. Health & Readiness

```typescript
describe('Health Endpoints', () => {
  test('GET /health → 200 + {status: "ok"}', async () => {
    const res = await request.get('/health');
    expect(res.status).toBe(200);
    expect(res.body).toMatchObject({ status: 'ok' });
  });

  test('GET /ready → 200/503 依存サービス状態反映', async () => {
    const res = await request.get('/ready');
    expect([200, 503]).toContain(res.status);
  });
});
```

#### 2. CRUD Operations (per resource)

```typescript
describe('{{RESOURCE}} CRUD', () => {
  let createdId: string;

  test('POST /{{resource}} → 201 + created resource', async () => {
    const payload = {{VALID_PAYLOAD}};
    const res = await request.post(`/{{resource}}`).send(payload);
    expect(res.status).toBe(201);
    expect(res.body).toMatchObject({ id: expect.any(String), ...payload });
    createdId = res.body.id;
  });

  test('GET /{{resource}}/:id → 200 + resource', async () => {
    const res = await request.get(`/{{resource}}/${createdId}`);
    expect(res.status).toBe(200);
    expect(res.body.id).toBe(createdId);
  });

  test('GET /{{resource}} → 200 + paginated list', async () => {
    const res = await request.get(`/{{resource}}?limit=10&offset=0`);
    expect(res.status).toBe(200);
    expect(res.body).toMatchObject({
      data: expect.arrayContaining([expect.objectContaining({ id: createdId })]),
      pagination: expect.objectContaining({ limit: 10, offset: 0, total: expect.any(Number) })
    });
  });

  test('PATCH /{{resource}}/:id → 200 + updated resource', async () => {
    const res = await request.patch(`/{{resource}}/${createdId}`).send({{PARTIAL_UPDATE}});
    expect(res.status).toBe(200);
    expect(res.body).toMatchObject({{PARTIAL_UPDATE}});
  });

  test('DELETE /{{resource}}/:id → 204', async () => {
    const res = await request.delete(`/{{resource}}/${createdId}`);
    expect(res.status).toBe(204);
    
    const getRes = await request.get(`/{{resource}}/${createdId}`);
    expect(getRes.status).toBe(404);
  });
});
```

#### 3. Error Cases (共通)

```typescript
describe('Error Handling', () => {
  const errorCases = [
    { name: 'Not Found', method: 'GET', path: '/{{resource}}/non-existent-id', expected: 404 },
    { name: 'Validation Error', method: 'POST', path: '/{{resource}}', body: {{INVALID_PAYLOAD}}, expected: 422 },
    { name: 'Unauthorized', method: 'GET', path: '/{{resource}}', headers: {}, expected: 401 },
    { name: 'Forbidden', method: 'DELETE', path: '/{{resource}}/id', headers: { Authorization: 'Bearer {{LOW_PRIV_TOKEN}}' }, expected: 403 },
    { name: 'Rate Limited', method: 'GET', path: '/{{resource}}', repeat: 101, expected: 429 },
  ];

  errorCases.forEach(({ name, method, path, body, headers, expected, repeat }) => {
    test(`${name} → ${expected}`, async () => {
      for (let i = 0; i < (repeat || 1); i++) {
        const res = await request[method.toLowerCase()](path)
          .send(body)
          .set(headers || { Authorization: 'Bearer {{VALID_TOKEN}}' });
        if (i === (repeat || 1) - 1) {
          expect(res.status).toBe(expected);
          expect(res.body).toMatchObject({
            error: expect.any(String),
            message: expect.any(String),
            code: expect.any(String)
          });
        }
      }
    });
  });
});
```

#### 4. Contract Testing (Consumer-Driven)

```typescript
// Pact を使用したコントラクトテスト
import { PactV3, MatchersV3 } from '@pact-foundation/pact';

const { like, eachLike, iso8601DateTimeWithMillis } = MatchersV3;

const provider = new PactV3({
  consumer: '{{CONSUMER_NAME}}',
  provider: '{{PROVIDER_NAME}}',
  dir: './pacts',
});

describe('Contract: {{RESOURCE}} API', () => {
  test('GET /{{resource}}/:id returns expected shape', async () => {
    await provider.addInteraction({
      state: '{{RESOURCE}} with id "123" exists',
      uponReceiving: 'a request for {{RESOURCE}} 123',
      withRequest: {
        method: 'GET',
        path: '/{{resource}}/123',
        headers: { Authorization: like('Bearer {{TOKEN}}') },
      },
      willRespondWith: {
        status: 200,
        headers: { 'Content-Type': 'application/json' },
        body: like({
          id: '123',
          name: like('{{NAME}}'),
          createdAt: iso8601DateTimeWithMillis(),
          status: like('active'),
        }),
      },
    });

    await provider.executeTest(async (mockServer) => {
      const res = await fetch(`${mockServer.url}/{{resource}}/123`, {
        headers: { Authorization: 'Bearer test-token' },
      });
      expect(res.status).toBe(200);
      const data = await res.json();
      expect(data.id).toBe('123');
    });
  });
});
```

---

## Test Data Management

| 戦略 | 用途 | 実装 |
| --- | --- | --- |
| **Fixtures** | 静的テストデータ | `tests/fixtures/*.json` |
| **Factories** | 動的生成・バリエーション | `tests/factories/*.ts` (Factory.ts) |
| **Builders** | 複雑なオブジェクト構築 | `tests/builders/*.ts` |
| **Database Seeding** | 統合テスト用DB初期化 | `tests/seeds/*.sql` + トランザクションロールバック |

```typescript
// tests/factories/user.factory.ts
export const createUser = (overrides: Partial<User> = {}): User => ({
  id: crypto.randomUUID(),
  email: `test-${Date.now()}@example.com`,
  name: 'Test User',
  status: 'active',
  createdAt: new Date().toISOString(),
  ...overrides,
});

// 使用例
const admin = createUser({ role: 'admin', email: 'admin@test.com' });
const inactive = createUser({ status: 'inactive' });
```

---

## CI/CD Integration

```yaml
# .github/workflows/integration-test.yml

name: Integration Tests

on:
  pull_request:
    types: [opened, synchronize, reopened]
  workflow_dispatch:
jobs:
  integration:
    runs-on: ubuntu-latest
    services:
      postgres:
        image: postgres:16
        env:
          POSTGRES_PASSWORD: test
          POSTGRES_DB: testdb
        ports: [5432:5432]
        options: >-
          --health-cmd="pg_isready -U postgres"
          --health-interval=10s
          --health-timeout=5s
          --health-retries=5
      redis:
        image: redis:7
        ports: [6379:6379]

    steps:
      - uses: actions/checkout@v4
      - name: Setup
        run: |
          npm ci
          npx prisma migrate deploy
      
      - name: Run Integration Tests
        env:
          DATABASE_URL: postgresql://postgres:test@localhost:5432/testdb
          REDIS_URL: redis://localhost:6379
          BASE_URL: http://localhost:3000
        run: |
          npm run test:integration -- --json > integration-result.json
      
      - name: Upload Evidence
        if: always()
        uses: actions/upload-artifact@v4
        with:
          name: integration-evidence
          path: |
            integration-result.json
            coverage/lcov.info
      
      - name: Verify Evidence
        run: |
          opencode run "/test-spec evidence integration-result.json --spec-hash $(sha256sum specs/test-*.md | cut -d' ' -f1)"
```

---

## Traceability

| Scenario | Req | ASVS | 800-53 | CC | Technique | Tier | Status |
| --- | --- | --- | --- | --- | --- | --- | --- |
| {{SCENARIO_ID}} | REQ-{{ID}} | {{v5.0.0-V0.0.0}} | {{SP800-53r5.2/XX-X}} | {{ATE_FUN.1}} | 5.2.6 | T1 | ⬜ TODO |

ASVS / 800-53 / CC の各列は該当なしを `-` とする。空欄は禁止。

---

## エビデンス出力例

```json
{
  "spec_hash": "sha256:a1b2c3d4...",
  "timestamp": "2026-10-08T21:30:00+09:00",
  "document_type": "test-execution-log",
  "profile": "{{PROFILE_NAME}}",
  "environment": {
    "os": "linux",
    "runtime": "node",
    "versions": { "node": "22.22.1", "npm": "9.2.0" }
  },
  "results": {
    "passed": 47,
    "failed": 0,
    "skipped": 2,
    "duration_ms": 15420,
    "coverage": {
      "lines": 87.3,
      "branches": 82.1,
      "functions": 90.5,
      "statements": 87.3
    },
    "mutation_score": 78.4
  },
  "techniques": [
    { "id": "equivalence-partitioning", "ref": "29119-4 5.2.1" },
    { "id": "decision-table-testing", "ref": "29119-4 5.2.6" }
  ],
  "risk_items": [
    { "id": "R-001", "tier": "T1", "asvs": ["v5.0.0-V8.1.1"] }
  ],
  "assurance": {
    "level": "A2",
    "cc_components": ["ATE_FUN.1", "ATE_COV.2"],
    "independent_execution": false
  },
  "conformance": "tailored",
  "signature": "MEUCIQD..."
}
```
