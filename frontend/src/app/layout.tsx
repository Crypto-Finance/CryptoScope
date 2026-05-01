import type { Metadata } from "next";
import { Geist, Geist_Mono } from "next/font/google";
import "./globals.css";
import { QueryProvider } from "@/lib/query-provider";
import Link from "next/link";

const geistSans = Geist({
  variable: "--font-geist-sans",
  subsets: ["latin"],
});

const geistMono = Geist_Mono({
  variable: "--font-geist-mono",
  subsets: ["latin"],
});

export const metadata: Metadata = {
  title: "CryptoScope",
  description: "Cryptocurrency data analysis and screening platform",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html
      lang="en"
      className={`${geistSans.variable} ${geistMono.variable} h-full antialiased`}
    >
      <body className="min-h-full flex flex-col bg-background">
        <QueryProvider>
          <header className="border-b">
            <div className="container mx-auto px-4 py-4">
              <nav className="flex items-center justify-between">
                <Link href="/" className="text-xl font-bold text-primary">
                  CryptoScope
                </Link>
                <div className="flex items-center gap-6">
                  <Link
                    href="/symbols"
                    className="text-sm font-medium text-muted-foreground hover:text-foreground transition-colors"
                  >
                    Symbols
                  </Link>
                  <Link
                    href="/screener"
                    className="text-sm font-medium text-muted-foreground hover:text-foreground transition-colors"
                  >
                    Screener
                  </Link>
                  <Link
                    href="/stats"
                    className="text-sm font-medium text-muted-foreground hover:text-foreground transition-colors"
                  >
                    Stats
                  </Link>
                </div>
              </nav>
            </div>
          </header>
          <main className="flex-1 container mx-auto px-4 py-8">
            {children}
          </main>
          <footer className="border-t py-6">
            <div className="container mx-auto px-4 text-center text-sm text-muted-foreground">
              CryptoScope &copy; {new Date().getFullYear()} - Cryptocurrency Data Platform
            </div>
          </footer>
        </QueryProvider>
      </body>
    </html>
  );
}
