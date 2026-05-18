import { formatBytes } from '@/utils/format';

interface ByteSizeProps {
  bytes: number;
  decimals?: number;
  className?: string;
}

/**
 * Renders a byte count as a human-readable size string.
 * Wraps the formatBytes utility for declarative usage in JSX.
 */
export function ByteSize({ bytes, decimals = 2, className }: ByteSizeProps) {
  return (
    <span className={className} title={`${bytes.toLocaleString()} bytes`}>
      {formatBytes(bytes, decimals)}
    </span>
  );
}
