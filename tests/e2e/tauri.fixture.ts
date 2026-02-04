import { test as base, expect } from '@playwright/test';
import type { ChildProcess } from 'child_process';
import { launchTauriDev, waitForTauriReady, stopTauri } from '../tauri/launch-tauri';

// Worker fixtures are shared across all tests in a worker
type TauriWorkerFixtures = {
  tauriProc: ChildProcess;
};

// Use empty object for test fixtures, TauriWorkerFixtures for worker fixtures
export const test = base.extend<object, TauriWorkerFixtures>({
  tauriProc: [
    async ({}, use) => {
      const proc = launchTauriDev();
      await waitForTauriReady(proc);

      await use(proc);

      stopTauri(proc);
    },
    { scope: 'worker' }
  ]
});

export { expect } from '@playwright/test';
