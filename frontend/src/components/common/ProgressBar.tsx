import { cn } from '@/utils/cn';

interface ProgressBarProps {
  /** Value from 0 to 100 */
  value: number;
  /** Optional label rendered above the bar */
  label?: string;
  /** Show percentage text on the right */
  showPercent?: boolean;
  /** Color variant */
  variant?: 'primary' | 'success' | 'warning' | 'danger';
  /** Height class */
  size?: 'sm' | 'md' | 'lg';
  className?: string;
}

const variantColors: Record<NonNullable<ProgressBarProps['variant']>, string> = {
  primary: 'bg-primary-500',
  success: 'bg-success-500',
  warning: 'bg-warning-500',
  danger: 'bg-danger-500',
};

const sizeClasses: Record<NonNullable<ProgressBarProps['size']>, string> = {
  sm: 'h-1.5',
  md: 'h-2.5',
  lg: 'h-4',
};

export function ProgressBar({
  value,
  label,
  showPercent = false,
  variant = 'primary',
  size = 'md',
  className,
}: ProgressBarProps) {
  const clamped = Math.max(0, Math.min(100, value));

  return (
    <div className={cn('w-full', className)}>
      {(label || showPercent) && (
        <div className="mb-1 flex items-center justify-between text-sm">
          {label && (
            <span className="text-neutral-400">{label}</span>
          )}
          {showPercent && (
            <span className="font-mono text-neutral-300">
              {clamped.toFixed(1)}%
            </span>
          )}
        </div>
      )}
      <div
        className={cn(
          'w-full overflow-hidden rounded-full bg-surface-800',
          sizeClasses[size],
        )}
        role="progressbar"
        aria-valuenow={clamped}
        aria-valuemin={0}
        aria-valuemax={100}
        aria-label={label ?? 'Progress'}
      >
        <div
          className={cn(
            'h-full rounded-full transition-all duration-300 ease-out',
            variantColors[variant],
          )}
          style={{ width: `${clamped}%` }}
        />
      </div>
    </div>
  );
}
