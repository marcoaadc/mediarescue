import { FileImage, FileVideo, FileQuestion } from 'lucide-react';
import { Card, CardContent } from '@/components/ui/card';
import { StatusBadge } from '@/components/common/StatusBadge';
import { ByteSize } from '@/components/common/ByteSize';
import { formatMediaType } from '@/utils/format';
import { cn } from '@/utils/cn';
import { getFileThumbnail } from '@/api/endpoints';
import type { RecoveredFile, MediaFormat } from '@/types';

interface FileCardProps {
  file: RecoveredFile;
  onSelect: (file: RecoveredFile) => void;
}

const imageFormats: MediaFormat[] = ['jpeg', 'png', 'cr2', 'nef', 'arw'];
const videoFormats: MediaFormat[] = ['mp4', 'mov', 'avi', 'mkv'];

function getFormatIcon(format: MediaFormat) {
  if (imageFormats.includes(format)) return FileImage;
  if (videoFormats.includes(format)) return FileVideo;
  return FileQuestion;
}

function getScoreVariant(score: number) {
  if (score >= 80) return 'success' as const;
  if (score >= 50) return 'warning' as const;
  if (score > 0) return 'danger' as const;
  return 'muted' as const;
}

export function FileCard({ file, onSelect }: FileCardProps) {
  const Icon = getFormatIcon(file.format);
  const isImage = imageFormats.includes(file.format);

  return (
    <Card
      className="cursor-pointer overflow-hidden transition-colors hover:border-neutral-700"
    >
      <button
        type="button"
        onClick={() => onSelect(file)}
        className="w-full text-left focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-primary-500"
        aria-label={`${formatMediaType(file.format)} file, ${file.recovery_score}% recovery score`}
      >
        {/* Thumbnail area */}
        <div className="relative flex h-32 items-center justify-center bg-surface-800">
          {isImage && file.status === 'valid' ? (
            <img
              src={getFileThumbnail(file.id)}
              alt=""
              className="h-full w-full object-cover"
              loading="lazy"
            />
          ) : (
            <Icon className="h-10 w-10 text-neutral-600" aria-hidden="true" />
          )}

          {/* Format badge overlay */}
          <div className="absolute left-2 top-2">
            <StatusBadge variant="muted">
              {formatMediaType(file.format)}
            </StatusBadge>
          </div>
        </div>

        <CardContent className="p-3">
          <div className="flex items-center justify-between gap-2">
            <span className="truncate text-xs font-mono text-neutral-400">
              {file.id.slice(0, 12)}
            </span>
            <ByteSize
              bytes={file.size_bytes}
              className="shrink-0 text-xs text-neutral-500"
            />
          </div>

          <div className="mt-2 flex items-center justify-between">
            <StatusBadge variant={getScoreVariant(file.recovery_score)}>
              {file.recovery_score > 0 ? `${file.recovery_score}%` : 'Pending'}
            </StatusBadge>
            {file.can_repair && (
              <span
                className={cn(
                  'text-xs text-primary-400',
                )}
              >
                Repairable
              </span>
            )}
          </div>
        </CardContent>
      </button>
    </Card>
  );
}
