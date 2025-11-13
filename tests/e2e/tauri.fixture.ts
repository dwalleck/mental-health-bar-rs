import { test as base, expect } from '@playwright/test';
import type { ChildProcess } from 'child_process';
import { launchTauriDev, waitForTauriReady, stopTauri } from '../tauri/launch-tauri';

type TauriFixtures = {
  tauriProc: ChildProcess;
};

export const test = base.extend<TauriFixtures>({
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
