'use client';

import { useState, useMemo, useEffect, useRef } from 'react';
import { DataTable } from '@/components/DataTable';
import { Button } from '@/components/ui/button';
import { Badge } from '@/components/ui/badge';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import { Card, CardHeader, CardTitle, CardDescription } from '@/components/ui/card';
import { RefreshCw, DollarSign, AlertCircle, ArrowLeftRight, TrendingUp, TrendingDown } from 'lucide-react';
import { StatusPip, StitchCard, StitchCardHeader, StitchCardContent } from '@/components/stitch';
import { cn } from '@/lib/utils';
import { getSortedData } from '@/lib/sort';
import { useSortState } from '@/hooks/useSortState';
import { generateMockComparisonData } from '@/lib/mock-data';
import { createComparisonColumns } from './columns';

// Types for cross-exchange comparison
export interface ExchangePrice {
  exchange: string;
  symbol: string;
  price: number;
  volume_24h: number;
  spread: number;
  spread_percent: number;
  last_update: string;
  status: 'connected' | 'connecting' | 'disconnected' | 'error' | 'simulated';
}

export interface ComparisonRow {
  symbol: string;
  bybit_price: number;
  binance_price: number;
  okx_price: number;
  avg_price: number;
  price_diff: number;
  price_diff_percent: number;
  bybit_volume: number;
  binance_volume: number;
  okx_volume: number;
  total_volume: number;
  arbitrage_opportunity: boolean;
  best_exchange: string;
  worst_exchange: string;
  status: 'connected' | 'connecting' | 'disconnected' | 'error' | 'simulated';
}

const EXCHANGES = [
  { value: 'ALL', label: 'All Symbols' },
  { value: 'BTCUSDT', label: 'BTC/USDT' },
  { value: 'ETHUSDT', label: 'ETH/USDT' },
  { value: 'SOLUSDT', label: 'SOL/USDT' },
  { value: 'XRPUSDT', label: 'XRP/USDT' },
  { value: 'BNBUSDT', label: 'BNB/USDT' },
];

const ARBITRAGE_THRESHOLD = 0.1; // 0.1% price difference threshold

