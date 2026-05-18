import { Play, Pause, StopCircle, RotateCcw } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { useScanStore } from '@/stores/scanStore';
import type { ScanStatus } from '@/types';

interface ScanControlsProps {
  onStart: () => void;
  onPause: () => void;
  onResume: () => void;
  onCancel: () => void;
  disabled?: boolean;
}

export function ScanControls({
  onStart,
  onPause,
  onResume,
  onCancel,
  disabled = false,
}: ScanControlsProps) {
  const status = useScanStore((s) => s.status);

  return (
    <div className="flex items-center gap-2" role="group" aria-label="Scan controls">
      {renderControls(status, { onStart, onPause, onResume, onCancel, disabled })}
    </div>
  );
}

function renderControls(
  status: ScanStatus,
  actions: ScanControlsProps,
) {
  switch (status) {
    case 'idle':
    case 'completed':
    case 'cancelled':
    case 'error':
      return (
        <Button onClick={actions.onStart} disabled={actions.disabled}>
          {status === 'idle' ? (
            <Play className="h-4 w-4" aria-hidden="true" />
          ) : (
            <RotateCcw className="h-4 w-4" aria-hidden="true" />
          )}
          {status === 'idle' ? 'Start Scan' : 'New Scan'}
        </Button>
      );

    case 'scanning':
      return (
        <>
          <Button variant="secondary" onClick={actions.onPause}>
            <Pause className="h-4 w-4" aria-hidden="true" />
            Pause
          </Button>
          <Button variant="destructive" onClick={actions.onCancel}>
            <StopCircle className="h-4 w-4" aria-hidden="true" />
            Cancel
          </Button>
        </>
      );

    case 'paused':
      return (
        <>
          <Button onClick={actions.onResume}>
            <Play className="h-4 w-4" aria-hidden="true" />
            Resume
          </Button>
          <Button variant="destructive" onClick={actions.onCancel}>
            <StopCircle className="h-4 w-4" aria-hidden="true" />
            Cancel
          </Button>
        </>
      );
  }
}
