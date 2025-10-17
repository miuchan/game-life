# Game Boy模拟器架构文档

## 🏗️ 项目结构

```
gameboy-emulator/
├── src/
│   ├── lib.rs                 # 库入口点
│   ├── main.rs                # 主程序入口
│   ├── cpu/                   # CPU模块
│   │   ├── mod.rs            # CPU模块入口
│   │   ├── cpu.rs            # CPU核心逻辑
│   │   ├── registers.rs      # 寄存器操作
│   │   └── flags.rs          # 标志位处理
│   ├── memory/                # 内存模块
│   │   ├── mod.rs            # 内存模块入口
│   │   └── bus.rs            # 内存总线
│   ├── instructions/          # 指令模块
│   │   ├── mod.rs            # 指令模块入口
│   │   ├── instruction.rs    # 指令定义
│   │   ├── arithmetic.rs     # 算术指令
│   │   ├── load.rs           # 加载指令
│   │   └── jump.rs           # 跳转指令
│   └── emulator/              # 模拟器核心
│       ├── mod.rs            # 模拟器模块入口
│       └── gameboy.rs        # Game Boy模拟器
├── Cargo.toml                 # 项目配置
├── Makefile                   # 构建脚本
├── build.sh                   # 高级构建脚本
├── quick-run.sh              # 快速运行脚本
├── README.md                  # 项目说明
└── ARCHITECTURE.md            # 架构文档
```

## 🧩 模块设计

### CPU模块 (`src/cpu/`)

#### `cpu.rs` - CPU核心
- **职责**: CPU执行逻辑、指令调度
- **主要功能**:
  - 指令执行 (`step()`)
  - 指令分发 (`execute()`)
  - 算术运算 (`add()`, `sub()`, `inc()`, `dec()`)

#### `registers.rs` - 寄存器管理
- **职责**: 8位和16位寄存器操作
- **主要功能**:
  - 8位寄存器访问 (`get_register()`, `set_register()`)
  - 16位寄存器对操作 (`get_bc()`, `set_bc()`, 等)
  - 寄存器枚举定义

#### `flags.rs` - 标志位处理
- **职责**: CPU标志位管理
- **主要功能**:
  - 标志位设置和获取
  - 字节转换 (`From<u8>`, `From<FlagsRegister>`)
  - 标志位重置

### 内存模块 (`src/memory/`)

#### `bus.rs` - 内存总线
- **职责**: 内存读写操作
- **主要功能**:
  - 字节读写 (`read_byte()`, `write_byte()`)
  - 字读写 (`read_word()`, `write_word()`)
  - 程序加载 (`load_program()`)

### 指令模块 (`src/instructions/`)

#### `instruction.rs` - 指令定义
- **职责**: 指令枚举和解码
- **主要功能**:
  - 指令定义 (`Instruction` 枚举)
  - 指令解码 (`from_byte()`)
  - 指令名称获取

#### `arithmetic.rs` - 算术指令
- **职责**: 算术指令目标定义
- **主要功能**:
  - 算术目标枚举 (`ArithmeticTarget`)

#### `load.rs` - 加载指令
- **职责**: 数据传输指令定义
- **主要功能**:
  - 8位加载目标/源 (`LoadTarget`, `LoadSource`)
  - 16位加载目标/源 (`LoadTarget16`, `LoadSource16`)

#### `jump.rs` - 跳转指令
- **职责**: 控制流指令定义
- **主要功能**:
  - 跳转目标枚举 (`JumpTarget`)

### 模拟器核心 (`src/emulator/`)

#### `gameboy.rs` - Game Boy模拟器
- **职责**: 模拟器高级接口
- **主要功能**:
  - 模拟器创建 (`new()`)
  - 程序加载 (`load_program()`)
  - 指令执行 (`step()`, `run_steps()`)
  - 状态获取 (`get_cpu_state()`)

## 🔄 数据流

```
main.rs
    ↓
GameBoy (emulator/gameboy.rs)
    ↓
CPU (cpu/cpu.rs)
    ↓
MemoryBus (memory/bus.rs)
    ↓
Instructions (instructions/*.rs)
```

## 🎯 设计原则

### 1. **单一职责原则**
- 每个模块只负责一个特定功能
- CPU模块只处理CPU相关逻辑
- 内存模块只处理内存操作
- 指令模块只处理指令定义

### 2. **模块化设计**
- 清晰的模块边界
- 松耦合的模块间关系
- 易于测试和维护

### 3. **错误处理**
- 使用 `Result<T, String>` 进行错误处理
- 清晰的错误信息
- 优雅的错误传播

### 4. **可扩展性**
- 易于添加新指令
- 易于添加新功能
- 易于修改现有逻辑

## 🧪 测试策略

### 单元测试
- 每个模块都有对应的测试
- 测试覆盖核心功能
- 边界条件测试

### 集成测试
- 端到端功能测试
- 指令执行测试
- 状态一致性测试

## 🚀 性能优化

### 编译优化
- 发布模式优化 (`cargo build --release`)
- LTO链接时优化
- 代码剥离

### 运行时优化
- 高效的指令解码
- 快速的内存访问
- 最小化的内存分配

## 📈 扩展计划

### 短期目标
- [ ] 添加更多指令支持
- [ ] 完善错误处理
- [ ] 增加更多测试

### 中期目标
- [ ] 添加图形处理单元 (GPU)
- [ ] 添加音频处理
- [ ] 添加输入处理

### 长期目标
- [ ] 完整的Game Boy兼容性
- [ ] 性能优化
- [ ] 用户界面

## 🔧 开发指南

### 添加新指令
1. 在 `instructions/` 模块中定义指令
2. 在 `instruction.rs` 中添加解码逻辑
3. 在 `cpu.rs` 中添加执行逻辑
4. 添加相应的测试

### 修改CPU逻辑
1. 在 `cpu/cpu.rs` 中修改执行逻辑
2. 确保错误处理正确
3. 更新相关测试
4. 验证功能正确性

### 添加新功能
1. 确定功能所属模块
2. 在相应模块中实现
3. 添加公共接口
4. 编写测试用例

---

**这个架构设计确保了代码的可维护性、可扩展性和可测试性，为后续开发奠定了坚实的基础。** 🏗️✨
