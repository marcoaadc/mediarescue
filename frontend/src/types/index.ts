// ─── Device Types ────────────────────────────────────────────────────────────

export type DeviceType =
  | 'hard_drive'
  | 'usb_drive'
  | 'disk_image'
  | 'ssd_drive'
  | 'unknown';

export interface DeviceInfo {
  id: string;
  name: string;
  path: string;
  size_bytes: number;
  sector_size: number;
  total_sectors: number;
  device_type: DeviceType;
  removable: boolean;
}

// ─── Scan Types ──────────────────────────────────────────────────────────────

export type MediaFormat =
  | 'jpeg'
  | 'png'
  | 'mp4'
  | 'mov'
  | 'avi'
  | 'mkv'
  | 'cr2'
  | 'nef'
  | 'arw';

export type ScanDepth = 'quick' | 'standard' | 'deep';

export interface ScanConfig {
  formats: MediaFormat[];
  depth: ScanDepth;
  max_workers: number;
  sector_retry_count: number;
}

export type ScanStatus =
  | 'idle'
  | 'scanning'
  | 'paused'
  | 'completed'
  | 'cancelled'
  | 'error';

export interface ScanProgress {
  sectors_done: number;
  sectors_total: number;
  signatures_found: number;
  speed_mbps: number;
  eta_seconds: number;
  status: ScanStatus;
}

// ─── File Types ──────────────────────────────────────────────────────────────

export type FileStatus =
  | 'discovered'
  | 'carving'
  | 'carved'
  | 'reconstructing'
  | 'reconstructed'
  | 'validating'
  | 'valid'
  | 'invalid'
  | 'failed';

export interface RecoveredFile {
  id: string;
  format: MediaFormat;
  disk_offset: number;
  size_bytes: number;
  recovery_score: number;
  status: FileStatus;
  can_repair: boolean;
}

// ─── WebSocket Event Types ───────────────────────────────────────────────────

export type WsEvent =
  | {
      type: 'scan_progress';
      sectors_done: number;
      sectors_total: number;
      signatures_found: number;
      speed_mbps: number;
    }
  | {
      type: 'file_discovered';
      file_id: string;
      format: string;
      offset: number;
      estimated_size: number;
    }
  | {
      type: 'file_recovered';
      file_id: string;
      score: number;
    }
  | {
      type: 'scan_complete';
      total_found: number;
      total_recovered: number;
      duration_secs: number;
    }
  | {
      type: 'scan_error';
      message: string;
      recoverable: boolean;
    }
  | { type: 'scan_paused' }
  | { type: 'scan_resumed' }
  | { type: 'scan_cancelled' };

// ─── API Response Types ──────────────────────────────────────────────────────

export interface ApiResponse<T> {
  data: T;
  success: boolean;
  error?: string;
}

export interface ScanSummary {
  scan_id: string;
  device_id: string;
  started_at: string;
  completed_at: string | null;
  files_found: number;
  files_recovered: number;
  status: ScanStatus;
}

// ─── Settings Types ──────────────────────────────────────────────────────────

export interface AppSettings {
  scan_depth: ScanDepth;
  enabled_formats: MediaFormat[];
  max_workers: number;
  sector_retry_count: number;
  output_directory: string;
}
