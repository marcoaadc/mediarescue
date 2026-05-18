import { Disc3, Wifi, WifiOff } from 'lucide-react';
import { StatusBadge } from '@/components/common/StatusBadge';

interface HeaderProps {
  isConnected: boolean;
}

export function Header({ isConnected }: HeaderProps) {
  return (
    <header className="flex h-14 items-center justify-between border-b border-neutral-800 bg-surface-950 px-6">
      <div className="flex items-center gap-3">
        <Disc3 className="h-6 w-6 text-primary-400" />
        <h1 className="text-lg font-bold tracking-tight text-neutral-100">
          MediaRescue
        </h1>
      </div>

      <div className="flex items-center gap-4">
        <StatusBadge
          variant={isConnected ? 'success' : 'danger'}
          dot
        >
          {isConnected ? (
            <span className="flex items-center gap-1">
              <Wifi className="h-3 w-3" aria-hidden="true" />
              Connected
            </span>
          ) : (
            <span className="flex items-center gap-1">
              <WifiOff className="h-3 w-3" aria-hidden="true" />
              Disconnected
            </span>
          )}
        </StatusBadge>
      </div>
    </header>
  );
}
