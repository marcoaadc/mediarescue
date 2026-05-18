import { useCallback } from 'react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { ScanProgress } from '@/components/scanner/ScanProgress';
import { ScanControls } from '@/components/scanner/ScanControls';
import { SectorMap } from '@/components/scanner/SectorMap';
import { SignatureHits } from '@/components/scanner/SignatureHits';
import { useScanStore } from '@/stores/scanStore';
import { useSelectedDevice } from '@/stores/deviceStore';
import { useSettingsStore } from '@/stores/settingsStore';
import { startScan, pauseScan, resumeScan, cancelScan } from '@/api/endpoints';

export function ScannerPage() {
  const selectedDevice = useSelectedDevice();
  const scanId = useScanStore((s) => s.scanId);
  const status = useScanStore((s) => s.status);
  const scanError = useScanStore((s) => s.error);
  const { setScanId, setStatus, setError, reset } = useScanStore();
  const { scanDepth, enabledFormats, maxWorkers, sectorRetryCount } =
    useSettingsStore();

  const handleStart = useCallback(async () => {
    if (!selectedDevice) return;

    reset();
    setStatus('scanning');

    try {
      const res = await startScan(selectedDevice.id, {
        formats: enabledFormats,
        depth: scanDepth,
        max_workers: maxWorkers,
        sector_retry_count: sectorRetryCount,
      });
      setScanId(res.data.scan_id);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to start scan');
    }
  }, [
    selectedDevice,
    enabledFormats,
    scanDepth,
    maxWorkers,
    sectorRetryCount,
    reset,
    setStatus,
    setScanId,
    setError,
  ]);

  const handlePause = useCallback(async () => {
    if (!scanId) return;
    try {
      await pauseScan(scanId);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to pause scan');
    }
  }, [scanId, setError]);

  const handleResume = useCallback(async () => {
    if (!scanId) return;
    try {
      await resumeScan(scanId);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to resume scan');
    }
  }, [scanId, setError]);

  const handleCancel = useCallback(async () => {
    if (!scanId) return;
    try {
      await cancelScan(scanId);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to cancel scan');
    }
  }, [scanId, setError]);

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-neutral-100">Scanner</h1>
          {selectedDevice ? (
            <p className="mt-1 text-sm text-neutral-500">
              Scanning: {selectedDevice.name} ({selectedDevice.path})
            </p>
          ) : (
            <p className="mt-1 text-sm text-warning-400">
              No device selected. Go to Dashboard to select a device.
            </p>
          )}
        </div>
        <ScanControls
          onStart={handleStart}
          onPause={handlePause}
          onResume={handleResume}
          onCancel={handleCancel}
          disabled={!selectedDevice}
        />
      </div>

      {/* Error banner */}
      {status === 'error' && (
        <div className="rounded-md border border-danger-500/30 bg-danger-500/5 p-4 text-sm text-danger-400">
          {scanError ?? 'An error occurred during the scan.'}
        </div>
      )}

      {/* Progress and sector map */}
      <div className="grid gap-6 lg:grid-cols-3">
        <Card className="lg:col-span-2">
          <CardHeader>
            <CardTitle>Progress</CardTitle>
          </CardHeader>
          <CardContent>
            <ScanProgress />
          </CardContent>
        </Card>

        <Card>
          <CardContent className="pt-5">
            <SectorMap />
          </CardContent>
        </Card>
      </div>

      {/* Signature hits */}
      <Card>
        <CardContent className="pt-5">
          <SignatureHits />
        </CardContent>
      </Card>
    </div>
  );
}
