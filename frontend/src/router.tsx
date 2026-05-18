import { createBrowserRouter } from 'react-router';
import { AppShell } from '@/components/layout/AppShell';
import { DashboardPage } from '@/pages/DashboardPage';
import { ScannerPage } from '@/pages/ScannerPage';
import { ResultsPage } from '@/pages/ResultsPage';
import { RecoveryPage } from '@/pages/RecoveryPage';
import { SettingsPage } from '@/pages/SettingsPage';

export const router = createBrowserRouter([
  {
    element: <AppShell />,
    children: [
      { index: true, element: <DashboardPage /> },
      { path: 'scanner', element: <ScannerPage /> },
      { path: 'results', element: <ResultsPage /> },
      { path: 'recovery', element: <RecoveryPage /> },
      { path: 'settings', element: <SettingsPage /> },
    ],
  },
]);
