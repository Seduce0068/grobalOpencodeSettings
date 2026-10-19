# E2E Test Specification (Playwright MCP)

## Target: {{APPLICATION_NAME}}

## Test basis

| 項目 | 内容 |
| --- | --- |
| 対象 (test item) | {{APPLICATION_NAME}} (主要ユーザーフロー) |
| 根拠 (test basis) | {{USER_STORY}} / {{REQUIREMENT_DOC}} |
| 判定基準 (test oracle) | {{VISIBLE_STATE_OR_API_RESPONSE}} |
| プロファイル | {{PROFILE_NAME}} |

## Risk & tier

| risk_id | リスク | tier | 完了基準 |
| --- | --- | --- | --- |
| R-001 | {{RISK_DESCRIPTION}} | {{T1\|T2\|T3}} | e2e 主要フロー 100% |

## Design technique

| 技法 | 29119-4 節 | 適用箇所 |
| --- | --- | --- |
| {{SCENARIO_TESTING}} | 5.2.9 | ユーザーフロー |
| {{STATE_TRANSITION_TESTING}} | 5.2.8 | 画面遷移・セッション |
| {{ERROR_GUESSING}} | 5.4.1 | 異常系・リンク切れ |

## Document map

| 29119-3 情報項目 | 該当セクション |
| --- | --- |
| Test case specification | Test Specs (BDD Style) |
| Test procedure specification | CI/CD 設定 |
| Test environment requirements | Playwright MCP Configuration |
| Test execution log / result | `evidence/test-run-*.json` |

### Playwright MCP Configuration

```json
{
  "mcpServers": {
    "playwright": {
      "command": "npx",
      "args": ["@playwright/mcp@latest", "--headless", "--isolated"],
      "env": {
        "PLAYWRIGHT_MCP_BASE_URL": "{{BASE_URL}}"
      }
    }
  }
}
```

### Page Objects (推奨構造)

```text
tests/e2e/
├── pages/
│   ├── base.page.ts          # 共通操作・待機・スクショ
│   ├── login.page.ts
│   ├── dashboard.page.ts
│   └── {{feature}}.page.ts
├── fixtures/
│   └── test-data.ts
├── specs/
│   ├── {{feature}}.spec.ts
│   └── {{feature}}.a11y.spec.ts
└── utils/
    ├── auth.ts               # 認証ヘルパー
    ├── selectors.ts          # セレクタ定数
    └── assertions.ts         # カスタムアサーション
```

### Base Page (共通)

```typescript
// tests/e2e/pages/base.page.ts
import { Page, Locator, expect } from '@playwright/test';

export abstract class BasePage {
  constructor(protected page: Page) {}

  async goto(path: string) {
    await this.page.goto(path, { waitUntil: 'networkidle' });
  }

  async waitForLoaded() {
    await this.page.waitForLoadState('domcontentloaded');
    await this.page.waitForLoadState('networkidle');
  }

  async screenshot(name: string) {
    await this.page.screenshot({ path: `evidence/screenshots/${name}-${Date.now()}.png`, fullPage: true });
  }

  async expectVisible(locator: Locator, timeout = 5000) {
    await expect(locator).toBeVisible({ timeout });
  }

  async expectText(locator: Locator, text: string | RegExp) {
    await expect(locator).toContainText(text);
  }
}
```

### Feature Page Object

```typescript
// tests/e2e/pages/{{feature}}.page.ts
import { Page, Locator, expect } from '@playwright/test';
import { BasePage } from './base.page';

export class {{Feature}}Page extends BasePage {
  // Selectors
  readonly submitButton: Locator;
  readonly inputField: Locator;
  readonly resultList: Locator;
  readonly errorMessage: Locator;

  constructor(page: Page) {
    super(page);
    this.submitButton = page.locator('[data-testid="submit-btn"]');
    this.inputField = page.locator('[data-testid="input-field"]');
    this.resultList = page.locator('[data-testid="result-list"]');
    this.errorMessage = page.locator('[data-testid="error-message"]');
  }

  async performAction(input: string) {
    await this.inputField.fill(input);
    await this.submitButton.click();
    await this.waitForLoaded();
  }

  async expectResult(expected: string) {
    await this.expectVisible(this.resultList);
    await this.expectText(this.resultList, expected);
  }

  async expectError(expected: string) {
    await this.expectVisible(this.errorMessage);
    await this.expectText(this.errorMessage, expected);
  }
}
```

### Test Specs (BDD Style)

