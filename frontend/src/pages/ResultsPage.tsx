import { FileGallery } from '@/components/results/FileGallery';
import { useScanStore } from '@/stores/scanStore';
import { Images } from 'lucide-react';

export function ResultsPage() {
  const files = useScanStore((s) => s.files);

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-2xl font-bold text-neutral-100">Results</h1>
        <p className="mt-1 text-sm text-neutral-500">
          Browse and preview recovered files.
        </p>
      </div>

      {files.length === 0 ? (
        <div className="flex flex-col items-center gap-4 py-24 text-center">
          <Images className="h-16 w-16 text-neutral-700" />
          <div>
            <p className="text-lg font-medium text-neutral-400">
              No recovered files yet
            </p>
            <p className="mt-1 text-sm text-neutral-600">
              Run a scan to discover and recover media files from your device.
            </p>
          </div>
        </div>
      ) : (
        <FileGallery files={files} />
      )}
    </div>
  );
}
