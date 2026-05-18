import { useState } from 'react';
import { Filter, X } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { cn } from '@/utils/cn';
import type { MediaFormat } from '@/types';

const ALL_FORMATS: MediaFormat[] = [
  'jpeg', 'png', 'mp4', 'mov', 'avi', 'mkv', 'cr2', 'nef', 'arw',
];

export interface FileFilters {
  formats: MediaFormat[];
  minScore: number;
  minSize: number;
  maxSize: number;
}

interface FilterBarProps {
  filters: FileFilters;
  onChange: (filters: FileFilters) => void;
  totalCount: number;
  filteredCount: number;
}

export function FilterBar({
  filters,
  onChange,
  totalCount,
  filteredCount,
}: FilterBarProps) {
  const [expanded, setExpanded] = useState(false);

  const hasActiveFilters =
    filters.formats.length < ALL_FORMATS.length ||
    filters.minScore > 0 ||
    filters.minSize > 0 ||
    filters.maxSize < Infinity;

  function toggleFormat(format: MediaFormat) {
    const next = filters.formats.includes(format)
      ? filters.formats.filter((f) => f !== format)
      : [...filters.formats, format];
    onChange({ ...filters, formats: next });
  }

  function clearFilters() {
    onChange({
      formats: [...ALL_FORMATS],
      minScore: 0,
      minSize: 0,
      maxSize: Infinity,
    });
  }

  return (
    <div className="space-y-3">
      <div className="flex items-center justify-between">
        <div className="flex items-center gap-3">
          <Button
            variant={expanded ? 'secondary' : 'ghost'}
            size="sm"
            onClick={() => setExpanded(!expanded)}
          >
            <Filter className="h-4 w-4" aria-hidden="true" />
            Filters
          </Button>
          {hasActiveFilters && (
            <Button variant="ghost" size="sm" onClick={clearFilters}>
              <X className="h-3 w-3" aria-hidden="true" />
              Clear
            </Button>
          )}
        </div>
        <span className="text-xs text-neutral-500">
          {filteredCount} of {totalCount} files
        </span>
      </div>

      {expanded && (
        <div className="rounded-md border border-neutral-800 bg-surface-900 p-4">
          {/* Format filter */}
          <div>
            <p className="mb-2 text-xs font-medium text-neutral-400">
              File Formats
            </p>
            <div className="flex flex-wrap gap-1.5">
              {ALL_FORMATS.map((format) => (
                <button
                  key={format}
                  type="button"
                  onClick={() => toggleFormat(format)}
                  className={cn(
                    'rounded-md border px-2.5 py-1 text-xs font-medium transition-colors',
                    'focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-primary-500',
                    filters.formats.includes(format)
                      ? 'border-primary-500/40 bg-primary-600/15 text-primary-400'
                      : 'border-neutral-700 text-neutral-500 hover:text-neutral-300',
                  )}
                  aria-pressed={filters.formats.includes(format)}
                >
                  {format.toUpperCase()}
                </button>
              ))}
            </div>
          </div>

          {/* Score filter */}
          <div className="mt-4">
            <label
              htmlFor="min-score"
              className="mb-2 block text-xs font-medium text-neutral-400"
            >
              Minimum Recovery Score: {filters.minScore}%
            </label>
            <input
              id="min-score"
              type="range"
              min={0}
              max={100}
              step={5}
              value={filters.minScore}
              onChange={(e) =>
                onChange({ ...filters, minScore: Number(e.target.value) })
              }
              className="w-full accent-primary-500"
            />
          </div>
        </div>
      )}
    </div>
  );
}
