import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Button } from '@/components/ui/button';
import { useSettingsStore } from '@/stores/settingsStore';
import { cn } from '@/utils/cn';
import type { ScanDepth, MediaFormat } from '@/types';

const ALL_FORMATS: MediaFormat[] = [
  'jpeg', 'png', 'mp4', 'mov', 'avi', 'mkv', 'cr2', 'nef', 'arw',
];

const depthOptions: { value: ScanDepth; label: string; description: string }[] = [
  {
    value: 'quick',
    label: 'Quick',
    description: 'Scan file headers only. Fastest but may miss fragmented files.',
  },
  {
    value: 'standard',
    label: 'Standard',
    description: 'Scan headers and data regions. Good balance of speed and recovery.',
  },
  {
    value: 'deep',
    label: 'Deep',
    description: 'Full sector-by-sector scan. Slowest but highest recovery rate.',
  },
];

export function SettingsPage() {
  const {
    scanDepth,
    enabledFormats,
    maxWorkers,
    sectorRetryCount,
    outputDirectory,
    setScanDepth,
    toggleFormat,
    setEnabledFormats,
    setMaxWorkers,
    setSectorRetryCount,
    setOutputDirectory,
  } = useSettingsStore();

  return (
    <div className="space-y-6">
      <div>
        <h1 className="text-2xl font-bold text-neutral-100">Settings</h1>
        <p className="mt-1 text-sm text-neutral-500">
          Configure scan parameters and output preferences.
        </p>
      </div>

      <div className="grid gap-6 lg:grid-cols-2">
        {/* Scan Depth */}
        <Card>
          <CardHeader>
            <CardTitle>Scan Depth</CardTitle>
          </CardHeader>
          <CardContent className="space-y-2">
            {depthOptions.map((option) => (
              <label
                key={option.value}
                className={cn(
                  'flex cursor-pointer items-start gap-3 rounded-md border p-3 transition-colors',
                  scanDepth === option.value
                    ? 'border-primary-500/40 bg-primary-600/5'
                    : 'border-neutral-800 hover:bg-surface-800/50',
                )}
              >
                <input
                  type="radio"
                  name="scan-depth"
                  value={option.value}
                  checked={scanDepth === option.value}
                  onChange={() => setScanDepth(option.value)}
                  className="mt-0.5 h-4 w-4 border-neutral-600 bg-surface-800 text-primary-500 focus:ring-primary-500 focus:ring-offset-0"
                />
                <div>
                  <span className="text-sm font-medium text-neutral-200">
                    {option.label}
                  </span>
                  <p className="mt-0.5 text-xs text-neutral-500">
                    {option.description}
                  </p>
                </div>
              </label>
            ))}
          </CardContent>
        </Card>

        {/* Format Filters */}
        <Card>
          <CardHeader>
            <div className="flex items-center justify-between">
              <CardTitle>Format Filters</CardTitle>
              <div className="flex gap-2">
                <Button
                  variant="ghost"
                  size="sm"
                  onClick={() => setEnabledFormats([...ALL_FORMATS])}
                >
                  All
                </Button>
                <Button
                  variant="ghost"
                  size="sm"
                  onClick={() => setEnabledFormats([])}
                >
                  None
                </Button>
              </div>
            </div>
          </CardHeader>
          <CardContent>
            <div className="grid grid-cols-3 gap-2">
              {ALL_FORMATS.map((format) => (
                <label
                  key={format}
                  className={cn(
                    'flex cursor-pointer items-center gap-2 rounded-md border px-3 py-2 text-sm transition-colors',
                    enabledFormats.includes(format)
                      ? 'border-primary-500/40 bg-primary-600/10 text-primary-400'
                      : 'border-neutral-800 text-neutral-500 hover:text-neutral-300',
                  )}
                >
                  <input
                    type="checkbox"
                    checked={enabledFormats.includes(format)}
                    onChange={() => toggleFormat(format)}
                    className="h-3.5 w-3.5 rounded border-neutral-600 bg-surface-800 text-primary-500 focus:ring-primary-500 focus:ring-offset-0"
                  />
                  {format.toUpperCase()}
                </label>
              ))}
            </div>
          </CardContent>
        </Card>

        {/* Performance */}
        <Card>
          <CardHeader>
            <CardTitle>Performance</CardTitle>
          </CardHeader>
          <CardContent className="space-y-5">
            <div>
              <label
                htmlFor="max-workers"
                className="mb-1.5 block text-sm text-neutral-400"
              >
                Max Workers: {maxWorkers}
              </label>
              <input
                id="max-workers"
                type="range"
                min={1}
                max={16}
                value={maxWorkers}
                onChange={(e) => setMaxWorkers(Number(e.target.value))}
                className="w-full accent-primary-500"
              />
              <div className="mt-1 flex justify-between text-xs text-neutral-600">
                <span>1</span>
                <span>16</span>
              </div>
            </div>
            <div>
              <label
                htmlFor="retry-count"
                className="mb-1.5 block text-sm text-neutral-400"
              >
                Sector Retry Count: {sectorRetryCount}
              </label>
              <input
                id="retry-count"
                type="range"
                min={0}
                max={10}
                value={sectorRetryCount}
                onChange={(e) => setSectorRetryCount(Number(e.target.value))}
                className="w-full accent-primary-500"
              />
              <div className="mt-1 flex justify-between text-xs text-neutral-600">
                <span>0</span>
                <span>10</span>
              </div>
            </div>
          </CardContent>
        </Card>

        {/* Output */}
        <Card>
          <CardHeader>
            <CardTitle>Output Directory</CardTitle>
          </CardHeader>
          <CardContent>
            <input
              type="text"
              value={outputDirectory}
              onChange={(e) => setOutputDirectory(e.target.value)}
              placeholder="/path/to/recovered-files"
              className="w-full rounded-md border border-neutral-700 bg-surface-800 px-3 py-2 text-sm text-neutral-200 placeholder-neutral-600 focus:border-primary-500 focus:outline-none focus:ring-1 focus:ring-primary-500"
              aria-label="Output directory path"
            />
            <p className="mt-2 text-xs text-neutral-600">
              Recovered files will be exported to this directory.
            </p>
          </CardContent>
        </Card>
      </div>
    </div>
  );
}
