import { useRef, useEffect } from 'react';
import { useScanStore } from '@/stores/scanStore';

const COLS = 32;
const ROWS = 16;
const CELL_SIZE = 10;
const GAP = 1;

const COLOR_SCANNED = '#6366f1';
const COLOR_EMPTY = '#1e1e2e';

export function SectorMap() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const sectorsDone = useScanStore((s) => s.sectorsDone);
  const sectorsTotal = useScanStore((s) => s.sectorsTotal);

  const gridSize = COLS * ROWS;
  const filledCount =
    sectorsTotal > 0 ? Math.round((sectorsDone / sectorsTotal) * gridSize) : 0;

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    ctx.clearRect(0, 0, canvas.width, canvas.height);

    for (let i = 0; i < gridSize; i++) {
      const col = i % COLS;
      const row = Math.floor(i / COLS);
      const x = col * (CELL_SIZE + GAP);
      const y = row * (CELL_SIZE + GAP);

      ctx.fillStyle = i < filledCount ? COLOR_SCANNED : COLOR_EMPTY;
      ctx.beginPath();
      ctx.roundRect(x, y, CELL_SIZE, CELL_SIZE, 2);
      ctx.fill();
    }
  }, [filledCount, gridSize]);

  const width = COLS * (CELL_SIZE + GAP) - GAP;
  const height = ROWS * (CELL_SIZE + GAP) - GAP;

  return (
    <div>
      <h3 className="mb-3 text-sm font-semibold text-neutral-200">
        Sector Map
      </h3>
      <canvas
        ref={canvasRef}
        width={width}
        height={height}
        className="rounded-md border border-neutral-800 bg-surface-950 p-2"
        role="img"
        aria-label={`Sector map: ${sectorsDone} of ${sectorsTotal} sectors scanned`}
      />
      <p className="mt-2 text-center text-xs text-neutral-600">
        Each cell represents a proportional group of sectors
      </p>
    </div>
  );
}