export default function ComparePage() {
  const [selectedSymbol, setSelectedSymbol] = useState('ALL');
  const { sortKey: sortField, sortDirection, handleSort } = useSortState({
    defaultKey: 'price_diff_percent',
    defaultDirection: 'desc',
  });
  const [isRefreshing, setIsRefreshing] = useState(false);
  const refreshTimeoutRef = useRef<NodeJS.Timeout | null>(null);

  // Mock data - in production, this would come from an API
  const [comparisonData, setComparisonData] = useState<ComparisonRow[]>(
    generateMockComparisonData('ALL')
  );

  const handleRefresh = () => {
    setIsRefreshing(true);
    // Clear any existing timeout to prevent multiple updates
    if (refreshTimeoutRef.current) {
      clearTimeout(refreshTimeoutRef.current);
    }
    // Simulate API call
    refreshTimeoutRef.current = setTimeout(() => {
      setComparisonData(generateMockComparisonData(selectedSymbol));
      setIsRefreshing(false);
      refreshTimeoutRef.current = null;
    }, 500);
  };

  // Cleanup timeout on component unmount
  useEffect(() => {
    return () => {
      if (refreshTimeoutRef.current) {
        clearTimeout(refreshTimeoutRef.current);
      }
    };
  }, []);

  const handleSymbolChange = (value: string) => {
    setSelectedSymbol(value);
    setComparisonData(generateMockComparisonData(value));
  };

  const sortedData = getSortedData(comparisonData, sortField, sortDirection);

  const arbitrageOpportunities = comparisonData.filter((row) => row.arbitrage_opportunity);
  const avgPriceDiff = comparisonData.length > 0
    ? comparisonData.reduce((sum, row) => sum + row.price_diff_percent, 0) / comparisonData.length
    : 0;

  const columns = useMemo(() => createComparisonColumns(ARBITRAGE_THRESHOLD), []);

  return (
    <div className="space-y-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold tracking-tight">
            Cross-Exchange Comparison
            <Badge variant="outline" className="ml-3 border-amber-500 text-amber-500">
              DEMO MODE
            </Badge>
          </h1>
          <p className="text-muted-foreground">
            Compare prices across multiple exchanges to find arbitrage opportunities
          </p>
        </div>
        <Button
          variant="outline"
          onClick={handleRefresh}
          disabled={isRefreshing}
        >
          <RefreshCw className={cn('mr-2 h-4 w-4', isRefreshing && 'animate-spin')} />
          Refresh
        </Button>
      </div>

      {/* Simulated Data Banner */}
      <div className="rounded-lg border border-amber-500/50 bg-amber-500/10 px-4 py-3 mb-6">
        <div className="flex items-center gap-2">
          <AlertCircle className="h-5 w-5 text-amber-500" />
          <span className="text-sm font-semibold text-amber-400">SIMULATED DATA</span>
          <span className="text-sm text-amber-300/80">
            Prices shown are randomly generated for demo purposes. Not real market data.
          </span>
        </div>
      </div>

      {/* Stats Cards */}
      <div className="grid gap-4 md:grid-cols-3">
        <StitchCard>
          <StitchCardHeader showBorder={false}>
            <div className="flex items-center gap-2">
              <StatusPip variant="connected" size="sm" />
              <span className="text-sm font-medium text-emerald-500">Exchanges Tracked</span>
            </div>
            <ArrowLeftRight className="h-4 w-4 text-muted-foreground" />
          </StitchCardHeader>
          <StitchCardContent variant="dense">
            <div className="text-2xl font-bold">3</div>
            <p className="text-xs text-muted-foreground">
              Bybit, Binance, OKX
            </p>
          </StitchCardContent>
        </StitchCard>

        <StitchCard>
          <StitchCardHeader showBorder={false}>
            <div className="flex items-center gap-2">
              <StatusPip variant={arbitrageOpportunities.length > 0 ? 'connected' : 'disconnected'} size="sm" />
              <span className={cn(
                'text-sm font-medium',
                arbitrageOpportunities.length > 0 ? 'text-amber-500' : 'text-muted-foreground'
              )}>
              Arbitrage Opportunities
            </span>
          </div>
          <DollarSign className="h-4 w-4 text-muted-foreground" />
        </StitchCardHeader>
        <StitchCardContent variant="dense">
          <div className="text-2xl font-bold">{arbitrageOpportunities.length}</div>
          <p className="text-xs text-muted-foreground">
            Price diff &gt; 0.1%
          </p>
        </StitchCardContent>
        </StitchCard>

        <StitchCard>
          <StitchCardHeader showBorder={false}>
            <div className="flex items-center gap-2">
              <StatusPip variant="connected" size="sm" />
              <span className="text-sm font-medium text-cyan-500">Avg Price Diff</span>
            </div>
            <TrendingUp className="h-4 w-4 text-muted-foreground" />
          </StitchCardHeader>
          <StitchCardContent variant="dense">
            <div className="text-2xl font-bold">{avgPriceDiff.toFixed(3)}%</div>
            <p className="text-xs text-muted-foreground">
              Across all symbols
            </p>
          </StitchCardContent>
        </StitchCard>
      </div>

      {/* Controls */}
      <Card>
        <CardHeader>
          <div className="flex items-center justify-between">
            <div>
              <CardTitle>Symbol Filter</CardTitle>
              <CardDescription>
                Select a symbol to compare across exchanges
              </CardDescription>
            </div>
            <Select value={selectedSymbol} onValueChange={(v) => v && handleSymbolChange(v)}>
              <SelectTrigger className="w-[200px]">
                <SelectValue placeholder="Select symbol" />
              </SelectTrigger>
              <SelectContent>
                {EXCHANGES.map((exchange) => (
                  <SelectItem key={exchange.value} value={exchange.value}>
                    {exchange.label}
                  </SelectItem>
                ))}
              </SelectContent>
            </Select>
          </div>
        </CardHeader>
      </Card>

      {/* Comparison Table */}
      <DataTable
        columns={columns}
        data={sortedData}
        isLoading={false}
        sortKey={sortField}
        sortDirection={sortDirection}
        onSort={handleSort}
        emptyMessage="No comparison data available"
      />

      {/* Legend */}
      <Card>
        <CardHeader>
          <CardTitle className="text-base">Legend</CardTitle>
          <CardDescription>
            Understanding the comparison metrics
          </CardDescription>
        </CardHeader>
        <div className="px-6 pb-4 space-y-2 text-sm">
          <div className="flex items-center gap-2">
            <Badge className="bg-amber-500 hover:bg-amber-600 text-white">
              <ArrowLeftRight className="mr-1 h-3 w-3" />
              Arbitrage
            </Badge>
            <span className="text-muted-foreground">
              Price difference exceeds 0.1% threshold
            </span>
          </div>
          <div className="flex items-center gap-2">
            <StatusPip variant="connected" size="sm" />
            <span className="text-muted-foreground">
              Exchange connection status (real-time)
            </span>
          </div>
          <div className="flex items-center gap-2">
            <div className="flex items-center gap-1 text-emerald-600">
              <TrendingUp className="h-3 w-3" />
              <span className="text-xs">Best Price</span>
            </div>
            <span className="text-muted-foreground">
              Exchange with lowest price (best for buying)
            </span>
          </div>
          <div className="flex items-center gap-2">
            <div className="flex items-center gap-1 text-red-500">
              <TrendingDown className="h-3 w-3" />
              <span className="text-xs">Worst Price</span>
            </div>
            <span className="text-muted-foreground">
              Exchange with highest price (best for selling)
            </span>
          </div>
        </div>
      </Card>

      {/* Footer Info */}
      {!isRefreshing && (
        <p className="text-sm text-muted-foreground text-center">
          Showing {comparisonData.length} symbol{comparisonData.length !== 1 ? 's' : ''} • 
          Data updates every 10 seconds
        </p>
      )}
    </div>
  );
}
