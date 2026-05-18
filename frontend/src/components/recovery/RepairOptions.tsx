import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Wrench, FileCode, ShieldCheck } from 'lucide-react';
import { cn } from '@/utils/cn';

export interface RepairSettings {
  fixHeaders: boolean;
  reconstructData: boolean;
  validateAfterRepair: boolean;
}

interface RepairOptionsProps {
  settings: RepairSettings;
  onChange: (settings: RepairSettings) => void;
  disabled?: boolean;
}

const options = [
  {
    key: 'fixHeaders' as const,
    label: 'Fix File Headers',
    description: 'Repair corrupted or missing file headers and metadata markers.',
    icon: FileCode,
  },
  {
    key: 'reconstructData' as const,
    label: 'Reconstruct Data',
    description: 'Attempt to rebuild missing data segments from adjacent sectors.',
    icon: Wrench,
  },
  {
    key: 'validateAfterRepair' as const,
    label: 'Validate After Repair',
    description: 'Run integrity checks on repaired files to verify correctness.',
    icon: ShieldCheck,
  },
];

export function RepairOptions({
  settings,
  onChange,
  disabled = false,
}: RepairOptionsProps) {
  function toggle(key: keyof RepairSettings) {
    onChange({ ...settings, [key]: !settings[key] });
  }

  return (
    <Card>
      <CardHeader>
        <CardTitle>Repair Options</CardTitle>
      </CardHeader>
      <CardContent className="space-y-3">
        {options.map((option) => (
          <label
            key={option.key}
            className={cn(
              'flex cursor-pointer items-start gap-3 rounded-md border border-neutral-800 p-3 transition-colors',
              'hover:bg-surface-800/50',
              settings[option.key] && 'border-primary-500/30 bg-primary-600/5',
              disabled && 'cursor-not-allowed opacity-50',
            )}
          >
            <input
              type="checkbox"
              checked={settings[option.key]}
              onChange={() => toggle(option.key)}
              disabled={disabled}
              className="mt-0.5 h-4 w-4 rounded border-neutral-600 bg-surface-800 text-primary-500 focus:ring-primary-500 focus:ring-offset-0"
            />
            <div className="flex-1">
              <div className="flex items-center gap-2">
                <option.icon className="h-4 w-4 text-neutral-400" aria-hidden="true" />
                <span className="text-sm font-medium text-neutral-200">
                  {option.label}
                </span>
              </div>
              <p className="mt-0.5 text-xs text-neutral-500">
                {option.description}
              </p>
            </div>
          </label>
        ))}
      </CardContent>
    </Card>
  );
}
