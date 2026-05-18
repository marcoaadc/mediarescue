import {
  HardDrive,
  Usb,
  FileArchive,
  CircuitBoard,
  HelpCircle,
} from 'lucide-react';
import { Card, CardContent } from '@/components/ui/card';
import { StatusBadge } from '@/components/common/StatusBadge';
import { ByteSize } from '@/components/common/ByteSize';
import { cn } from '@/utils/cn';
import type { DeviceInfo, DeviceType } from '@/types';

interface DeviceCardProps {
  device: DeviceInfo;
  isSelected: boolean;
  onSelect: (id: string) => void;
}

const deviceIcons: Record<DeviceType, typeof HardDrive> = {
  hard_drive: HardDrive,
  usb_drive: Usb,
  disk_image: FileArchive,
  ssd_drive: CircuitBoard,
  unknown: HelpCircle,
};

const deviceLabels: Record<DeviceType, string> = {
  hard_drive: 'HDD',
  usb_drive: 'USB',
  disk_image: 'Image',
  ssd_drive: 'SSD',
  unknown: 'Unknown',
};

export function DeviceCard({ device, isSelected, onSelect }: DeviceCardProps) {
  const Icon = deviceIcons[device.device_type];

  return (
    <Card
      className={cn(
        'cursor-pointer transition-colors',
        isSelected
          ? 'border-primary-500 bg-primary-600/5'
          : 'hover:border-neutral-700',
      )}
    >
      <CardContent className="p-4">
        <button
          type="button"
          onClick={() => onSelect(device.id)}
          className="flex w-full items-start gap-3 text-left focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-primary-500"
          aria-pressed={isSelected}
          aria-label={`Select device ${device.name}`}
        >
          <div
            className={cn(
              'flex h-10 w-10 shrink-0 items-center justify-center rounded-lg',
              isSelected ? 'bg-primary-600/20 text-primary-400' : 'bg-surface-800 text-neutral-400',
            )}
          >
            <Icon className="h-5 w-5" aria-hidden="true" />
          </div>

          <div className="min-w-0 flex-1">
            <div className="flex items-center gap-2">
              <p className="truncate font-medium text-neutral-100">{device.name}</p>
              {device.removable && (
                <StatusBadge variant="info">Removable</StatusBadge>
              )}
            </div>
            <p className="mt-0.5 truncate text-xs text-neutral-500">{device.path}</p>
            <div className="mt-2 flex items-center gap-3 text-xs text-neutral-400">
              <span>
                <ByteSize bytes={device.size_bytes} />
              </span>
              <span>{deviceLabels[device.device_type]}</span>
              <span>{device.sector_size}B sectors</span>
            </div>
          </div>
        </button>
      </CardContent>
    </Card>
  );
}
