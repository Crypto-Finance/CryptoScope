import type { Metadata } from "next";
import { Inter } from "next/font/google";
import "./globals.css";
import { QueryProvider } from "@/lib/query-provider";
import { StitchProvider } from "@/lib/stitch";
import { AppShell } from "@/components/layout";
import { Toaster } from "@/components/ui/sonner";

const inter = Inter({
  variable: "--font-inter",
  subsets: ["latin"],
  display: "swap",
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
      className={`${inter.variable} h-full antialiased`}
      suppressHydrationWarning
    >
      <body className="h-full bg-background">
        <StitchProvider>
          <QueryProvider>
            <AppShell connectionStatus="connected">
              {children}
              <Toaster />
            </AppShell>
          </QueryProvider>
        </StitchProvider>
      </body>
    </html>
  );
}
