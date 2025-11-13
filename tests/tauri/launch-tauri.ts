import { spawn, type ChildProcess } from 'child_process';

export function launchTauriDev(): ChildProcess {
  // Launches Tauri in dev mode from the src-tauri directory.
  // Uses TAURI_MODE=e2e to allow test-specific behavior if you want it in your Rust code.
  const proc = spawn('cargo', ['tauri', 'dev'], {
    cwd: 'src-tauri',
    env: {
      ...process.env,
      TAURI_MODE: 'e2e',
      RUST_BACKTRACE: '1'
    },
    stdio: ['ignore', 'pipe', 'pipe']
  });

  proc.stdout?.on('data', (data) => {
    const line = data.toString();
    if (line.toLowerCase().includes('error')) {
      console.error('[tauri]', line.trim());
    }
  });

  proc.stderr?.on('data', (data) => {
    console.error('[tauri:err]', data.toString().trim());
  });

  return proc;
}

export async function waitForTauriReady(proc: ChildProcess, timeoutMs = 60_000): Promise<void> {
  // Basic wait: in practice, key off a specific log line from your app like "E2E_READY".
  const start = Date.now();

  return new Promise((resolve, reject) => {
    const check = () => {
      if (Date.now() - start > timeoutMs) {
        reject(new Error('Timed out waiting for Tauri to be ready'));
        return;
      }
      if (proc.killed || proc.exitCode !== null) {
        reject(new Error('Tauri process exited before becoming ready'));
        return;
      }
      setTimeout(check, 1000);
    };
    check();
  });
}

export function stopTauri(proc: ChildProcess): void {
  if (!proc.killed) {
    proc.kill();
  }
}
