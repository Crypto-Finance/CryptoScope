#!/bin/bash
# CryptoScope Docker Build Script
# Usage: ./scripts/build-docker.sh

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m'

print_status() {
    echo -e "${GREEN}✓${NC} $1"
}

print_header() {
    echo -e "${BLUE}========================================${NC}"
    echo -e "${BLUE}  CryptoScope Docker Build${NC}"
    echo -e "${BLUE}========================================${NC}"
    echo ""
}

cd "$PROJECT_ROOT"

print_header

# Create data directory if it doesn't exist
mkdir -p data

print_status "Building backend image..."
docker build -f Dockerfile.backend -t cryptoscope-backend:latest .

print_status "Building frontend image..."
docker build -f Dockerfile.frontend -t cryptoscope-frontend:latest .

echo ""
print_status "Build complete!"
echo ""
echo "Images created:"
docker images | grep cryptoscope
echo ""
echo "To run:"
echo "  docker compose up -d"
echo ""
echo "To test locally:"
echo "  docker run --rm -p 3000:3000 cryptoscope-backend:latest"
echo "  docker run --rm -p 3001:3001 -e NEXT_PUBLIC_API_URL=http://localhost:3000 cryptoscope-frontend:latest"
