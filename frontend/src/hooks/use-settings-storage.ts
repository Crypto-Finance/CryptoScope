import { useState, useEffect, useCallback } from 'react';
import { toast } from 'sonner';
import type { ExchangeName, RefreshInterval, APIKeys } from '@/lib/settings-storage';
import {
  getStoredAPIKeys,
  saveAPIKeys,
  getStoredRefreshInterval,
  saveRefreshInterval,
  getDenseMode,
  saveDenseMode,
  EXCHANGE_KEY_MAP,
} from '@/lib/settings-storage';

/**
 * Custom hook for managing settings storage with React state.
 * Provides stateful access to API keys, refresh interval, and dense mode settings.
 * 
 * @returns Object containing settings state and update functions
 * 
 * @example
 * ```tsx
 * const {
 *   apiKeys,
 *   refreshInterval,
 *   denseMode,
 *   updateAPIKeys,
 *   updateRefreshInterval,
 *   updateDenseMode,
 * } = useSettingsStorage();
 * ```
 */
export function useSettingsStorage() {
  const [apiKeys, setApiKeys] = useState<APIKeys>(getEmptyAPIKeysPlaceholder());
  const [refreshInterval, setRefreshInterval] = useState<RefreshInterval>(10);
  const [denseMode, setDenseModeState] = useState(false);
  const [isLoaded, setIsLoaded] = useState(false);

  // Load settings on mount
  useEffect(() => {
    let mounted = true;

    const loadSettings = async () => {
      const keys = await getStoredAPIKeys();
      const interval = getStoredRefreshInterval();
      const dense = getDenseMode();

      if (mounted) {
        setApiKeys(keys);
        setRefreshInterval(interval);
        setDenseModeState(dense);
        setIsLoaded(true);
      }
    };

    loadSettings();

    return () => {
      mounted = false;
    };
  }, []);

  /**
   * Update API keys for an exchange.
   * @param exchange - Exchange name
   * @param keys - API key pair (and optional passphrase for OKX)
   * @returns true if save succeeded, false otherwise
   */
  const updateAPIKeys = useCallback(async (
    exchange: ExchangeName,
    keys: { apiKey: string; apiSecret: string; passphrase?: string }
  ): Promise<boolean> => {
    const exchangeKey = EXCHANGE_KEY_MAP[exchange];
    const newKeys = { ...apiKeys, [exchangeKey]: keys };
    
    try {
      await saveAPIKeys(newKeys);
      setApiKeys(newKeys);
      toast.success(`${exchange} API keys saved successfully`);
      return true;
    } catch (error) {
      console.error('Failed to save API keys:', error);
      toast.error(`Failed to save ${exchange} API keys. Please try again.`);
      return false;
    }
  }, [apiKeys]);

  /**
   * Update refresh interval.
   * @param interval - New refresh interval in seconds
   */
  const updateRefreshInterval = useCallback((interval: RefreshInterval) => {
    setRefreshInterval(interval);
    saveRefreshInterval(interval);
  }, []);

  /**
   * Update dense mode setting.
   * @param enabled - Whether to enable dense mode
   */
  const updateDenseMode = useCallback((enabled: boolean) => {
    setDenseModeState(enabled);
    saveDenseMode(enabled);
  }, []);

  return {
    apiKeys,
    refreshInterval,
    denseMode,
    isLoaded,
    updateAPIKeys,
    updateRefreshInterval,
    updateDenseMode,
  };
}

/**
 * Get empty API keys placeholder for initial state.
 * This is a synchronous version for useState initialization.
 */
function getEmptyAPIKeysPlaceholder(): APIKeys {
  return {
    bybit: { apiKey: '', apiSecret: '' },
    binance: { apiKey: '', apiSecret: '' },
    okx: { apiKey: '', apiSecret: '', passphrase: '' },
  };
}
