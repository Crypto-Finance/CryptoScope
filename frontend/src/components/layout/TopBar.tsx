'use client';

import { cn } from '@/lib/utils';
import { Button } from '@/components/ui/button';
import { TerminalInput } from '@/components/stitch/TerminalInput';
import { ThemeToggle } from '@/components/stitch/ThemeToggle';
import { StatusPip } from '@/components/stitch/StatusPip';
import { getStatusLabel } from '@/lib/status';
import { Menu, Search } from 'lucide-react';

interface TopBarProps {
  className?: string;
  onMenuClick?: () => void;
  connectionStatus?: 'connected' | 'connecting' | 'disconnected' | 'error';
  onSearch?: (query: string) => void;
}

/**
 * TopBar - Top header bar
 * 
 * Features:
 * - CryptoScope logo (mobile)
 * - Search input with terminal aesthetic
 * - Theme toggle
 * - Exchange connection status
 * - Mobile menu trigger
 */
export function TopBar({
  className,
  onMenuClick,
  connectionStatus = 'connected',
  onSearch,
}: TopBarProps) {
  return (
    <header
      className={cn(
        'h-14 border-b bg-card',
        'flex items-center justify-between',
        'px-4 md:px-6',
        'sticky top-0 z-40',
        className
      )}
    >
      {/* Left Section - Mobile Menu */}
      <div className="flex items-center gap-4">
        <Button
          variant="ghost"
          size="icon-sm"
          className="md:hidden"
          onClick={onMenuClick}
          aria-label="Open menu"
        >
          <Menu className="h-4 w-4" />
        </Button>

        {/* Desktop Logo - Hidden on mobile */}
        <div className="hidden md:flex items-center gap-2">
          <div className="h-7 w-7 rounded-md bg-gradient-to-br from-cyan-500 to-blue-600 flex items-center justify-center">
            <span className="text-white font-bold text-xs">CS</span>
          </div>
        </div>
      </div>

      {/* Center Section - Search */}
      <div className="flex-1 max-w-md mx-4">
        <TerminalInput
          placeholder="Search symbols, pairs..."
          icon={Search}
          variant="compact"
          containerClassName="mb-0"
          className="h-9"
          onChange={(e) => onSearch?.(e.target.value)}
        />
      </div>

      {/* Right Section - Status & Theme */}
      <div className="flex items-center gap-3">
        {/* Connection Status */}
        <div className="hidden sm:flex items-center gap-2 text-xs">
          <StatusPip variant={connectionStatus} size="sm" />
          <span className="text-muted-foreground">{getStatusLabel(connectionStatus)}</span>
        </div>

        {/* Theme Toggle */}
        <ThemeToggle />
      </div>
    </header>
  );
}
