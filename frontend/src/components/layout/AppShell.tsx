import { Outlet } from 'react-router';
import { Header } from './Header';
import { Sidebar } from './Sidebar';
import { ErrorBoundary } from '@/components/common/ErrorBoundary';
import { useWebSocket } from '@/hooks/useWebSocket';

export function AppShell() {
  const { isConnected } = useWebSocket();

  return (
    <div className="flex h-screen flex-col overflow-hidden bg-surface-950">
      <Header isConnected={isConnected} />
      <div className="flex flex-1 overflow-hidden">
        <Sidebar />
        <main className="flex-1 overflow-auto p-6">
          <ErrorBoundary>
            <Outlet />
          </ErrorBoundary>
        </main>
      </div>
    </div>
  );
}
