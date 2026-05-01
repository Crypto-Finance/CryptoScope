'use client';

import { useState } from 'react';
import { useStats } from '@/hooks/useStats';
import { StatCard } from '@/components/StatCard';
import { CategoryFilter, type CategoryValue } from '@/components/CategoryFilter';
import { Button } from '@/components/ui/button';
import { Card, CardContent, CardHeader, CardTitle, CardDescription } from '@/components/ui/card';
import { Alert, AlertDescription } from '@/components/ui/alert';
import { Badge } from '@/components/ui/badge';
import { RefreshCw, Coins } from 'lucide-react';
import { cn } from '@/lib/utils';

export default function StatsPage() {
  const [category, setCategory] = useState<CategoryValue>('all');

  const {
    data: stats = {
      total_count: 0,
      by_category: [],
      by_contract_type: [],
    },
    isLoading,
    error,
    refetch,
    isRefetching,
  } = useStats({
    category: category === 'all' ? undefined : category,
  });

  return (
    <div className="space-y-6">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-3xl font-bold tracking-tight">Statistics</h1>
          <p className="text-muted-foreground">
            Market overview and statistics
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
            Failed to load statistics: {(error as Error).message}
          </AlertDescription>
        </Alert>
      )}

      <Card>
        <CardHeader>
          <CardTitle>Category Filter</CardTitle>
          <CardDescription>
            Filter statistics by contract category
          </CardDescription>
        </CardHeader>
        <CardContent>
          <CategoryFilter
            value={category}
            onChange={setCategory}
            variant="tabs"
          />
        </CardContent>
      </Card>

      {isLoading ? (
        <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
          {Array.from({ length: 6 }).map((_, i) => (
            <Card key={i}>
              <CardHeader className="pb-2">
                <div className="h-4 w-24 bg-muted animate-pulse rounded" />
              </CardHeader>
              <CardContent>
                <div className="h-8 w-32 bg-muted animate-pulse rounded" />
              </CardContent>
            </Card>
          ))}
        </div>
      ) : stats ? (
        <>
          <div className="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
            <StatCard
              title="Total Symbols"
              value={stats.total_count}
              description="Across all categories"
              icon={<Coins className="h-4 w-4" />}
            />
          </div>

          <div className="grid gap-4 md:grid-cols-2">
            <Card>
              <CardHeader>
                <CardTitle>Category Breakdown</CardTitle>
                <CardDescription>
                  Distribution by category
                </CardDescription>
              </CardHeader>
              <CardContent>
                <div className="space-y-3">
                  {stats.by_category.map(({ category, count }) => (
                    <div key={category} className="flex items-center justify-between">
                      <div className="flex items-center gap-2">
                        <Badge variant="secondary" className="capitalize">
                          {category}
                        </Badge>
                      </div>
                      <div className="flex items-center gap-4">
                        <div className="w-32 bg-muted rounded-full h-2">
                          <div
                            className="bg-primary h-2 rounded-full"
                            style={{ width: `${stats.total_count > 0 ? (count / stats.total_count) * 100 : 0}%` }}
                          />
                        </div>
                        <span className="text-sm font-medium w-12 text-right">
                          {count}
                        </span>
                      </div>
                    </div>
                  ))}
                </div>
              </CardContent>
            </Card>

            <Card>
              <CardHeader>
                <CardTitle>Contract Type Breakdown</CardTitle>
                <CardDescription>
                  Distribution by contract type
                </CardDescription>
              </CardHeader>
              <CardContent>
                <div className="space-y-3">
                  {stats.by_contract_type.map(({ category, count }) => (
                    <div key={category} className="flex items-center justify-between">
                      <div className="flex items-center gap-2">
                        <Badge variant="outline">
                          {category}
                        </Badge>
                      </div>
                      <div className="flex items-center gap-4">
                        <div className="w-32 bg-muted rounded-full h-2">
                          <div
                            className="bg-primary h-2 rounded-full"
                            style={{ width: `${stats.total_count > 0 ? (count / stats.total_count) * 100 : 0}%` }}
                          />
                        </div>
                        <span className="text-sm font-medium w-12 text-right">
                          {count}
                        </span>
                      </div>
                    </div>
                  ))}
                </div>
              </CardContent>
            </Card>
          </div>
        </>
      ) : (
        <Alert>
          <AlertDescription>
            No statistics available. Make sure the backend API is running.
          </AlertDescription>
        </Alert>
      )}
    </div>
  );
}
