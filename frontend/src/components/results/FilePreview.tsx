import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
  DialogDescription,
} from '@/components/ui/dialog';
import { StatusBadge } from '@/components/common/StatusBadge';
import { ByteSize } from '@/components/common/ByteSize';
import { formatMediaType } from '@/utils/format';
import { getFilePreview } from '@/api/endpoints';
import type { RecoveredFile, MediaFormat } from '@/types';

interface FilePreviewProps {
  file: RecoveredFile | null;
  open: boolean;
  onClose: () => void;
}

const imageFormats: MediaFormat[] = ['jpeg', 'png', 'cr2', 'nef', 'arw'];
const videoFormats: MediaFormat[] = ['mp4', 'mov', 'avi', 'mkv'];

export function FilePreview({ file, open, onClose }: FilePreviewProps) {
  if (!file) return null;

  const isImage = imageFormats.includes(file.format);
  const isVideo = videoFormats.includes(file.format);
  const previewUrl = getFilePreview(file.id);

  return (
    <Dialog open={open} onOpenChange={(o) => !o && onClose()}>
      <DialogContent className="max-w-2xl">
        <DialogHeader>
          <DialogTitle className="flex items-center gap-2">
            <StatusBadge variant="muted">
              {formatMediaType(file.format)}
            </StatusBadge>
            <span className="font-mono text-sm text-neutral-400">
              {file.id.slice(0, 16)}
            </span>
          </DialogTitle>
          <DialogDescription>
            <ByteSize bytes={file.size_bytes} /> - Recovery score:{' '}
            {file.recovery_score}%
          </DialogDescription>
        </DialogHeader>

        {/* Preview area */}
        <div className="flex min-h-[300px] items-center justify-center rounded-md bg-surface-800">
          {isImage && file.status === 'valid' ? (
            <img
              src={previewUrl}
              alt={`Preview of recovered ${file.format} file`}
              className="max-h-[60vh] max-w-full rounded object-contain"
            />
          ) : isVideo && file.status === 'valid' ? (
            <video
              src={previewUrl}
              controls
              className="max-h-[60vh] max-w-full rounded"
              aria-label={`Preview of recovered ${file.format} video`}
            >
              <track kind="captions" />
            </video>
          ) : (
            <div className="text-center text-neutral-500">
              <p className="text-lg">Preview not available</p>
              <p className="mt-1 text-sm">
                File status: {file.status}
              </p>
            </div>
          )}
        </div>

        {/* File metadata */}
        <div className="grid grid-cols-2 gap-3 text-sm">
          <div>
            <span className="text-neutral-500">Disk Offset</span>
            <p className="font-mono text-neutral-300">
              0x{file.disk_offset.toString(16).toUpperCase()}
            </p>
          </div>
          <div>
            <span className="text-neutral-500">Status</span>
            <p className="text-neutral-300">{file.status}</p>
          </div>
          <div>
            <span className="text-neutral-500">Can Repair</span>
            <p className="text-neutral-300">{file.can_repair ? 'Yes' : 'No'}</p>
          </div>
          <div>
            <span className="text-neutral-500">Recovery Score</span>
            <p className="text-neutral-300">{file.recovery_score}%</p>
          </div>
        </div>
      </DialogContent>
    </Dialog>
  );
}
