import { useState, useRef, useCallback, useEffect } from 'react';

interface UseSaveIndicatorOptions {
  duration?: number;
}

export function useSaveIndicator({ duration = 2000 }: UseSaveIndicatorOptions = {}) {
  const [isSaved, setIsSaved] = useState(false);
  const timeoutRef = useRef<ReturnType<typeof setTimeout> | null>(null);

  useEffect(() => {
    return () => {
      if (timeoutRef.current) {
        clearTimeout(timeoutRef.current);
      }
    };
  }, []);

  const markSaved = useCallback(() => {
    if (timeoutRef.current) {
      clearTimeout(timeoutRef.current);
    }
    setIsSaved(true);
    timeoutRef.current = setTimeout(() => {
      setIsSaved(false);
      timeoutRef.current = null;
    }, duration);
  }, [duration]);

  return { isSaved, markSaved };
}
