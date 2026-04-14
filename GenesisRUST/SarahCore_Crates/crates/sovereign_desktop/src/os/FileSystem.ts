export type NodeType = 'file' | 'dir';

export interface VNode {
  name: string;
  type: NodeType;
  content: string;
  children: Map<string, VNode>;
  parent: VNode | null;
}

export class FileSystem {
  root: VNode;

  constructor() {
    this.root = { name: '/', type: 'dir', content: '', children: new Map(), parent: null };
  }

  normalizePath(currentDir: string, targetPath: string): string {
    const path = targetPath.startsWith('/') ? targetPath : `${currentDir}/${targetPath}`;
    const parts = path.split('/').filter(p => p !== '' && p !== '.');
    const stack: string[] = [];
    for (const p of parts) {
      if (p === '..') stack.pop();
      else stack.push(p);
    }
    return '/' + stack.join('/');
  }

  private getNode(path: string): VNode | null {
    if (path === '/') return this.root;
    const parts = path.split('/').filter(p => p !== '');
    let current = this.root;
    for (const part of parts) {
      if (current.type !== 'dir') return null;
      const next = current.children.get(part);
      if (!next) return null;
      current = next;
    }
    return current;
  }

  mkdir(path: string): boolean {
    if (path === '/') return false;
    const parts = path.split('/').filter(p => p !== '');
    const name = parts.pop()!;
    const parentPath = '/' + parts.join('/');
    const parent = this.getNode(parentPath);
    
    if (!parent || parent.type !== 'dir' || parent.children.has(name)) return false;
    
    parent.children.set(name, {
      name,
      type: 'dir',
      content: '',
      children: new Map(),
      parent
    });
    return true;
  }

  writeFile(path: string, content: string): boolean {
    const parts = path.split('/').filter(p => p !== '');
    const name = parts.pop()!;
    const parentPath = '/' + parts.join('/');
    const parent = this.getNode(parentPath);
    
    if (!parent || parent.type !== 'dir') return false;
    
    const existing = parent.children.get(name);
    if (existing && existing.type === 'dir') return false;

    if (existing) {
      existing.content = content;
    } else {
      parent.children.set(name, {
        name,
        type: 'file',
        content,
        children: new Map(),
        parent
      });
    }
    return true;
  }

  readFile(path: string): string | null {
    const node = this.getNode(path);
    if (!node || node.type !== 'file') return null;
    return node.content;
  }

  readDir(path: string): string[] | null {
    const node = this.getNode(path);
    if (!node || node.type !== 'dir') return null;
    return Array.from(node.children.keys());
  }

  rm(path: string): boolean {
    if (path === '/') return false;
    const parts = path.split('/').filter(p => p !== '');
    const name = parts.pop()!;
    const parentPath = '/' + parts.join('/');
    const parent = this.getNode(parentPath);
    
    if (!parent || parent.type !== 'dir') return false;
    return parent.children.delete(name);
  }
}
