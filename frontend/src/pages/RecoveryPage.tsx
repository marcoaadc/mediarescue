import { useState, useMemo, useCallback } from 'react';
import { Download } from 'lucide-react';
import { Button } from '@/components/ui/button';
import {
  RepairOptions,
  type RepairSettings,
} from '@/components/recovery/RepairOptions';
import {
  RepairProgress,
  type RepairItem,
} from '@/components/recovery/RepairProgress';
import { ExportDialog } from '@/components/recovery/ExportDialog';
import { useScanStore } from '@/stores/scanStore';
import { repairFile, exportFiles } from '@/api/endpoints';

export function RecoveryPage() {
  const files = useScanStore((s) => s.files);
  const repairableFiles = useMemo(
    () => files.filter((f) => f.can_repair),
    [files],
  );

  const [repairSettings, setRepairSettings] = useState<RepairSettings>({
    fixHeaders: true,
    reconstructData: true,
    validateAfterRepair: true,
  });

  const [repairItems, setRepairItems] = useState<RepairItem[]>([]);
  const [isRepairing, setIsRepairing] = useState(false);
  const [showExport, setShowExport] = useState(false);
  const [isExporting, setIsExporting] = useState(false);

  const handleStartRepair = useCallback(async () => {
    if (repairableFiles.length === 0) return;

    const items: RepairItem[] = repairableFiles.map((file) => ({
      file,
      progress: 0,
      status: 'pending' as const,
    }));
    setRepairItems(items);
    setIsRepairing(true);

    for (let i = 0; i < items.length; i++) {
      const item = items[i]!;
      setRepairItems((prev) =>
        prev.map((p, idx) =>
          idx === i ? { ...p, status: 'repairing', progress: 50 } : p,
        ),
      );

      try {
        await repairFile(item.file.id, {
          fix_headers: repairSettings.fixHeaders,
          reconstruct_data: repairSettings.reconstructData,
        });
        setRepairItems((prev) =>
          prev.map((p, idx) =>
            idx === i ? { ...p, status: 'completed', progress: 100 } : p,
          ),
        );
      } catch {
        setRepairItems((prev) =>
          prev.map((p, idx) =>
            idx === i ? { ...p, status: 'failed', progress: 100 } : p,
          ),
        );
      }
    }

    setIsRepairing(false);
  }, [repairableFiles, repairSettings]);

  const handleExport = useCallback(
    async (outputDir: string) => {
      setIsExporting(true);
      try {
        const fileIds = files.filter((f) => f.status === 'valid').map((f) => f.id);
        await exportFiles(fileIds, outputDir);
      } finally {
        setIsExporting(false);
        setShowExport(false);
      }
    },
    [files],
  );

  const validCount = files.filter((f) => f.status === 'valid').length;

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-2xl font-bold text-neutral-100">Recovery</h1>
          <p className="mt-1 text-sm text-neutral-500">
            Repair damaged files and export recovered data.
          </p>
        </div>
        <div className="flex items-center gap-2">
          <Button
            onClick={handleStartRepair}
            disabled={repairableFiles.length === 0 || isRepairing}
          >
            {isRepairing ? 'Repairing...' : `Repair ${repairableFiles.length} Files`}
          </Button>
          <Button
            variant="outline"
            onClick={() => setShowExport(true)}
            disabled={validCount === 0}
          >
            <Download className="h-4 w-4" aria-hidden="true" />
            Export ({validCount})
          </Button>
        </div>
      </div>

      <div className="grid gap-6 lg:grid-cols-2">
        <RepairOptions
          settings={repairSettings}
          onChange={setRepairSettings}
          disabled={isRepairing}
        />
        <RepairProgress items={repairItems} />
      </div>

      <ExportDialog
        open={showExport}
        onClose={() => setShowExport(false)}
        onExport={handleExport}
        fileCount={validCount}
        isExporting={isExporting}
      />
    </div>
  );
}
