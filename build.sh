#!/bin/bash
# Game Boy模拟器构建脚本

set -e  # 遇到错误时退出

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 打印带颜色的消息
print_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
}

# 检查Rust是否安装
check_rust() {
    if ! command -v cargo &> /dev/null; then
        print_error "Rust未安装！请先安装Rust："
        echo "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
        exit 1
    fi
    print_success "Rust已安装"
}

# 检查代码格式
check_format() {
    print_info "检查代码格式..."
    if cargo fmt --check; then
        print_success "代码格式正确"
    else
        print_warning "代码格式需要调整，正在格式化..."
        cargo fmt
        print_success "代码已格式化"
    fi
}

# 运行代码检查
run_lint() {
    print_info "运行代码检查..."
    if cargo clippy -- -D warnings; then
        print_success "代码检查通过"
    else
        print_warning "代码检查发现问题，但继续构建..."
    fi
}

# 构建项目
build_project() {
    local mode=$1
    print_info "构建项目 ($mode)..."
    
    if [ "$mode" = "release" ]; then
        cargo build --release
    else
        cargo build
    fi
    
    print_success "构建完成！"
}

# 运行测试
run_tests() {
    print_info "运行测试..."
    if cargo test; then
        print_success "所有测试通过"
    else
        print_error "测试失败"
        exit 1
    fi
}

# 运行模拟器
run_emulator() {
    local mode=$1
    print_info "启动Game Boy模拟器..."
    
    if [ "$mode" = "release" ]; then
        cargo run --release
    else
        cargo run
    fi
}

# 显示帮助信息
show_help() {
    echo "Game Boy模拟器构建脚本"
    echo ""
    echo "用法: $0 [选项]"
    echo ""
    echo "选项:"
    echo "  build       构建项目"
    echo "  run         构建并运行"
    echo "  test        运行测试"
    echo "  check       代码检查"
    echo "  format      格式化代码"
    echo "  clean       清理构建文件"
    echo "  release     构建发布版本"
    echo "  run-release 运行发布版本"
    echo "  all         完整构建流程（检查+测试+构建）"
    echo "  help        显示此帮助信息"
    echo ""
    echo "示例:"
    echo "  $0 run      - 构建并运行"
    echo "  $0 release  - 构建优化版本"
    echo "  $0 all      - 完整构建流程"
}

# 完整构建流程
full_build() {
    print_info "开始完整构建流程..."
    
    check_rust
    check_format
    run_lint
    build_project "debug"
    run_tests
    print_success "完整构建流程完成！"
}

# 主函数
main() {
    case "${1:-run}" in
        "build")
            check_rust
            build_project "debug"
            ;;
        "run")
            check_rust
            build_project "debug"
            run_emulator "debug"
            ;;
        "test")
            check_rust
            run_tests
            ;;
        "check")
            check_rust
            run_lint
            ;;
        "format")
            check_rust
            check_format
            ;;
        "clean")
            print_info "清理构建文件..."
            cargo clean
            print_success "清理完成！"
            ;;
        "release")
            check_rust
            build_project "release"
            ;;
        "run-release")
            check_rust
            build_project "release"
            run_emulator "release"
            ;;
        "all")
            full_build
            ;;
        "help"|"-h"|"--help")
            show_help
            ;;
        *)
            print_error "未知选项: $1"
            show_help
            exit 1
            ;;
    esac
}

# 运行主函数
main "$@"
