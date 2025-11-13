End-to-end testing setup (Playwright + Tauri + SvelteKit)

This project is configured for two kinds of Playwright tests:

1) Browser-only SvelteKit E2E
2) Tauri desktop app E2E (real Tauri process running alongside Playwright)

Files added

- playwright.config.ts
  - Defines:
    - Project "web-chromium" for browser E2E.
    - Project "tauri-e2e" for Tauri E2E.
  - Uses:
    - testDir: ./tests/e2e
    - webServer:
      - command: npm run dev
      - url: http://127.0.0.1:5173

- tests/e2e/home.web.spec.ts
  - Minimal browser E2E example:
    - Starts from baseURL (via webServer) and asserts the page loads.

- tests/tauri/launch-tauri.ts
  - Helper to:
    - launchTauriDev(): runs `cargo tauri dev` from ./src-tauri with TAURI_MODE=e2e.
    - waitForTauriReady(): placeholder wait loop; you can improve this by watching for a specific log line.
    - stopTauri(): stops the dev process.

- tests/e2e/tauri.fixture.ts
  - Playwright test fixture that:
    - Starts Tauri via launchTauriDev().
    - Waits until it is (naively) considered ready.
    - Exposes tauriProc so tests run while Tauri is live.
    - Stops Tauri when tests complete.

- tests/e2e/basic.tauri.spec.ts
  - Example Tauri E2E:
    - Uses the tauri.fixture.
    - Navigates to http://127.0.0.1:5173/ and asserts the UI is visible.
    - Intended to represent interacting with your SvelteKit UI while the real Tauri app is running.

- package.json
  - Added scripts:
    - "test:e2e": "playwright test"
    - "test:e2e:web": "playwright test --project=web-chromium"
    - "test:e2e:tauri": "playwright test --project=tauri-e2e"

Required manual steps

1) Ensure Playwright browsers/system deps
   - Already installed dev dependency: @playwright/test.
   - You chose to manage system deps manually.
   - Run (on your host, as appropriate):
     - npx playwright install
     - Optionally: npx playwright install-deps

2) Fix Node type errors (recommended)
   - The helper/fixture use Node APIs; add Node types:
     - npm install -D @types/node
   - Update tsconfig.json to include:
     - "compilerOptions": {
         "types": ["node", "vite/client", "svelte"]
       }
   - Or limit the included files so Node helpers are recognized where needed.

3) Make Tauri readiness robust (recommended)
   - In src-tauri/src/main.rs (or your startup path), when the window is ready, log a distinct line, for example:
     - println!("E2E_READY");
   - Then update waitForTauriReady to watch proc.stdout for "E2E_READY" instead of using a blind timeout loop.

How to run

- Browser-only SvelteKit E2E:
  - npm run test:e2e:web

- Tauri E2E:
  - Requirements:
    - Rust toolchain + Tauri deps installed.
    - Able to run `cargo tauri dev` from ./src-tauri.
  - Command:
    - npm run test:e2e:tauri

Notes

- The Tauri E2E setup here runs Playwright tests while the actual Tauri dev process is running.
- For more advanced setups (attaching directly to the Tauri WebView via CDP, or testing packaged builds), extend tests/tauri/launch-tauri.ts and the tauri.fixture accordingly.
- For stability in CI:
  - Use a deterministic readiness signal.
  - Add timeouts and better error reporting around the Tauri dev process.

This wiring gives you:
- A working Playwright config.
- A browser E2E example.
- A Tauri-aware test harness that can be evolved to cover real Tauri flows.
