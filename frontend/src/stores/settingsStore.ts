import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import type { ScanDepth, MediaFormat } from '@/types';

interface SettingsState {
  scanDepth: ScanDepth;
  enabledFormats: MediaFormat[];
  maxWorkers: number;
  sectorRetryCount: number;
  outputDirectory: string;

  setScanDepth: (depth: ScanDepth) => void;
  toggleFormat: (format: MediaFormat) => void;
  setEnabledFormats: (formats: MediaFormat[]) => void;
  setMaxWorkers: (workers: number) => void;
  setSectorRetryCount: (count: number) => void;
  setOutputDirectory: (dir: string) => void;
}

const ALL_FORMATS: MediaFormat[] = [
  'jpeg', 'png', 'mp4', 'mov', 'avi', 'mkv', 'cr2', 'nef', 'arw',
];

export const useSettingsStore = create<SettingsState>()(
  persist(
    (set) => ({
      scanDepth: 'standard',
      enabledFormats: [...ALL_FORMATS],
      maxWorkers: 4,
      sectorRetryCount: 3,
      outputDirectory: '',

      setScanDepth: (scanDepth) => set({ scanDepth }),

      toggleFormat: (format) =>
        set((state) => ({
          enabledFormats: state.enabledFormats.includes(format)
            ? state.enabledFormats.filter((f) => f !== format)
            : [...state.enabledFormats, format],
        })),

      setEnabledFormats: (enabledFormats) => set({ enabledFormats }),
      setMaxWorkers: (maxWorkers) => set({ maxWorkers }),
      setSectorRetryCount: (sectorRetryCount) => set({ sectorRetryCount }),
      setOutputDirectory: (outputDirectory) => set({ outputDirectory }),
    }),
    { name: 'mediarescue-settings' },
  ),
);
