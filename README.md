# LudoLingua

<div align="center">

**Professional RPG Game Translation Platform**

*Transform your RPG Maker games with AI-powered translation and intelligent text management*

[![Latest Release](https://img.shields.io/github/v/release/KATBlackCoder/LudoLingua?style=for-the-badge&logo=github)](https://github.com/KATBlackCoder/LudoLingua/releases)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20Linux-blue?style=for-the-badge)](https://github.com/KATBlackCoder/LudoLingua/releases)
[![License](https://img.shields.io/badge/license-MIT-green?style=for-the-badge)](LICENSE)

</div>

## 🚀 **What is LudoLingua?**

LudoLingua is a **professional desktop application** that revolutionizes RPG game translation. Built with modern technologies (Rust + Nuxt 4), it provides game developers and translators with powerful AI-assisted translation tools, intelligent text management, and seamless project integration.

### **Key Capabilities**

- 🤖 **AI-Powered Translation** - Ollama (local) & RunPod (cloud) integration
- 📁 **Smart Project Loading** - Automatic RPG Maker MV/MZ detection & parsing
- 🎯 **Intelligent Text Extraction** - 12+ file types with context-aware processing
- 📚 **Advanced Glossary System** - Database-backed terminology management
- 🔄 **Bulk Operations** - Row selection, retranslation, and batch processing
- 📊 **Translation Management** - Full CRUD interface with filtering & search
- 💾 **Seamless Export** - Direct injection back to game files
- 🔔 **Native Notifications** - Desktop notifications for all operations
- 🔄 **Auto-Updates** - Professional update management with signed releases

## 🎮 **Supported Game Engines**

| Engine | Status | File Support | Notes |
|--------|--------|--------------|-------|
| **RPG Maker MV** | ✅ **Full Support** | 12+ file types | Complete implementation |
| **RPG Maker MZ** | ✅ **Full Support** | 12+ file types | Uses MV modules with `data/` paths |
| **Wolf RPG** | 🧪 **Experimental** | Core files | Basic support, expanding |

### **Supported File Types**
- **Core Data**: Actors, Items, Skills, Weapons, Armors, Classes, System, States, Enemies
- **Events**: CommonEvents, MapInfos, MapXXX files
- **Advanced**: All RPG Maker JSON data structures with intelligent parsing

## 📥 **Installation**

### **Quick Start - Download & Run**

**Windows**
```bash
# Download from GitHub Releases
LudoLingua_0.x.x_x64_en-US.msi
# Run installer → Done!
```

**Linux**
```bash
# Download AppImage
LudoLingua_0.x.x_amd64.AppImage
# Make executable & run
chmod +x LudoLingua_0.x.x_amd64.AppImage
./LudoLingua_0.x.x_amd64.AppImage
```

**🔗 [Download Latest Release](https://github.com/KATBlackCoder/LudoLingua/releases)**

### **Development Setup**

For contributors and advanced users:

```bash
# Clone repository
git clone https://github.com/KATBlackCoder/LudoLingua.git
cd LudoLingua

# Install dependencies
pnpm install

# Start development server
pnpm tauri dev

# For non-Ubuntu 22 Linux users (if you have display issues)
WEBKIT_DISABLE_COMPOSITING_MODE=1 GDK_BACKEND=x11 pnpm tauri dev

# Build for production
pnpm tauri build

# For non-Ubuntu 22 Linux users (no AppImage will be generated)
WEBKIT_DISABLE_COMPOSITING_MODE=1 GDK_BACKEND=x11 pnpm tauri build
```

## 🛠️ **Technology Stack**

**Frontend**
- **Framework**: Nuxt 4.1.1 SPA with TypeScript
- **UI Library**: Nuxt UI v4.0.0-alpha.1 (Pro components)
- **State Management**: Pinia with Composition API
- **Styling**: Tailwind CSS with responsive design

**Backend**
- **Core**: Rust with Tokio async runtime
- **Desktop**: Tauri for cross-platform desktop apps
- **Database**: SQLite with manifest-based project tracking
- **AI Integration**: Unified LLM service layer

**AI Providers**
- **Ollama**: Local AI models (recommended)
- **RunPod**: Cloud GPU instances
- **Future**: Sugoi multilingual server support

## 🤖 **AI Translation Setup**

### **Option 1: Ollama (Recommended)**
```bash
# Install Ollama
curl -fsSL https://ollama.com/install.sh | sh

# Pull the recommended model
ollama pull qwen2.5:latest

# Start Ollama server
ollama serve
```
*LudoLingua automatically connects to `http://localhost:11434`*

**Recommended Model**: `qwen2.5:latest` - Superior multilingual capabilities, especially strong for Asian languages and technical translation

### **Option 2: RunPod**
1. Create account at [RunPod](https://runpod.io/)
2. Deploy a GPU pod with one of these models:
   - `qwen2.5:14b-instruct-q5_K_M` (Recommended)
   - `qwen3:14b-q4_K_M`
3. Copy the pod ID to LudoLingua settings
4. *LudoLingua automatically formats the URL*

**Recommended Models**: Qwen2.5 14B or Qwen3 14B - 128K context, 140+ languages support, excellent for complex RPG texts

**⚙️ Configure in Settings → AI Provider**

## 📚 **Advanced Glossary System**

LudoLingua features a **database-backed glossary system** that ensures translation consistency across your entire project.

### **Smart Term Management**

**Database Storage**
- **Location**: `ludolingua.db` in OS app data directory
- **Windows**: `%AppData%/ml.ludolingua.blackat/`
- **macOS**: `~/Library/Application Support/ml.ludolingua.blackat/`
- **Linux**: `~/.local/share/ml.ludolingua.blackat/`

**Term Categories**
- **Characters**: Proper names (people, monsters, places)
- **Essential Terms**: Game-specific vocabulary & currencies
- **Translation Rules**: Style guidelines & constraints
- **Locations**: Place names as common nouns
- **Time & Weather**: Temporal & weather terms
- **Mechanics**: System terms & game mechanics
- **Status Effects**: Buffs/debuffs & ailment names

### **Automatic Integration**
- **Context-Aware**: Terms automatically injected based on text type
- **Smart Filtering**: Only relevant terms included per translation
- **Consistent Results**: Same terminology across all translations
- **Easy Management**: Full CRUD interface with search & filtering

## Placeholders Reference

⚠️ **Important**: Any token enclosed in square brackets (e.g., `[COLOR_5]`) is a placeholder that encodes engine semantics. Do not translate, change, remove, or re-order these unless you know what you are doing. They will be restored on export.

### Formatting & Names
- `[COLOR_n]` → `\C[n]` or `\c[n]` (color index)
- `[NAME_n]` → `\N[n]` (name by index)
- `[NEWLINE_n]` → `\n[n]` (insert n newlines)
- `[ICON_n]` → `\i[n]` (icon index)
- `[FONT_n]` → `\f[n]` (font/face index)

### Variables, Items & Entities
- `[VARIABLE_…]` → `\V[` (variables)
- `[SWITCH_…]` → `\S[` (switches)
- `[ITEM_…]`/`[WEAPON_…]`/`[ARMOR_…]` → `\I[`/`\W[`/`\A[`
- `[ACTOR_…]` → `\P[` (actor reference)
- `[GOLD]`/`[CURRENCY]` → `\G`/`\$

### Control & Flow
- `[CTRL_DOT]`/`[CTRL_WAIT]`/`[CTRL_INSTANT]` → `\.`/`\|`/`^`
- `[CONDITIONAL_vX>Y]` → `en(v[X]>Y)`
- `[NEWLINE]` → actual newline `\n`

### Arguments & Whitespace
- `[ARG_n]` → `%n` (argument placeholder)
- `[FWSPC_n]`/`[SPC_n]`/`[TAB_n]` → n full-width spaces / n ASCII spaces / n tabs

## 🚀 **Quick Start Guide**

### **1. Download & Install**
Download the latest release for your platform from [GitHub Releases](https://github.com/KATBlackCoder/LudoLingua/releases)

### **2. Configure AI Provider**
- **Ollama**: Install locally, LudoLingua auto-connects
- **RunPod**: Enter pod ID, LudoLingua formats URL automatically

### **3. Load Your RPG Project**
- Click "Load Project" → Select your RPG Maker game folder
- LudoLingua automatically detects engine type (MV/MZ)
- Text extraction happens automatically

### **4. Translate & Manage**
- **AI Translation**: Bulk translate with context-aware prompts
- **Manual Editing**: Fine-tune translations in the editor
- **Glossary Management**: Add terms for consistency
- **Bulk Operations**: Select rows, retranslate, filter by length

### **5. Export Back to Game**
- Click "Export" → Translations injected directly to game files
- Files saved to `project/ludolingua/` folder
- Original files preserved with backup

## 📊 **Current Status**

**✅ Core Features**
- RPG Maker MV/MZ support
- AI translation (Ollama/RunPod)
- Database-backed glossary
- Smart text extraction
- Direct file export

**✅ Advanced Features**
- Bulk operations & row selection
- Text length filtering
- Native notifications
- Auto-updates with signing
- Translation management

**🧪 Experimental**
- Wolf RPG support
- Sugoi provider (planned)

## 🔧 **Advanced Configuration**

### **Prompt System**
LudoLingua uses intelligent prompt templates that adapt based on text type:

- **Development**: Templates loaded from filesystem for instant editing
- **Production**: Templates embedded at compile time for reliability
- **Context-Aware**: Different prompts for dialogue, system text, character names, etc.

### **Vocabulary Filtering**
Smart filtering ensures only relevant glossary terms are included:

- **Character/Dialogue**: Character names + essential terms
- **Skills/States**: Status effects + mechanics + essential terms  
- **Equipment**: Mechanics + essential terms
- **System/Class**: Mechanics + essential terms

### **Customization**
- **Add Sections**: Modify filtering arrays in `PromptBuilder::filter_vocabulary_sections`
- **New Prompt Types**: Add variants to `PromptType` enum and create template files
- **Vocabulary Management**: Edit `prompts/vocabularies.txt` with exact header matching

---

## 🤝 **Contributing**

We welcome contributions! Please see our [Contributing Guidelines](CONTRIBUTING.md) for details.

### **Development Setup**
```bash
git clone https://github.com/KATBlackCoder/LudoLingua.git
cd LudoLingua
pnpm install
pnpm tauri dev

# For non-Ubuntu 22 Linux users (if you have display issues)
WEBKIT_DISABLE_COMPOSITING_MODE=1 GDK_BACKEND=x11 pnpm tauri dev
```

### **Reporting Issues**
Found a bug? Please [open an issue](https://github.com/KATBlackCoder/LudoLingua/issues) with:
- Your operating system
- LudoLingua version
- Steps to reproduce
- Expected vs actual behavior

---

## 📄 **License**

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 🙏 **Acknowledgments**

- **RPG Maker Community** for inspiration and feedback
- **Tauri Team** for the excellent desktop framework
- **Nuxt Team** for the modern web framework
- **OpenAI & Ollama** for AI translation capabilities

---

<div align="center">

**Made with ❤️ for the RPG translation community**

[⭐ Star this repo](https://github.com/KATBlackCoder/LudoLingua) • [🐛 Report Bug](https://github.com/KATBlackCoder/LudoLingua/issues) • [💡 Request Feature](https://github.com/KATBlackCoder/LudoLingua/issues)

</div>