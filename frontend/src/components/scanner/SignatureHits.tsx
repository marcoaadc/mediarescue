import { useScanStore } from '@/stores/scanStore';
import { StatusBadge } from '@/components/common/StatusBadge';
import { ByteSize } from '@/components/common/ByteSize';
import { formatMediaType, formatNumber } from '@/utils/format';
import type { FileStatus } from '@/types';

const statusVariant: Record<FileStatus, 'info' | 'warning' | 'success' | 'danger' | 'muted'> = {
  discovered: 'info',
  carving: 'warning',
  carved: 'warning',
  reconstructing: 'warning',
  reconstructed: 'info',
  validating: 'info',
  valid: 'success',
  invalid: 'danger',
  failed: 'danger',
};

export function SignatureHits() {
  const files = useScanStore((s) => s.files);

  // Show the most recent 50 files
  const visibleFiles = files.slice(-50).reverse();

  return (
    <div>
      <div className="mb-3 flex items-center justify-between">
        <h3 className="text-sm font-semibold text-neutral-200">
          Detected Signatures
        </h3>
        <span className="text-xs text-neutral-500">
          {formatNumber(files.length)} total
        </span>
      </div>

      <div className="overflow-auto rounded-md border border-neutral-800">
        <table className="w-full text-left text-sm">
          <thead className="border-b border-neutral-800 bg-surface-900 text-xs text-neutral-500">
            <tr>
              <th scope="col" className="px-3 py-2 font-medium">Format</th>
              <th scope="col" className="px-3 py-2 font-medium">Offset</th>
              <th scope="col" className="px-3 py-2 font-medium">Size</th>
              <th scope="col" className="px-3 py-2 font-medium">Score</th>
              <th scope="col" className="px-3 py-2 font-medium">Status</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-neutral-800/50">
            {visibleFiles.length === 0 ? (
              <tr>
                <td colSpan={5} className="px-3 py-8 text-center text-neutral-600">
                  No signatures detected yet
                </td>
              </tr>
            ) : (
              visibleFiles.map((file) => (
                <tr key={file.id} className="hover:bg-surface-800/50">
                  <td className="px-3 py-2">
                    <StatusBadge variant="muted">
                      {formatMediaType(file.format)}
                    </StatusBadge>
                  </td>
                  <td className="px-3 py-2 font-mono text-neutral-400">
                    0x{file.disk_offset.toString(16).toUpperCase()}
                  </td>
                  <td className="px-3 py-2 text-neutral-400">
                    <ByteSize bytes={file.size_bytes} />
                  </td>
                  <td className="px-3 py-2 font-mono text-neutral-300">
                    {file.recovery_score > 0 ? `${file.recovery_score}%` : '--'}
                  </td>
                  <td className="px-3 py-2">
                    <StatusBadge variant={statusVariant[file.status]}>
                      {file.status}
                    </StatusBadge>
                  </td>
                </tr>
              ))
            )}
          </tbody>
        </table>
      </div>
    </div>
  );
}
