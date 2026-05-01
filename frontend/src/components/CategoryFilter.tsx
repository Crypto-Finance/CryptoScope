'use client';

import { Tabs, TabsList, TabsTrigger } from '@/components/ui/tabs';
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from '@/components/ui/select';
import { cn } from '@/lib/utils';

export type CategoryValue = 'linear' | 'inverse' | 'spot' | 'all';

interface CategoryFilterProps {
  value: CategoryValue;
  onChange: (value: CategoryValue) => void;
  variant?: 'tabs' | 'select';
  className?: string;
}

const categories: { value: CategoryValue; label: string }[] = [
  { value: 'all', label: 'All' },
  { value: 'linear', label: 'Linear' },
  { value: 'inverse', label: 'Inverse' },
  { value: 'spot', label: 'Spot' },
];

export function CategoryFilter({
  value,
  onChange,
  variant = 'tabs',
  className,
}: CategoryFilterProps) {
  if (variant === 'select') {
    return (
      <Select value={value} onValueChange={(v) => onChange(v as CategoryValue)}>
        <SelectTrigger className={cn('w-[150px]', className)}>
          <SelectValue placeholder="Select category" />
        </SelectTrigger>
        <SelectContent>
          {categories.map((category) => (
            <SelectItem key={category.value} value={category.value}>
              {category.label}
            </SelectItem>
          ))}
        </SelectContent>
      </Select>
    );
  }

  return (
    <Tabs value={value} onValueChange={(v) => onChange(v as CategoryValue)} className={className}>
      <TabsList>
        {categories.map((category) => (
          <TabsTrigger key={category.value} value={category.value}>
            {category.label}
          </TabsTrigger>
        ))}
      </TabsList>
    </Tabs>
  );
}
