import Link from "next/link";
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from "@/components/ui/card";
import { Button } from "@/components/ui/button";
import { Badge } from "@/components/ui/badge";

export default function Home() {
  return (
    <div className="space-y-8">
      <div className="space-y-2">
        <h1 className="text-4xl font-bold tracking-tight">Dashboard</h1>
        <p className="text-muted-foreground">
          Welcome to CryptoScope - Your cryptocurrency data analysis platform
        </p>
      </div>

      <div className="grid gap-6 md:grid-cols-2 lg:grid-cols-3">
        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              Symbols
              <Badge variant="secondary">Browse</Badge>
            </CardTitle>
            <CardDescription>
              Explore all available cryptocurrency symbols across different exchanges
            </CardDescription>
          </CardHeader>
          <CardContent>
            <div className="space-y-4">
              <ul className="text-sm text-muted-foreground space-y-1">
                <li>• View symbol details</li>
                <li>• Check current prices</li>
                <li>• Filter by exchange</li>
              </ul>
              <Link href="/symbols" className="block">
                <Button className="w-full">View Symbols</Button>
              </Link>
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              Screener
              <Badge variant="secondary">Filter</Badge>
            </CardTitle>
            <CardDescription>
              Screen cryptocurrencies based on custom criteria
            </CardDescription>
          </CardHeader>
          <CardContent>
            <div className="space-y-4">
              <ul className="text-sm text-muted-foreground space-y-1">
                <li>• Filter by market cap</li>
                <li>• Filter by sector/industry</li>
                <li>• Filter by price range</li>
              </ul>
              <Link href="/screener" className="block">
                <Button className="w-full">Open Screener</Button>
              </Link>
            </div>
          </CardContent>
        </Card>

        <Card>
          <CardHeader>
            <CardTitle className="flex items-center gap-2">
              Statistics
              <Badge variant="secondary">Overview</Badge>
            </CardTitle>
            <CardDescription>
              Get an overview of available data and statistics
            </CardDescription>
          </CardHeader>
          <CardContent>
            <div className="space-y-4">
              <ul className="text-sm text-muted-foreground space-y-1">
                <li>• Total symbols count</li>
                <li>• Exchange distribution</li>
                <li>• Asset class breakdown</li>
              </ul>
              <Link href="/stats" className="block">
                <Button className="w-full">View Stats</Button>
              </Link>
            </div>
          </CardContent>
        </Card>
      </div>

      <Card>
        <CardHeader>
          <CardTitle>Quick Start Guide</CardTitle>
          <CardDescription>Get started with CryptoScope in minutes</CardDescription>
        </CardHeader>
        <CardContent>
          <ol className="space-y-3 text-sm">
            <li className="flex items-start gap-3">
              <Badge variant="outline" className="mt-0.5">1</Badge>
              <span>
                Start by exploring the <Link href="/symbols" className="text-primary hover:underline">Symbols</Link> page to see all available cryptocurrencies
              </span>
            </li>
            <li className="flex items-start gap-3">
              <Badge variant="outline" className="mt-0.5">2</Badge>
              <span>
                Use the <Link href="/screener" className="text-primary hover:underline">Screener</Link> to filter symbols based on your criteria
              </span>
            </li>
            <li className="flex items-start gap-3">
              <Badge variant="outline" className="mt-0.5">3</Badge>
              <span>
                Check <Link href="/stats" className="text-primary hover:underline">Statistics</Link> for an overview of the data coverage
              </span>
            </li>
          </ol>
        </CardContent>
      </Card>
    </div>
  );
}
