import { defineConfig, devices } from '@playwright/test';

export default defineConfig({
  testDir: './tests/e2e',
  timeout: 60_000,
  expect: {
    timeout: 10_000
  },
  fullyParallel: true,
  reporter: 'list',
  use: {
    trace: 'on-first-retry'
  },
  projects: [
    {
      name: 'web-chromium',
      use: {
        ...devices['Desktop Chrome'],
        baseURL: 'http://127.0.0.1:5173'
      },
      testMatch: /.*web\.spec\.ts/
    },
    {
      name: 'tauri-e2e',
      use: {
        ...devices['Desktop Chrome']
      },
      testMatch: /.*tauri\.spec\.ts/
    }
  ],
  webServer: {
    command: 'npm run dev',
    url: 'http://127.0.0.1:5173',
    reuseExistingServer: true,
    timeout: 60_000
  }
});
