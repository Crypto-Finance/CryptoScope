'use client';

import { useState, useEffect, useMemo } from 'react';
import { useSymbols } from '@/hooks/useSymbols';
import { DataTable, type Column } from '@/components/DataTable';
import { CategoryFilter, type CategoryValue } from '@/components/CategoryFilter';
import { Button } from '@/components/ui/button';
import { Alert, AlertDescription } from '@/components/ui/alert';
import { Badge } from '@/components/ui/badge';
import { RefreshCw } from 'lucide-react';
import { TerminalInput, FilterChip, StitchCard, StitchCardHeader, StitchCardContent } from '@/components/stitch';
import { CardTitle, CardDescription } from '@/components/ui/card';
import { cn } from '@/lib/utils';
import type { CryptoSymbol } from '@/lib/api';
import { getSortedData } from '@/lib/sort';
import { useSortState } from '@/hooks/useSortState';

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

  // Track active filters for FilterChip display
  const activeFilters = useMemo(() => {
    const filters: Array<{ key: string; label: string; onDismiss: () => void }> = [];
    
    if (category !== 'all') {
      filters.push({
        key: 'category',
        label: `Category: ${category}`,
        onDismiss: () => setCategory('all'),
      });
    }
    
    if (debouncedSearch) {
      filters.push({
        key: 'search',
        label: `Search: "${debouncedSearch}"`,
        onDismiss: () => {
          setSearch('');
          setDebouncedSearch('');
        },
      });
    }
    
    return filters;
  }, [category, debouncedSearch]);

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

  const { sortKey, sortDirection, handleSort } = useSortState({
    defaultKey: 'symbol',
    defaultDirection: 'asc',
  });

  const sortedData = getSortedData(
    Array.isArray(symbols) ? symbols : [],
    sortKey,
    sortDirection,
  );

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

      <StitchCard>
        <StitchCardHeader>
          <div>
            <CardTitle>Filters</CardTitle>
            <CardDescription>
              Filter symbols by category or search by name
            </CardDescription>
          </div>
        </StitchCardHeader>
        <StitchCardContent>
          <div className="flex flex-col gap-4">
            <div className="flex flex-col gap-4 md:flex-row md:items-center md:justify-between">
              <div className="flex items-center gap-4">
                <CategoryFilter
                  value={category}
                  onChange={setCategory}
                  variant="tabs"
                />
              </div>
              <div className="w-full md:w-80">
                <TerminalInput
                  placeholder="Search symbols..."
                  value={search}
                  onChange={(e) => setSearch(e.target.value)}
                  variant="compact"
                />
              </div>
            </div>
            
            {/* Active Filters */}
            {activeFilters.length > 0 && (
              <div className="flex flex-wrap items-center gap-2">
                <span className="text-xs text-muted-foreground">Active filters:</span>
                {activeFilters.map((filter) => (
                  <FilterChip
                    key={filter.key}
                    label={filter.label}
                    onDismiss={filter.onDismiss}
                  />
                ))}
                <Button
                  variant="ghost"
                  size="sm"
                  onClick={() => {
                    setCategory('all');
                    setSearch('');
                    setDebouncedSearch('');
                  }}
                  className="h-6 text-xs hover:bg-muted"
                >
                  Clear all
                </Button>
              </div>
            )}
          </div>
        </StitchCardContent>
      </StitchCard>

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
