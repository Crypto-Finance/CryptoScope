import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  // Output standalone for Docker deployment
  output: 'standalone',

  // Allow cross-origin requests from this host in development
  allowedDevOrigins: [''],

  // Proxy API requests to the Rust backend (development only)
  async rewrites() {
    return [
      {
        source: '/api/:path*',
        destination: 'http://localhost:3000/api/:path*',
      },
    ];
  },

  // Enable React Strict Mode for better development experience
  reactStrictMode: true,
};

export default nextConfig;
