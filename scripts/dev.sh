#!/bin/bash
# CryptoScope Development Helper Script
# Usage: ./scripts/dev.sh

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_header() {
    echo -e "${BLUE}========================================${NC}"
    echo -e "${BLUE}  CryptoScope Development Environment${NC}"
    echo -e "${BLUE}========================================${NC}"
    echo ""
}

print_status() {
    echo -e "${GREEN}✓${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠${NC} $1"
}

print_error() {
    echo -e "${RED}✗${NC} $1"
}

check_dependencies() {
    print_status "Checking dependencies..."
    
    # Check Rust
    if ! command -v cargo &> /dev/null; then
        print_error "Rust/Cargo not found. Please install Rust: https://rustup.rs/"
        exit 1
    fi
    print_status "Rust: $(cargo --version)"
    
    # Check Node.js
    if ! command -v node &> /dev/null; then
        print_error "Node.js not found. Please install Node.js: https://nodejs.org/"
        exit 1
    fi
    print_status "Node.js: $(node --version)"
    
    # Check npm
    if ! command -v npm &> /dev/null; then
        print_error "npm not found"
        exit 1
    fi
    print_status "npm: $(npm --version)"
    
    # Check Docker (optional)
    if command -v docker &> /dev/null; then
        print_status "Docker: $(docker --version)"
    else
        print_warning "Docker not found. Docker features will be unavailable."
    fi
    
    echo ""
}

start_backend() {
    print_status "Starting Rust backend..."
    cd "$PROJECT_ROOT"
    export RUST_LOG=debug
    cargo run &
    BACKEND_PID=$!
    echo $BACKEND_PID > /tmp/cryptoscope_backend.pid
    print_status "Backend started (PID: $BACKEND_PID)"
}

start_frontend() {
    print_status "Starting Next.js frontend..."
    cd "$PROJECT_ROOT/frontend"
    npm run dev &
    FRONTEND_PID=$!
    echo $FRONTEND_PID > /tmp/cryptoscope_frontend.pid
    print_status "Frontend started (PID: $FRONTEND_PID)"
}

stop_services() {
    print_status "Stopping services..."
    
    if [ -f /tmp/cryptoscope_backend.pid ]; then
        kill $(cat /tmp/cryptoscope_backend.pid) 2>/dev/null || true
        rm /tmp/cryptoscope_backend.pid
        print_status "Backend stopped"
    fi
    
    if [ -f /tmp/cryptoscope_frontend.pid ]; then
        kill $(cat /tmp/cryptoscope_frontend.pid) 2>/dev/null || true
        rm /tmp/cryptoscope_frontend.pid
        print_status "Frontend stopped"
    fi
}

show_help() {
    echo "Usage: $0 [command]"
    echo ""
    echo "Commands:"
    echo "  start       Start both backend and frontend"
    echo "  stop        Stop all running services"
    echo "  restart     Restart all services"
    echo "  backend     Start backend only"
    echo "  frontend    Start frontend only"
    echo "  docker-up   Start with Docker Compose"
    echo "  docker-down Stop Docker Compose"
    echo "  docker-dev  Start Docker development mode"
    echo "  clean       Clean build artifacts"
    echo "  help        Show this help message"
    echo ""
}

clean_artifacts() {
    print_status "Cleaning build artifacts..."
    
    cd "$PROJECT_ROOT"
    rm -rf target
    print_status "Cleared Rust target/"
    
    cd "$PROJECT_ROOT/frontend"
    rm -rf node_modules .next
    print_status "Cleared frontend node_modules/ and .next/"
    
    print_status "Clean complete!"
}

docker_up() {
    print_status "Starting Docker Compose..."
    cd "$PROJECT_ROOT"
    docker compose up -d
    print_status "Services started!"
    echo ""
    echo -e "${GREEN}Access:${NC}"
    echo "  Frontend: http://localhost:3001"
    echo "  Backend:  http://localhost:3000"
    echo "  Swagger:  http://localhost:3000/swagger-ui"
}

docker_down() {
    print_status "Stopping Docker Compose..."
    cd "$PROJECT_ROOT"
    docker compose down
    print_status "Services stopped!"
}

docker_dev() {
    print_status "Starting Docker development mode..."
    cd "$PROJECT_ROOT"
    docker compose -f docker-compose.dev.yml up -d
    print_status "Development services started!"
    echo ""
    echo -e "${GREEN}Access:${NC}"
    echo "  Frontend: http://localhost:3001"
    echo "  Backend:  http://localhost:3000"
    echo ""
    print_warning "Hot reload enabled - changes will auto-recompile"
}

# Main script
print_header

case "${1:-help}" in
    start)
        check_dependencies
        start_backend
        sleep 2
        start_frontend
        print_status "Development environment started!"
        echo ""
        echo "Press Ctrl+C to stop all services, or run: $0 stop"
        ;;
    stop)
        stop_services
        ;;
    restart)
        stop_services
        sleep 1
        start_backend
        sleep 2
        start_frontend
        ;;
    backend)
        check_dependencies
        start_backend
        ;;
    frontend)
        check_dependencies
        start_frontend
        ;;
    docker-up)
        docker_up
        ;;
    docker-down)
        docker_down
        ;;
    docker-dev)
        docker_dev
        ;;
    clean)
        clean_artifacts
        ;;
    help|*)
        show_help
        ;;
esac
