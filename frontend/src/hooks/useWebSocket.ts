import { useEffect, useRef, useCallback, useState } from 'react';
import { scanWs } from '@/api/websocket';
import { useScanStore } from '@/stores/scanStore';
import type { WsEvent, RecoveredFile } from '@/types';

export function useWebSocket() {
  const {
    updateProgress,
    setStatus,
    addFile,
    updateFile,
    setError,
  } = useScanStore();

  const [isConnected, setIsConnected] = useState(false);

  const handlersRef = useRef({ updateProgress, setStatus, addFile, updateFile, setError });
  handlersRef.current = { updateProgress, setStatus, addFile, updateFile, setError };

  const handleEvent = useCallback((event: WsEvent) => {
    const h = handlersRef.current;

    switch (event.type) {
      case 'scan_progress':
        h.updateProgress({
          sectors_done: event.sectors_done,
          sectors_total: event.sectors_total,
          signatures_found: event.signatures_found,
          speed_mbps: event.speed_mbps,
        });
        break;

      case 'file_discovered':
        h.addFile({
          id: event.file_id,
          format: event.format as RecoveredFile['format'],
          disk_offset: event.offset,
          size_bytes: event.estimated_size,
          recovery_score: 0,
          status: 'discovered',
          can_repair: false,
        });
        break;

      case 'file_recovered':
        h.updateFile(event.file_id, {
          recovery_score: event.score,
          status: 'valid',
        });
        break;

      case 'scan_complete':
        h.setStatus('completed');
        break;

      case 'scan_error':
        h.setError(event.message);
        break;

      case 'scan_paused':
        h.setStatus('paused');
        break;

      case 'scan_resumed':
        h.setStatus('scanning');
        break;

      case 'scan_cancelled':
        h.setStatus('cancelled');
        break;
    }
  }, []);

  useEffect(() => {
    scanWs.connect();
    const unsubscribe = scanWs.subscribe(handleEvent);

    const interval = setInterval(() => {
      setIsConnected(scanWs.isConnected);
    }, 1000);

    setIsConnected(scanWs.isConnected);

    return () => {
      clearInterval(interval);
      unsubscribe();
      scanWs.disconnect();
    };
  }, [handleEvent]);

  return { isConnected };
}
