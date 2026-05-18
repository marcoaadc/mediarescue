import { useQuery } from '@tanstack/react-query';
import { DeviceSelector } from '@/components/dashboard/DeviceSelector';
import { QuickStats } from '@/components/dashboard/QuickStats';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { StatusBadge } from '@/components/common/StatusBadge';
import { useDeviceStore } from '@/stores/deviceStore';
import { listDevices, refreshDevices, getScanHistory } from '@/api/endpoints';
import type { ScanSummary } from '@/types';

export function DashboardPage() {
  const { setDevices, setLoading } = useDeviceStore();

  const devicesQuery = useQuery({
    queryKey: ['devices'],
    queryFn: async () => {
      setLoading(true);
      try {
        const res = await listDevices();
        setDevices(res.data);
        return res.data;
      } finally {
        setLoading(false);
      }
    },
  });

  const historyQuery = useQuery({
    queryKey: ['scan-history'],
    queryFn: async () => {
      const res = await getScanHistory();
      return res.data;
    },
  });

  async function handleRefresh() {
    setLoading(true);
    try {
      const res = await refreshDevices();
      setDevices(res.data);
    } finally {
      setLoading(false);
    }
    await devicesQuery.refetch();
  }

  const scans = historyQuery.data ?? [];
  const totalScans = scans.length;
  const filesRecovered = scans.reduce((sum, s) => sum + s.files_recovered, 0);
  const successRate =
    totalScans > 0
      ? (scans.filter((s) => s.status === 'completed').length / totalScans) * 100
      : 0;
  const lastScan = scans.length > 0 ? formatDate(scans[0]!) : null;

  return (
    <div className="space-y-8">
      <div>
        <h1 className="text-2xl font-bold text-neutral-100">Dashboard</h1>
        <p className="mt-1 text-sm text-neutral-500">
          Select a device and start recovering your media files.
        </p>
      </div>

      <QuickStats
        totalScans={totalScans}
        filesRecovered={filesRecovered}
        successRate={successRate}
        lastScanDate={lastScan}
      />

      <DeviceSelector
        devices={devicesQuery.data ?? []}
        isLoading={devicesQuery.isLoading}
        onRefresh={handleRefresh}
      />

      {/* Recent scans */}
      <Card>
        <CardHeader>
          <CardTitle>Recent Scans</CardTitle>
        </CardHeader>
        <CardContent>
          {scans.length === 0 ? (
            <p className="py-4 text-center text-sm text-neutral-600">
              No scan history yet. Select a device and start your first scan.
            </p>
          ) : (
            <div className="overflow-auto">
              <table className="w-full text-left text-sm">
                <thead className="border-b border-neutral-800 text-xs text-neutral-500">
                  <tr>
                    <th scope="col" className="px-3 py-2 font-medium">Scan ID</th>
                    <th scope="col" className="px-3 py-2 font-medium">Date</th>
                    <th scope="col" className="px-3 py-2 font-medium">Found</th>
                    <th scope="col" className="px-3 py-2 font-medium">Recovered</th>
                    <th scope="col" className="px-3 py-2 font-medium">Status</th>
                  </tr>
                </thead>
                <tbody className="divide-y divide-neutral-800/50">
                  {scans.slice(0, 10).map((scan) => (
                    <tr key={scan.scan_id} className="hover:bg-surface-800/50">
                      <td className="px-3 py-2 font-mono text-neutral-400">
                        {scan.scan_id.slice(0, 8)}
                      </td>
                      <td className="px-3 py-2 text-neutral-400">
                        {formatDate(scan)}
                      </td>
                      <td className="px-3 py-2 text-neutral-300">
                        {scan.files_found}
                      </td>
                      <td className="px-3 py-2 text-neutral-300">
                        {scan.files_recovered}
                      </td>
                      <td className="px-3 py-2">
                        <ScanStatusBadge status={scan.status} />
                      </td>
                    </tr>
                  ))}
                </tbody>
              </table>
            </div>
          )}
        </CardContent>
      </Card>
    </div>
  );
}

function formatDate(scan: ScanSummary): string {
  const d = scan.completed_at ?? scan.started_at;
  return new Date(d).toLocaleDateString('en-US', {
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
  });
}

function ScanStatusBadge({ status }: { status: string }) {
  const variantMap: Record<string, 'success' | 'info' | 'danger' | 'warning' | 'muted'> = {
    completed: 'success',
    scanning: 'info',
    error: 'danger',
    cancelled: 'warning',
  };
  const variant = variantMap[status] ?? 'muted';

  return (
    <StatusBadge variant={variant} dot>
      {status}
    </StatusBadge>
  );
}
