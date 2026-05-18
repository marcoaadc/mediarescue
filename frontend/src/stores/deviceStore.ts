import { create } from 'zustand';
import type { DeviceInfo } from '@/types';

interface DeviceState {
  devices: DeviceInfo[];
  selectedDeviceId: string | null;
  isLoading: boolean;
  error: string | null;

  setDevices: (devices: DeviceInfo[]) => void;
  selectDevice: (deviceId: string | null) => void;
  setLoading: (loading: boolean) => void;
  setError: (error: string | null) => void;
}

export const useDeviceStore = create<DeviceState>((set) => ({
  devices: [],
  selectedDeviceId: null,
  isLoading: false,
  error: null,

  setDevices: (devices) => set({ devices, error: null }),
  selectDevice: (deviceId) => set({ selectedDeviceId: deviceId }),
  setLoading: (isLoading) => set({ isLoading }),
  setError: (error) => set({ error }),
}));

/** Derived selector: get the full DeviceInfo for the currently selected device */
export const useSelectedDevice = () =>
  useDeviceStore((state) =>
    state.devices.find((d) => d.id === state.selectedDeviceId) ?? null,
  );
