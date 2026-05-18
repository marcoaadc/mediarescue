import { NavLink } from 'react-router';
import {
  LayoutDashboard,
  ScanSearch,
  Images,
  Wrench,
  Settings,
} from 'lucide-react';
import { cn } from '@/utils/cn';

const navItems = [
  { to: '/', icon: LayoutDashboard, label: 'Dashboard' },
  { to: '/scanner', icon: ScanSearch, label: 'Scanner' },
  { to: '/results', icon: Images, label: 'Results' },
  { to: '/recovery', icon: Wrench, label: 'Recovery' },
  { to: '/settings', icon: Settings, label: 'Settings' },
] as const;

export function Sidebar() {
  return (
    <aside className="flex w-56 flex-col border-r border-neutral-800 bg-surface-950">
      <nav className="flex flex-1 flex-col gap-1 p-3" aria-label="Main navigation">
        {navItems.map((item) => (
          <NavLink
            key={item.to}
            to={item.to}
            end={item.to === '/'}
            className={({ isActive }) =>
              cn(
                'flex items-center gap-3 rounded-md px-3 py-2 text-sm font-medium transition-colors',
                'focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-primary-500',
                isActive
                  ? 'bg-primary-600/15 text-primary-400'
                  : 'text-neutral-400 hover:bg-surface-800 hover:text-neutral-200',
              )
            }
          >
            <item.icon className="h-4 w-4 shrink-0" aria-hidden="true" />
            {item.label}
          </NavLink>
        ))}
      </nav>

      <div className="border-t border-neutral-800 p-3">
        <p className="text-xs text-neutral-600">v0.1.0</p>
      </div>
    </aside>
  );
}
