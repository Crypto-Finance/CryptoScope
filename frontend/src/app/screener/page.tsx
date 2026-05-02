'use client';

import { useState } from 'react';
import { useScreener } from '@/hooks/useScreener';
import { DataTable, type Column } from '@/components/DataTable';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Tabs, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from '@/components/ui/card';
import { Alert, AlertDescription } from '@/components/ui/alert';
import { Badge } from '@/components/ui/badge';
import { RefreshCw, TrendingUp, TrendingDown } from 'lucide-react';
import type { ScreenerItem } from '@/lib/api';
import { cn } from '@/lib/utils';

type SortField = 'symbol' | 'current_price' | 'change_percent' | 'change_value' | 'volume_24h';

export default function ScreenerPage() {
  const [mode, setMode] = useState<'kline' | 'mark'>('kline');
  const [top, setTop] = useState(20);
  const [minChange, setMinChange] = useState('');
  const [sortField, setSortField] = useState<string>('change_percent');
  const [sortDirection, setSortDirection] = useState<'asc' | 'desc'>('desc');

  const {
    data: screenerData = [],
    isLoading,
    error,
    refetch,
    isRefetching,
  } = useScreener({
    exchange: 'bybit',
    mode,
    top,
    minChange: minChange ? parseFloat(minChange) : undefined,
  });

  const columns: Column<ScreenerItem>[] = [
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
          ${item.current_price.toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 })}
        </span>
      ),
    },
    {
      key: 'open_price',
      header: 'Open Price',
      sortable: true,
      render: (item) => (
        <span className="font-mono text-muted-foreground">
          ${item.open_price.toLocaleString(undefined, { minimumFractionDigits: 2, maximumFractionDigits: 2 })}
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
          {item.change_value > 0 ? '+' : ''}{item.change_value.toFixed(2)}
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

  const handleSort = (key: string) => {
    const sortKey = key as SortField;
    if (sortField === sortKey) {
      setSortDirection(sortDirection === 'asc' ? 'desc' : 'asc');
    } else {
      setSortField(sortKey);
      setSortDirection('asc');
    }
  };

  const sortedData = (Array.isArray(screenerData) ? screenerData : []).sort((a, b) => {
        const aValue = a[sortField as keyof ScreenerItem];
        const bValue = b[sortField as keyof ScreenerItem];

        if (aValue === undefined || bValue === undefined) return 0;
        if (aValue < bValue) return sortDirection === 'asc' ? -1 : 1;
        if (aValue > bValue) return sortDirection === 'asc' ? 1 : -1;
        return 0;
      });

  const gainers = (Array.isArray(screenerData) ? screenerData : []).filter((item) => item.change_percent > 0);
  const losers = (Array.isArray(screenerData) ? screenerData : []).filter((item) => item.change_percent < 0);

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold tracking-tight">Screener</h1>
          <p className="text-muted-foreground">
            Top gainers and losers in the cryptocurrency market
          </p>
        </div>
        <Button
          variant="outline"
          onClick={() => refetch()}
          disabled={isRefetching}
        >
          <RefreshCw className={cn('mr-2 h-4 w-4', isRefetching && 'animate-spin')} />
          Refresh
        </Button>
      </div>

      {error && (
        <Alert variant="destructive">
          <AlertDescription>
            Failed to load screener data: {(error as Error).message}
          </AlertDescription>
        </Alert>
      )}

      <div className="grid gap-4 md:grid-cols-2">
        <Card>
          <CardHeader className="pb-2">
            <CardTitle className="text-sm font-medium text-emerald-500">
              Top Gainers
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{gainers.length}</div>
            <p className="text-xs text-muted-foreground">
              Symbols with positive change
            </p>
          </CardContent>
        </Card>
        <Card>
          <CardHeader className="pb-2">
            <CardTitle className="text-sm font-medium text-red-500">
              Top Losers
            </CardTitle>
          </CardHeader>
          <CardContent>
            <div className="text-2xl font-bold">{losers.length}</div>
            <p className="text-xs text-muted-foreground">
              Symbols with negative change
            </p>
          </CardContent>
        </Card>
      </div>

      <Card>
        <CardHeader>
          <CardTitle>Filters</CardTitle>
          <CardDescription>
            Customize your screener view
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div className="flex flex-col gap-4">
            <div className="flex flex-wrap items-center gap-4">
              <div className="flex items-center gap-2">
                <span className="text-sm text-muted-foreground">Mode:</span>
                <Tabs value={mode} onValueChange={(v) => setMode(v as 'kline' | 'mark')}>
                  <TabsList>
                    <TabsTrigger value="kline">Kline</TabsTrigger>
                    <TabsTrigger value="mark">Mark</TabsTrigger>
                  </TabsList>
                </Tabs>
              </div>

              <div className="flex items-center gap-2">
                <span className="text-sm text-muted-foreground">Top:</span>
                <Input
                  type="number"
                  value={top}
                  onChange={(e) => setTop(parseInt(e.target.value) || 20)}
                  className="w-20"
                  min={5}
                  max={100}
                />
              </div>

              <div className="flex items-center gap-2">
                <span className="text-sm text-muted-foreground">Min Change %:</span>
                <Input
                  type="number"
                  value={minChange}
                  onChange={(e) => setMinChange(e.target.value)}
                  placeholder="0"
                  className="w-24"
                  step={0.1}
                />
              </div>
            </div>
          </div>
        </CardContent>
      </Card>

      <DataTable
        columns={columns}
        data={sortedData}
        isLoading={isLoading}
        sortKey={sortField}
        sortDirection={sortDirection}
        onSort={handleSort}
        emptyMessage="No data found matching your criteria"
      />

      {!isLoading && Array.isArray(screenerData) && (
        <p className="text-sm text-muted-foreground text-center">
          Showing {screenerData.length} symbol{screenerData.length !== 1 ? 's' : ''} • Mode: {mode}
        </p>
      )}
    </div>
  );
}
