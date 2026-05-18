import { useState, useMemo } from 'react';
import { FileCard } from './FileCard';
import { FilterBar, type FileFilters } from './FilterBar';
import { FilePreview } from './FilePreview';
import type { RecoveredFile, MediaFormat } from '@/types';

const ALL_FORMATS: MediaFormat[] = [
  'jpeg', 'png', 'mp4', 'mov', 'avi', 'mkv', 'cr2', 'nef', 'arw',
];

interface FileGalleryProps {
  files: RecoveredFile[];
}

export function FileGallery({ files }: FileGalleryProps) {
  const [filters, setFilters] = useState<FileFilters>({
    formats: [...ALL_FORMATS],
    minScore: 0,
    minSize: 0,
    maxSize: Infinity,
  });

  const [previewFile, setPreviewFile] = useState<RecoveredFile | null>(null);

  const filteredFiles = useMemo(() => {
    return files.filter((file) => {
      if (!filters.formats.includes(file.format)) return false;
      if (file.recovery_score < filters.minScore) return false;
      if (file.size_bytes < filters.minSize) return false;
      if (file.size_bytes > filters.maxSize) return false;
      return true;
    });
  }, [files, filters]);

  return (
    <div className="space-y-4">
      <FilterBar
        filters={filters}
        onChange={setFilters}
        totalCount={files.length}
        filteredCount={filteredFiles.length}
      />

      {filteredFiles.length === 0 ? (
        <div className="flex flex-col items-center gap-2 py-16 text-center">
          <p className="text-neutral-400">No files match the current filters</p>
          <p className="text-sm text-neutral-600">
            Try adjusting your filters or running a new scan.
          </p>
        </div>
      ) : (
        <div className="grid gap-3 sm:grid-cols-2 md:grid-cols-3 lg:grid-cols-4 xl:grid-cols-5">
          {/* Virtual scrolling placeholder: for production, wrap this in a
              virtualized grid from @tanstack/react-virtual */}
          {filteredFiles.map((file) => (
            <FileCard
              key={file.id}
              file={file}
              onSelect={setPreviewFile}
            />
          ))}
        </div>
      )}

      <FilePreview
        file={previewFile}
        open={previewFile !== null}
        onClose={() => setPreviewFile(null)}
      />
    </div>
  );
}
