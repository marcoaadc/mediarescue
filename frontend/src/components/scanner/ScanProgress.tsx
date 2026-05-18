import { ProgressBar } from '@/components/common/ProgressBar';
import { formatDuration, formatSpeed, formatNumber } from '@/utils/format';
import { useScanStore, useScanPercent } from '@/stores/scanStore';

export function ScanProgress() {
  const percent = useScanPercent();
  const sectorsDone = useScanStore((s) => s.sectorsDone);
  const sectorsTotal = useScanStore((s) => s.sectorsTotal);
  const signaturesFound = useScanStore((s) => s.signaturesFound);
  const speedMbps = useScanStore((s) => s.speedMbps);
  const etaSeconds = useScanStore((s) => s.etaSeconds);
  const status = useScanStore((s) => s.status);

  return (
    <div className="space-y-4">
      <div className="flex items-center justify-between">
        <h3 className="text-sm font-semibold text-neutral-200">Scan Progress</h3>
        <span className="font-mono text-sm text-primary-400">
          {percent.toFixed(1)}%
        </span>
      </div>

      <ProgressBar
        value={percent}
        variant={
          status === 'error'
            ? 'danger'
            : status === 'paused'
              ? 'warning'
              : status === 'completed'
                ? 'success'
                : 'primary'
        }
        size="lg"
      />

      <div className="grid grid-cols-2 gap-x-6 gap-y-2 text-sm lg:grid-cols-4">
        <div>
          <p className="text-neutral-500">Sectors</p>
          <p className="font-mono text-neutral-200">
            {formatNumber(sectorsDone)} / {formatNumber(sectorsTotal)}
          </p>
        </div>
        <div>
          <p className="text-neutral-500">Speed</p>
          <p className="font-mono text-neutral-200">{formatSpeed(speedMbps)}</p>
        </div>
        <div>
          <p className="text-neutral-500">ETA</p>
          <p className="font-mono text-neutral-200">
            {status === 'scanning' ? formatDuration(etaSeconds) : '--'}
          </p>
        </div>
        <div>
          <p className="text-neutral-500">Signatures Found</p>
          <p className="font-mono text-neutral-200">
            {formatNumber(signaturesFound)}
          </p>
        </div>
      </div>
    </div>
  );
}
