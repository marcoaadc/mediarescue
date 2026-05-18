import { get, post, put } from './client';
import type {
  DeviceInfo,
  RecoveredFile,
  ScanConfig,
  ScanSummary,
  AppSettings,
  ApiResponse,
} from '@/types';

// ─── Devices ─────────────────────────────────────────────────────────────────

export function listDevices(): Promise<ApiResponse<DeviceInfo[]>> {
  return get('/devices');
}

export function getDevice(id: string): Promise<ApiResponse<DeviceInfo>> {
  return get(`/devices/${id}`);
}

export function refreshDevices(): Promise<ApiResponse<DeviceInfo[]>> {
  return post('/devices/refresh');
}

// ─── Scan ────────────────────────────────────────────────────────────────────

export function startScan(
  deviceId: string,
  config: ScanConfig,
): Promise<ApiResponse<{ scan_id: string }>> {
  return post(`/scan/start`, { device_id: deviceId, config });
}

export function pauseScan(scanId: string): Promise<ApiResponse<null>> {
  return post(`/scan/${scanId}/pause`);
}

export function resumeScan(scanId: string): Promise<ApiResponse<null>> {
  return post(`/scan/${scanId}/resume`);
}

export function cancelScan(scanId: string): Promise<ApiResponse<null>> {
  return post(`/scan/${scanId}/cancel`);
}

export function getScanHistory(): Promise<ApiResponse<ScanSummary[]>> {
  return get('/scan/history');
}

// ─── Files ───────────────────────────────────────────────────────────────────

export function listRecoveredFiles(
  scanId: string,
): Promise<ApiResponse<RecoveredFile[]>> {
  return get(`/scan/${scanId}/files`);
}

export function getFileThumbnail(fileId: string): string {
  return `/api/files/${fileId}/thumbnail`;
}

export function getFilePreview(fileId: string): string {
  return `/api/files/${fileId}/preview`;
}

// ─── Recovery ────────────────────────────────────────────────────────────────

export function repairFile(
  fileId: string,
  options: { fix_headers: boolean; reconstruct_data: boolean },
): Promise<ApiResponse<RecoveredFile>> {
  return post(`/files/${fileId}/repair`, options);
}

export function exportFiles(
  fileIds: string[],
  outputDir: string,
): Promise<ApiResponse<{ exported: number; failed: number }>> {
  return post('/files/export', { file_ids: fileIds, output_dir: outputDir });
}

// ─── Settings ────────────────────────────────────────────────────────────────

export function getSettings(): Promise<ApiResponse<AppSettings>> {
  return get('/settings');
}

export function updateSettings(
  settings: Partial<AppSettings>,
): Promise<ApiResponse<AppSettings>> {
  return put('/settings', settings);
}
