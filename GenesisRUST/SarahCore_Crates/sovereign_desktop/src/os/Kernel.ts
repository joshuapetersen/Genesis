import { FileSystem } from './FileSystem';
import { ProcessManager } from './ProcessManager';

export class Kernel {
  fs: FileSystem;
  pm: ProcessManager;
  private schedulerInterval: number | null = null;

  constructor() {
    this.fs = new FileSystem();
    this.pm = new ProcessManager();
    this.initFS();
  }

  private initFS() {
    this.fs.mkdir('/home');
    this.fs.mkdir('/home/guest');
    this.fs.writeFile('/home/guest/readme.txt', 'Welcome to Web OS!\nThis is a monolithic kernel simulation.\nTry running "ls", "cat readme.txt", or "ps".');
    this.fs.mkdir('/bin');
    this.fs.mkdir('/etc');
    this.fs.mkdir('/var');
  }

  startScheduler() {
    if (this.schedulerInterval) return;
    // Run scheduler loop every 50ms (simulating CPU cycles for round-robin scheduling)
    this.schedulerInterval = window.setInterval(() => this.schedule(), 50);
  }

  private schedule() {
    if (this.pm.readyQueue.length === 0) return;

    // Round-robin: dequeue, run one step, enqueue if not done
    const p = this.pm.readyQueue.shift()!;
    if (p.state === 'terminated') return;

    p.state = 'running';
    try {
      const result = p.task.next();
      if (result.done) {
        p.state = 'terminated';
        this.pm.processes.delete(p.pid);
      } else {
        p.state = 'ready';
        this.pm.readyQueue.push(p);
      }
    } catch (e) {
      console.error(`Process ${p.pid} crashed:`, e);
      p.state = 'terminated';
      this.pm.processes.delete(p.pid);
    }
  }

  // --- Syscalls ---

  sys_spawn(name: string, task: Generator<any, void, any>) {
    return this.pm.spawn(name, task);
  }
  
  sys_kill(pid: number) {
    return this.pm.kill(pid);
  }

  sys_ps() {
    return Array.from(this.pm.processes.values()).map(p => ({
      pid: p.pid,
      name: p.name,
      state: p.state,
      uptime: Date.now() - p.createdAt
    }));
  }

  sys_read(path: string) { return this.fs.readFile(path); }
  sys_write(path: string, content: string) { return this.fs.writeFile(path, content); }
  sys_mkdir(path: string) { return this.fs.mkdir(path); }
  sys_rm(path: string) { return this.fs.rm(path); }
  sys_readdir(path: string) { return this.fs.readDir(path); }
  sys_resolve_path(cwd: string, path: string) { return this.fs.normalizePath(cwd, path); }
}

// Singleton instance representing the monolithic kernel
export const kernel = new Kernel();
kernel.startScheduler();

// Spawn an idle process just so there's always something in the scheduler
function* idleTask() {
  while (true) {
    yield;
  }
}
kernel.sys_spawn('idle', idleTask());
