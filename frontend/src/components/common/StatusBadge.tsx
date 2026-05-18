import { cn } from '@/utils/cn';

type BadgeVariant = 'default' | 'success' | 'warning' | 'danger' | 'info' | 'muted';

interface StatusBadgeProps {
  children: React.ReactNode;
  variant?: BadgeVariant;
  /** Show a pulsing dot before the text */
  dot?: boolean;
  className?: string;
}

const variantStyles: Record<BadgeVariant, string> = {
  default:
    'bg-neutral-800 text-neutral-300 border-neutral-700',
  success:
    'bg-success-500/15 text-success-400 border-success-500/30',
  warning:
    'bg-warning-500/15 text-warning-400 border-warning-500/30',
  danger:
    'bg-danger-500/15 text-danger-400 border-danger-500/30',
  info:
    'bg-primary-500/15 text-primary-400 border-primary-500/30',
  muted:
    'bg-neutral-900 text-neutral-500 border-neutral-800',
};

const dotColors: Record<BadgeVariant, string> = {
  default: 'bg-neutral-400',
  success: 'bg-success-400',
  warning: 'bg-warning-400',
  danger: 'bg-danger-400',
  info: 'bg-primary-400',
  muted: 'bg-neutral-600',
};

export function StatusBadge({
  children,
  variant = 'default',
  dot = false,
  className,
}: StatusBadgeProps) {
  return (
    <span
      className={cn(
        'inline-flex items-center gap-1.5 rounded-md border px-2 py-0.5 text-xs font-medium',
        variantStyles[variant],
        className,
      )}
    >
      {dot && (
        <span
          className={cn(
            'inline-block h-1.5 w-1.5 rounded-full',
            dotColors[variant],
          )}
          aria-hidden="true"
        />
      )}
      {children}
    </span>
  );
}
