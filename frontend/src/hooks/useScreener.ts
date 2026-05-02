'use client';

import { useQuery } from '@tanstack/react-query';
import { screenerApi, type ScreenerItem } from '@/lib/api';

interface UseScreenerParams {
  exchange?: string;
  mode?: 'kline' | 'mark';
  top?: number;
  minChange?: number;
  enabled?: boolean;
  refetchInterval?: number | false;
}

export function useScreener(params: UseScreenerParams = {}) {
  const {
    exchange = 'bybit',
    mode = 'kline',
    top,
    minChange,
    enabled = true,
    refetchInterval = 10000,
  } = params;

  return useQuery<ScreenerItem[], Error>({
    queryKey: ['screener', exchange, mode, top, minChange],
    queryFn: () => screenerApi.screen({ exchange, mode, top, min_change: minChange }),
    enabled,
    refetchInterval,
    staleTime: 3000,
    retry: 2,
  });
}
