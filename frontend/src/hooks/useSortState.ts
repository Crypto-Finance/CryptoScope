import { useState, useCallback } from 'react';
import type { SortDirection } from '@/lib/sort';

export interface UseSortStateOptions {
  defaultKey?: string;
  defaultDirection?: SortDirection;
}

/**
 * Custom hook for managing sort state in data tables.
 * Handles sort key and direction toggling logic.
 * 
 * @param options - Configuration options for default sort state
 * @returns Object containing sortKey, sortDirection, and handleSort function
 * 
 * @example
 * ```tsx
 * const { sortKey, sortDirection, handleSort } = useSortState({
 *   defaultKey: 'change_percent',
 *   defaultDirection: 'desc',
 * });
 * ```
 */
export function useSortState(options: UseSortStateOptions = {}) {
  const { defaultKey = '', defaultDirection = 'asc' } = options;
  const [sortKey, setSortKey] = useState(defaultKey);
  const [sortDirection, setSortDirection] = useState<SortDirection>(defaultDirection);

  const handleSort = useCallback((key: string) => {
    if (sortKey === key) {
      setSortDirection((prev) => (prev === 'asc' ? 'desc' : 'asc'));
    } else {
      setSortKey(key);
      setSortDirection('asc');
    }
  }, [sortKey]);

  return { sortKey, sortDirection, handleSort };
}
