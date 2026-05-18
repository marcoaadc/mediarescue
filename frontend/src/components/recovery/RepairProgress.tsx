import { ProgressBar } from '@/components/common/ProgressBar';
import { StatusBadge } from '@/components/common/StatusBadge';
import { formatMediaType } from '@/utils/format';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import type { RecoveredFile } from '@/types';

interface RepairItem {
  file: RecoveredFile;
  progress: number;
  status: 'pending' | 'repairing' | 'completed' | 'failed';
}

interface RepairProgressProps {
  items: RepairItem[];
}

const statusVariant = {
  pending: 'muted' as const,
  repairing: 'info' as const,
  completed: 'success' as const,
  failed: 'danger' as const,
};

const progressVariant = {
  pending: 'primary' as const,
  repairing: 'primary' as const,
  completed: 'success' as const,
  failed: 'danger' as const,
};

export function RepairProgress({ items }: RepairProgressProps) {
  if (items.length === 0) {
    return (
      <Card>
        <CardContent className="py-8 text-center text-neutral-500">
          Select files and repair options to begin.
        </CardContent>
      </Card>
    );
  }

  const completed = items.filter((i) => i.status === 'completed').length;
  const totalProgress =
    items.length > 0
      ? items.reduce((sum, i) => sum + i.progress, 0) / items.length
      : 0;

  return (
    <Card>
      <CardHeader>
        <div className="flex items-center justify-between">
          <CardTitle>Repair Progress</CardTitle>
          <span className="text-xs text-neutral-500">
            {completed} / {items.length} completed
          </span>
        </div>
        <ProgressBar
          value={totalProgress}
          variant="primary"
          size="sm"
          showPercent
        />
      </CardHeader>
      <CardContent>
        <div className="max-h-64 space-y-3 overflow-auto">
          {items.map((item) => (
            <div
              key={item.file.id}
              className="flex items-center gap-3 text-sm"
            >
              <StatusBadge variant={statusVariant[item.status]}>
                {formatMediaType(item.file.format)}
              </StatusBadge>
              <span className="min-w-0 flex-1 truncate font-mono text-xs text-neutral-400">
                {item.file.id.slice(0, 12)}
              </span>
              <div className="w-32">
                <ProgressBar
                  value={item.progress}
                  variant={progressVariant[item.status]}
                  size="sm"
                />
              </div>
              <StatusBadge variant={statusVariant[item.status]}>
                {item.status}
              </StatusBadge>
            </div>
          ))}
        </div>
      </CardContent>
    </Card>
  );
}

export type { RepairItem };
