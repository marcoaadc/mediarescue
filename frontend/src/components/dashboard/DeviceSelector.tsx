import { RefreshCw, HardDrive } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { DeviceCard } from './DeviceCard';
import { useDeviceStore } from '@/stores/deviceStore';
import { cn } from '@/utils/cn';
import type { DeviceInfo } from '@/types';

interface DeviceSelectorProps {
  devices: DeviceInfo[];
  isLoading: boolean;
  onRefresh: () => void;
}

export function DeviceSelector({
  devices,
  isLoading,
  onRefresh,
}: DeviceSelectorProps) {
  const { selectedDeviceId, selectDevice } = useDeviceStore();

  return (
    <section aria-labelledby="device-selector-heading">
      <div className="mb-4 flex items-center justify-between">
        <h2
          id="device-selector-heading"
          className="text-base font-semibold text-neutral-100"
        >
          Available Devices
        </h2>
        <Button
          variant="outline"
          size="sm"
          onClick={onRefresh}
          disabled={isLoading}
          aria-label="Refresh device list"
        >
          <RefreshCw
            className={cn('h-4 w-4', isLoading && 'animate-spin')}
            aria-hidden="true"
          />
          Refresh
        </Button>
      </div>

      {devices.length === 0 ? (
        <div className="flex flex-col items-center gap-3 rounded-lg border border-dashed border-neutral-800 py-12 text-center">
          <HardDrive className="h-10 w-10 text-neutral-600" />
          <div>
            <p className="font-medium text-neutral-400">No devices found</p>
            <p className="mt-1 text-sm text-neutral-600">
              Connect a drive or mount a disk image, then refresh.
            </p>
          </div>
        </div>
      ) : (
        <div className="grid gap-3 sm:grid-cols-2 lg:grid-cols-3">
          {devices.map((device) => (
            <DeviceCard
              key={device.id}
              device={device}
              isSelected={selectedDeviceId === device.id}
              onSelect={selectDevice}
            />
          ))}
        </div>
      )}
    </section>
  );
}
