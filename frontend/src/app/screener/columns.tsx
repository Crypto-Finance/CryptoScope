import type { Column } from '@/components/DataTable';
import type { ScreenerItem } from '@/lib/api';
import { Badge } from '@/components/ui/badge';
import { cn } from '@/lib/utils';
import { TrendingUp, TrendingDown } from 'lucide-react';
import { formatPrice } from '@/lib/formatters';

/**
 * Create column definitions for the screener data table.
 * @returns Array of column definitions
 */
export function createScreenerColumns(): Column<ScreenerItem>[] {
  return [
    {
      key: 'symbol',
      header: 'Symbol',
      sortable: true,
      render: (item) => (
        <div className="font-medium">{item.symbol}</div>
      ),
    },
    {
      key: 'current_price',
      header: 'Current Price',
      sortable: true,
      render: (item) => (
        <span className="font-mono">
          ${formatPrice(item.current_price)}
        </span>
      ),
    },
    {
      key: 'open_price',
      header: 'Open Price',
      sortable: true,
      render: (item) => (
        <span className="font-mono text-muted-foreground">
          ${formatPrice(item.open_price)}
        </span>
      ),
    },
    {
      key: 'change_percent',
      header: 'Change %',
      sortable: true,
      render: (item) => (
        <Badge
          variant={item.change_percent >= 0 ? 'default' : 'secondary'}
          className={cn(
            item.change_percent > 0 && 'bg-emerald-500 hover:bg-emerald-600',
            item.change_percent < 0 && 'bg-red-500 hover:bg-red-600'
          )}
        >
          {item.change_percent > 0 && <TrendingUp className="mr-1 h-3 w-3" />}
          {item.change_percent < 0 && <TrendingDown className="mr-1 h-3 w-3" />}
          {item.change_percent.toFixed(2)}%
        </Badge>
      ),
    },
    {
      key: 'change_value',
      header: 'Change',
      sortable: true,
      render: (item) => (
        <span className={cn(
          'font-mono',
          item.change_value > 0 && 'text-emerald-500',
          item.change_value < 0 && 'text-red-500'
        )}>
          {item.change_value > 0 ? '+' : ''}{item.change_value.toFixed(5)}
        </span>
      ),
    },
    {
      key: 'volume_24h',
      header: 'Volume 24h',
      sortable: true,
      render: (item) => (
        <span className="text-muted-foreground">
          {item.volume_24h.toLocaleString()}
        </span>
      ),
    },
  ];
}
