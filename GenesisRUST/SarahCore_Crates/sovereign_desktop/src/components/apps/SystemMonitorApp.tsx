import React, { useEffect, useState } from 'react';
import { kernel } from '../../os/Kernel';
import { Activity } from 'lucide-react';

export default function SystemMonitorApp() {
  const [processes, setProcesses] = useState<{pid: number, name: string, state: string, uptime: number}[]>([]);

  useEffect(() => {
    const interval = setInterval(() => {
      setProcesses(kernel.sys_ps());
    }, 500);
    return () => clearInterval(interval);
  }, []);

  return (
    <div className="h-full w-full bg-white dark:bg-gray-900 flex flex-col text-sm">
      <div className="h-12 bg-gray-100 dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700 flex items-center px-4 gap-2 text-gray-700 dark:text-gray-300">
        <Activity size={18} />
        <span className="font-medium">System Monitor</span>
      </div>
      <div className="flex-1 overflow-auto p-4">
        <table className="w-full text-left border-collapse">
          <thead>
            <tr className="border-b border-gray-200 dark:border-gray-700 text-gray-500 dark:text-gray-400">
              <th className="pb-2 font-medium">PID</th>
              <th className="pb-2 font-medium">Process Name</th>
              <th className="pb-2 font-medium">Status</th>
              <th className="pb-2 font-medium">Uptime</th>
              <th className="pb-2 font-medium text-right">Action</th>
            </tr>
          </thead>
          <tbody>
            {processes.map(p => (
              <tr key={p.pid} className="border-b border-gray-100 dark:border-gray-800/50 text-gray-800 dark:text-gray-200">
                <td className="py-2">{p.pid}</td>
                <td className="py-2 font-mono text-xs">{p.name}</td>
                <td className="py-2">
                  <span className={`px-2 py-0.5 rounded-full text-xs ${p.state === 'running' ? 'bg-green-100 text-green-700 dark:bg-green-900/30 dark:text-green-400' : 'bg-yellow-100 text-yellow-700 dark:bg-yellow-900/30 dark:text-yellow-400'}`}>
                    {p.state}
                  </span>
                </td>
                <td className="py-2 text-gray-500">{Math.floor(p.uptime / 1000)}s</td>
                <td className="py-2 text-right">
                  {p.name !== 'idle' && (
                    <button 
                      onClick={() => kernel.sys_kill(p.pid)}
                      className="text-red-500 hover:text-red-700 text-xs px-2 py-1 rounded border border-red-200 dark:border-red-900/50 hover:bg-red-50 dark:hover:bg-red-900/20"
                    >
                      Kill
                    </button>
                  )}
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
}
