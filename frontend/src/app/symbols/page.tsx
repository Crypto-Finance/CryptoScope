'use client';

import { useState, useEffect } from 'react';
import { useSymbols } from '@/hooks/useSymbols';
import { DataTable, type Column } from '@/components/DataTable';
import { CategoryFilter, type CategoryValue } from '@/components/CategoryFilter';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from '@/components/ui/card';
import { Alert, AlertDescription } from '@/components/ui/alert';
import { Badge } from '@/components/ui/badge';
import { RefreshCw, Search } from 'lucide-react';
import { cn } from '@/lib/utils';
import type { CryptoSymbol } from '@/lib/api';

export default function SymbolsPage() {
  const [category, setCategory] = useState<CategoryValue>('all');
  const [search, setSearch] = useState('');
  const [debouncedSearch, setDebouncedSearch] = useState('');

  // Debounce search input
  useEffect(() => {
    const timer = setTimeout(() => {
      setDebouncedSearch(search);
    }, 300);
    return () => clearTimeout(timer);
  }, [search]);

  const {
    data: symbols = [],
    isLoading,
    error,
    refetch,
    isRefetching,
  } = useSymbols({
    exchange: 'bybit',
    category: category === 'all' ? undefined : category,
    search: debouncedSearch || undefined,
  });

  const columns: Column<CryptoSymbol>[] = [
    {
      key: 'symbol',
      header: 'Symbol',
      sortable: true,
      render: (item) => (
        <div className="font-medium">{item.symbol}</div>
      ),
    },
    {
      key: 'contractType',
      header: 'Contract Type',
      sortable: true,
      render: (item) => (
        <Badge variant="outline">{item.contractType}</Badge>
      ),
    },
    {
      key: 'baseCoin',
      header: 'Base Coin',
      sortable: true,
      render: (item) => (
        <Badge variant="secondary">{item.baseCoin}</Badge>
      ),
    },
    {
      key: 'quoteCoin',
      header: 'Quote Coin',
      sortable: true,
      render: (item) => (
        <Badge variant="secondary">{item.quoteCoin}</Badge>
      ),
    },
    {
      key: 'category',
      header: 'Category',
      sortable: true,
      render: (item) => (
        <Badge variant="secondary" className="capitalize">
          {item.category}
        </Badge>
      ),
    },
  ];

  const [sortKey, setSortKey] = useState<string>('symbol');
  const [sortDirection, setSortDirection] = useState<'asc' | 'desc'>('asc');

  const handleSort = (key: string) => {
    if (sortKey === key) {
      setSortDirection(sortDirection === 'asc' ? 'desc' : 'asc');
    } else {
      setSortKey(key);
      setSortDirection('asc');
    }
  };

  const sortedData = (Array.isArray(symbols) ? symbols : []).sort((a, b) => {
        const aValue = sortKey.includes('.')
          ? sortKey.split('.').reduce((acc, part) => acc && typeof acc === 'object' ? (acc as Record<string, unknown>)[part] : undefined, a as unknown)
          : (a as unknown as Record<string, unknown>)[sortKey];
        const bValue = sortKey.includes('.')
          ? sortKey.split('.').reduce((acc, part) => acc && typeof acc === 'object' ? (acc as Record<string, unknown>)[part] : undefined, b as unknown)
          : (b as unknown as Record<string, unknown>)[sortKey];

        if (aValue === undefined || bValue === undefined || aValue === null || bValue === null) return 0;
        if (aValue < bValue) return sortDirection === 'asc' ? -1 : 1;
        if (aValue > bValue) return sortDirection === 'asc' ? 1 : -1;
        return 0;
      });

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold tracking-tight">Symbols</h1>
          <p className="text-muted-foreground">
            Browse all available cryptocurrency symbols
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
            Failed to load symbols: {(error as Error).message}
          </AlertDescription>
        </Alert>
      )}

      <Card>
        <CardHeader>
          <CardTitle>Filters</CardTitle>
          <CardDescription>
            Filter symbols by category or search by name
          </CardDescription>
        </CardHeader>
        <CardContent>
          <div className="flex flex-col gap-4 md:flex-row md:items-center md:justify-between">
            <div className="flex items-center gap-4">
              <CategoryFilter
                value={category}
                onChange={setCategory}
                variant="tabs"
              />
            </div>
            <div className="relative w-full md:w-64">
              <Search className="absolute left-2 top-2.5 h-4 w-4 text-muted-foreground" />
              <Input
                placeholder="Search symbols..."
                value={search}
                onChange={(e) => setSearch(e.target.value)}
                className="pl-8"
              />
            </div>
          </div>
        </CardContent>
      </Card>

      <DataTable
        columns={columns}
        data={sortedData}
        isLoading={isLoading}
        sortKey={sortKey}
        sortDirection={sortDirection}
        onSort={handleSort}
        emptyMessage="No symbols found matching your criteria"
      />

      {!isLoading && Array.isArray(symbols) && (
        <p className="text-sm text-muted-foreground text-center">
          Showing {symbols.length} symbol{symbols.length !== 1 ? 's' : ''}
        </p>
      )}
    </div>
  );
}
