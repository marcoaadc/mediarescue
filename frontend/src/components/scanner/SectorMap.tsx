import { useScanStore } from '@/stores/scanStore';
import { cn } from '@/utils/cn';

/**
 * Placeholder sector map component.
 * Renders a grid of colored cells representing sector scan status.
 * Future: replace with a canvas-based implementation for large sector counts.
 */
export function SectorMap() {
  const sectorsDone = useScanStore((s) => s.sectorsDone);
  const sectorsTotal = useScanStore((s) => s.sectorsTotal);

  // Display a fixed grid of cells representing proportional scan progress
  const gridSize = 256; // 16x16 grid
  const filledCount =
    sectorsTotal > 0
      ? Math.round((sectorsDone / sectorsTotal) * gridSize)
      : 0;

  return (
    <div>
      <h3 className="mb-3 text-sm font-semibold text-neutral-200">
        Sector Map
      </h3>
      <div
        className="grid gap-0.5 rounded-md border border-neutral-800 bg-surface-950 p-2"
        style={{
          gridTemplateColumns: 'repeat(16, 1fr)',
        }}
        role="img"
        aria-label={`Sector map: ${sectorsDone} of ${sectorsTotal} sectors scanned`}
      >
        {Array.from({ length: gridSize }, (_, i) => (
          <div
            key={i}
            className={cn(
              'aspect-square rounded-[2px] transition-colors duration-150',
              i < filledCount ? 'bg-primary-500' : 'bg-surface-800',
            )}
          />
        ))}
      </div>
      <p className="mt-2 text-center text-xs text-neutral-600">
        Each cell represents a proportional group of sectors
      </p>
    </div>
  );
}
