import { create } from 'zustand';
import type { ScanStatus, RecoveredFile } from '@/types';

interface ScanState {
  scanId: string | null;
  status: ScanStatus;
  sectorsDone: number;
  sectorsTotal: number;
  signaturesFound: number;
  speedMbps: number;
  etaSeconds: number;
  files: RecoveredFile[];
  error: string | null;

  setScanId: (id: string | null) => void;
  setStatus: (status: ScanStatus) => void;
  updateProgress: (progress: {
    sectors_done: number;
    sectors_total: number;
    signatures_found: number;
    speed_mbps: number;
  }) => void;
  addFile: (file: RecoveredFile) => void;
  updateFile: (fileId: string, updates: Partial<RecoveredFile>) => void;
  setFiles: (files: RecoveredFile[]) => void;
  setError: (error: string | null) => void;
  reset: () => void;
}

const initialState = {
  scanId: null,
  status: 'idle' as ScanStatus,
  sectorsDone: 0,
  sectorsTotal: 0,
  signaturesFound: 0,
  speedMbps: 0,
  etaSeconds: 0,
  files: [],
  error: null,
};

export const useScanStore = create<ScanState>((set) => ({
  ...initialState,

  setScanId: (scanId) => set({ scanId }),
  setStatus: (status) => set({ status }),

  updateProgress: (progress) => {
    const remaining = progress.sectors_total - progress.sectors_done;
    const etaSeconds =
      progress.speed_mbps > 0
        ? (remaining * 512) / (progress.speed_mbps * 1_000_000)
        : 0;

    set({
      sectorsDone: progress.sectors_done,
      sectorsTotal: progress.sectors_total,
      signaturesFound: progress.signatures_found,
      speedMbps: progress.speed_mbps,
      etaSeconds,
    });
  },

  addFile: (file) =>
    set((state) => ({ files: [...state.files, file] })),

  updateFile: (fileId, updates) =>
    set((state) => ({
      files: state.files.map((f) =>
        f.id === fileId ? { ...f, ...updates } : f,
      ),
    })),

  setFiles: (files) => set({ files }),
  setError: (error) => set({ error, status: error ? 'error' : 'idle' }),
  reset: () => set(initialState),
}));

/** Derived selector: scan progress as 0-100 */
export const useScanPercent = () =>
  useScanStore((state) =>
    state.sectorsTotal > 0
      ? (state.sectorsDone / state.sectorsTotal) * 100
      : 0,
  );
