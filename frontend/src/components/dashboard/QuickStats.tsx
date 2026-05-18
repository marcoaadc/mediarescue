import { ScanSearch, FileCheck, TrendingUp, Clock } from 'lucide-react';
import { Card, CardContent } from '@/components/ui/card';
import { formatNumber, formatPercent } from '@/utils/format';

interface QuickStatsProps {
  totalScans: number;
  filesRecovered: number;
  successRate: number;
  lastScanDate: string | null;
}

export function QuickStats({
  totalScans,
  filesRecovered,
  successRate,
  lastScanDate,
}: QuickStatsProps) {
  const stats = [
    {
      label: 'Total Scans',
      value: formatNumber(totalScans),
      icon: ScanSearch,
      color: 'text-primary-400',
    },
    {
      label: 'Files Recovered',
      value: formatNumber(filesRecovered),
      icon: FileCheck,
      color: 'text-success-400',
    },
    {
      label: 'Success Rate',
      value: formatPercent(successRate),
      icon: TrendingUp,
      color: 'text-warning-400',
    },
    {
      label: 'Last Scan',
      value: lastScanDate ?? 'Never',
      icon: Clock,
      color: 'text-neutral-400',
    },
  ];

  return (
    <div className="grid gap-3 sm:grid-cols-2 lg:grid-cols-4">
      {stats.map((stat) => (
        <Card key={stat.label}>
          <CardContent className="flex items-center gap-4 p-4">
            <div className="flex h-10 w-10 shrink-0 items-center justify-center rounded-lg bg-surface-800">
              <stat.icon
                className={`h-5 w-5 ${stat.color}`}
                aria-hidden="true"
              />
            </div>
            <div>
              <p className="text-xs font-medium text-neutral-500">
                {stat.label}
              </p>
              <p className="text-lg font-semibold text-neutral-100">
                {stat.value}
              </p>
            </div>
          </CardContent>
        </Card>
      ))}
    </div>
  );
}
