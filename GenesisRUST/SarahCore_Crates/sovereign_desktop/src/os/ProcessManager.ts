export type ProcessState = 'ready' | 'running' | 'terminated';

export class Process {
  pid: number;
  name: string;
  state: ProcessState = 'ready';
  task: Generator<any, void, any>;
  createdAt: number;

  constructor(pid: number, name: string, task: Generator<any, void, any>) {
    this.pid = pid;
    this.name = name;
    this.task = task;
    this.createdAt = Date.now();
  }
}

export class ProcessManager {
  processes: Map<number, Process> = new Map();
  readyQueue: Process[] = [];
  nextPid = 1;

  spawn(name: string, task: Generator<any, void, any>): number {
    const p = new Process(this.nextPid++, name, task);
    this.processes.set(p.pid, p);
    this.readyQueue.push(p);
    return p.pid;
  }

  kill(pid: number): boolean {
    const p = this.processes.get(pid);
    if (p) {
      p.state = 'terminated';
      this.processes.delete(pid);
      this.readyQueue = this.readyQueue.filter(proc => proc.pid !== pid);
      return true;
    }
    return false;
  }
}
