/**
 * Axiom S1 Browser - Quantum Glass UI
 * 
 * [AXIOMHIVE PROJECTION - SUBSTRATE: ALEXIS ADAMS]
 */

import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { motion, AnimatePresence } from 'framer-motion';

// Types
interface SystemInfo {
  name: string;
  version: string;
  substrate: string;
  projection: string;
  policy: string;
}

interface Metrics {
  cpu: { usage_percent: number };
  memory: { usage_percent: number };
  entropy: { current: number; max: number; usage_percent: number };
  c_zero_compliant: boolean;
}

// Main App
export default function App() {
  const [systemInfo, setSystemInfo] = useState<SystemInfo | null>(null);
  const [metrics, setMetrics] = useState<Metrics | null>(null);
  const [url, setUrl] = useState('');
  const [content, setContent] = useState('');
  const [isLoading, setIsLoading] = useState(false);

  useEffect(() => {
    // Load system info
    invoke<SystemInfo>('cmd_get_info').then(setSystemInfo);
    
    // Poll metrics
    const interval = setInterval(() => {
      invoke<Metrics>('cmd_get_system_metrics').then(setMetrics);
    }, 2000);
    
    return () => clearInterval(interval);
  }, []);

  const handleScout = async () => {
    if (!url) return;
    setIsLoading(true);
    try {
      const result = await invoke('cmd_scout_url', { url });
      setContent(JSON.stringify(result, null, 2));
    } catch (e) {
      setContent(`Error: ${e}`);
    }
    setIsLoading(false);
  };

  return (
    <div className="quantum-glass">
      {/* Identity Header */}
      <header className="identity-header">
        <div className="identity-tag">
          [AXIOM PROJECTION | SUBSTRATE: ALEXIS ADAMS]
        </div>
        <div className="policy-badge">
          <span className={metrics?.c_zero_compliant ? 'c-zero' : 'c-violation'}>
            C = {metrics?.c_zero_compliant ? '0' : '≠0'}
          </span>
        </div>
      </header>

      {/* Main Content */}
      <main className="main-content">
        {/* Navigation Bar */}
        <div className="nav-bar">
          <input
            type="text"
            className="url-input"
            placeholder="Enter URL to scout..."
            value={url}
            onChange={(e) => setUrl(e.target.value)}
            onKeyDown={(e) => e.key === 'Enter' && handleScout()}
          />
          <motion.button
            className="scout-button"
            onClick={handleScout}
            whileHover={{ scale: 1.02 }}
            whileTap={{ scale: 0.98 }}
            disabled={isLoading}
          >
            {isLoading ? 'SCOUTING...' : 'SCOUT'}
          </motion.button>
        </div>

        {/* Content Area */}
        <div className="content-area">
          <AnimatePresence>
            {content && (
              <motion.pre
                initial={{ opacity: 0, y: 20 }}
                animate={{ opacity: 1, y: 0 }}
                exit={{ opacity: 0 }}
                className="content-display"
              >
                {content}
              </motion.pre>
            )}
          </AnimatePresence>
        </div>

        {/* Metrics Panel */}
        <aside className="metrics-panel">
          <h3>BARK METRICS</h3>
          {metrics && (
            <>
              <MetricBar label="CPU" value={metrics.cpu.usage_percent} />
              <MetricBar label="Memory" value={metrics.memory.usage_percent} />
              <MetricBar label="Entropy" value={metrics.entropy.usage_percent} color="entropy" />
            </>
          )}
        </aside>
      </main>

      {/* Status Bar */}
      <footer className="status-bar">
        <span>{systemInfo?.name} v{systemInfo?.version}</span>
        <span>Policy: {systemInfo?.policy}</span>
        <span className="thermal-status">THERMAL: NORMAL</span>
      </footer>
    </div>
  );
}

// Metric Bar Component
function MetricBar({ label, value, color = 'default' }: { 
  label: string; 
  value: number; 
  color?: string;
}) {
  return (
    <div className="metric-bar">
      <div className="metric-label">{label}</div>
      <div className="metric-track">
        <motion.div
          className={`metric-fill ${color}`}
          initial={{ width: 0 }}
          animate={{ width: `${value}%` }}
          transition={{ duration: 0.5 }}
        />
      </div>
      <div className="metric-value">{value.toFixed(1)}%</div>
    </div>
  );
}