```typescript
// tests/e2e/specs/{{feature}}.spec.ts
import { test, expect } from '@playwright/test';
import { {{Feature}}Page } from '../pages/{{feature}}.page';
import { loginAs } from '../utils/auth';

test.describe('{{FEATURE_NAME}}', () => {
  let pageObj: {{Feature}}Page;

  test.beforeEach(async ({ page }) => {
    pageObj = new {{Feature}}Page(page);
    await loginAs(page, 'standard_user');
    await pageObj.goto('/{{feature}}');
  });

  test('正常系: {{ACTION}} で {{EXPECTED_RESULT}}', async () => {
    await pageObj.performAction('{{VALID_INPUT}}');
    await pageObj.expectResult('{{EXPECTED_OUTPUT}}');
    
    // エビデンス収集
    await pageObj.screenshot('{{feature}}-success');
  });

  test('境界値: {{BOUNDARY_INPUT}} で {{BOUNDARY_RESULT}}', async () => {
    await pageObj.performAction('{{BOUNDARY_INPUT}}');
    await pageObj.expectResult('{{BOUNDARY_RESULT}}');
  });

  test('異常系: {{INVALID_INPUT}} でエラー表示', async () => {
    await pageObj.performAction('{{INVALID_INPUT}}');
    await pageObj.expectError('{{ERROR_MESSAGE}}');
    
    await pageObj.screenshot('{{feature}}-error');
  });

  test('アクセシビリティ: キーボード操作・スクリーンリーダー対応', async () => {
    // Tab 順序
    await pageObj.page.keyboard.press('Tab');
    await expect(pageObj.inputField).toBeFocused();
    
    await pageObj.page.keyboard.press('Tab');
    await expect(pageObj.submitButton).toBeFocused();
    
    // ARIA 属性
    await expect(pageObj.inputField).toHaveAttribute('aria-label');
    await expect(pageObj.submitButton).toHaveAttribute('aria-disabled', 'false');
  });
});
```

### Visual Regression (オプション)

```typescript
import { test, expect } from '@playwright/test';

test('Visual: {{FEATURE}} page matches baseline', async ({ page }) => {
  await page.goto('/{{feature}}');
  await page.waitForLoadState('networkidle');
  
  await expect(page).toHaveScreenshot('{{feature}}-page.png', {
    fullPage: true,
    animations: 'disabled',
    threshold: 0.2,
  });
});
```

### MCP Tools 連携 (自動化)

```typescript
// tests/e2e/utils/mcp-helpers.ts
import { Page } from '@playwright/test';

export async function mcpNavigate(page: Page, url: string) {
  // MCP ツール経由でナビゲート (browser_navigate)
  // 実際のテストでは page.goto() を使用
  await page.goto(url, { waitUntil: 'networkidle' });
}

export async function mcpSnapshot(page: Page, selector?: string) {
  // browser_snapshot でアクセシビリティツリー取得
  // CI で構造比較に使用
}

export async function mcpNetworkRequests(page: Page, filter?: string) {
  // browser_network_requests で API 呼び出し検証
  // 期待するエンドポイント・ペイロード確認
}
```

### CI/CD 設定

```yaml
# .github/workflows/e2e-test.yml
name: E2E Tests (Playwright)

on:
  pull_request:
    types: [opened, synchronize, reopened]
  workflow_dispatch:
  schedule:
    - cron: '0 2 * * *'  # 毎日 2:00 UTC 定期実行

jobs:
  e2e:
    runs-on: ubuntu-latest
    timeout-minutes: 30
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Node & Playwright
        run: |
          npm ci
          npx playwright install --with-deps chromium
      
      - name: Start App
        run: |
          npm run build
          npm run start &
          sleep 10  # 起動待ち
      
      - name: Run E2E Tests
        env:
          PLAYWRIGHT_BASE_URL: http://localhost:3000
        run: |
          npx playwright test --reporter=json --output=evidence/e2e-result.json
      
      - name: Upload Evidence
        if: always()
        uses: actions/upload-artifact@v4
        with:
          name: e2e-evidence
          path: |
            evidence/e2e-result.json
            evidence/screenshots/
            playwright-report/
      
      - name: Verify Evidence
        if: always()
        run: |
          opencode run "/test-spec evidence evidence/e2e-result.json --spec-hash $(sha256sum specs/test-e2e-*.md | cut -d' ' -f1)"
```

### Selectors 設計指針

| 優先度 | セレクタタイプ | 例 |
| --- | --- | --- |
| 1 | `data-testid` | `[data-testid="submit-btn"]` |
| 2 | ARIA role + name | `page.getByRole('button', {name: 'Submit'})` |
| 3 | Semantic HTML | `page.getByLabel('Email')` |
| 4 | CSS (最終手段) | `.btn-primary` |

**禁止**: nth-child, クラス名のみ, インラインスタイル依存

## Traceability

| Scenario | Req | ASVS | 800-53 | CC | Technique | Tier | Status |
| --- | --- | --- | --- | --- | --- | --- | --- |
| {{SCENARIO_ID}} | REQ-{{ID}} | {{v5.0.0-V0.0.0}} | {{SP800-53r5.2/XX-X}} | {{ATE_FUN.1}} | 5.2.9 | T1 | ⬜ TODO |

ASVS / 800-53 / CC の各列は該当なしを `-` とする。空欄は禁止。
