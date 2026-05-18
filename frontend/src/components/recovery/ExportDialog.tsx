import { useState } from 'react';
import { Download, FolderOpen } from 'lucide-react';
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
  DialogFooter,
} from '@/components/ui/dialog';
import { Button } from '@/components/ui/button';
import { useSettingsStore } from '@/stores/settingsStore';

interface ExportDialogProps {
  open: boolean;
  onClose: () => void;
  onExport: (outputDir: string) => void;
  fileCount: number;
  isExporting: boolean;
}

export function ExportDialog({
  open,
  onClose,
  onExport,
  fileCount,
  isExporting,
}: ExportDialogProps) {
  const savedOutputDir = useSettingsStore((s) => s.outputDirectory);
  const [outputDir, setOutputDir] = useState(savedOutputDir);

  function handleExport() {
    if (outputDir.trim()) {
      onExport(outputDir.trim());
    }
  }

  return (
    <Dialog open={open} onOpenChange={(o) => !o && onClose()}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Export Recovered Files</DialogTitle>
          <DialogDescription>
            Export {fileCount} recovered file{fileCount !== 1 ? 's' : ''} to a
            local directory.
          </DialogDescription>
        </DialogHeader>

        <div>
          <label
            htmlFor="output-dir"
            className="mb-1.5 block text-sm font-medium text-neutral-300"
          >
            Output Directory
          </label>
          <div className="flex gap-2">
            <input
              id="output-dir"
              type="text"
              value={outputDir}
              onChange={(e) => setOutputDir(e.target.value)}
              placeholder="/path/to/output"
              className="flex-1 rounded-md border border-neutral-700 bg-surface-800 px-3 py-2 text-sm text-neutral-200 placeholder-neutral-600 focus:border-primary-500 focus:outline-none focus:ring-1 focus:ring-primary-500"
            />
            <Button variant="outline" size="icon" aria-label="Browse folder">
              <FolderOpen className="h-4 w-4" />
            </Button>
          </div>
        </div>

        <DialogFooter>
          <Button variant="ghost" onClick={onClose} disabled={isExporting}>
            Cancel
          </Button>
          <Button
            onClick={handleExport}
            disabled={!outputDir.trim() || isExporting}
          >
            <Download className="h-4 w-4" aria-hidden="true" />
            {isExporting ? 'Exporting...' : 'Export'}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
