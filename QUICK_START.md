# 🚀 LLM Dev Agent - Quick Start Guide

## ⚡ TL;DR - Just Run This
```bash
cd /home/rolan/code/agent/llm-dev-agent
npm run dev
```

## 🎯 What to Look For

### ✅ **CORRECT: Desktop Application**
- **Window Title**: "LLM Dev Agent - Desktop Mode ✅"  
- **Welcome Message**: "🚀 **Tauri Desktop Mode** - Full functionality enabled!"
- **Green Border**: Left side of header
- **File Operations**: Work perfectly
- **Claude Integration**: Full tool access

### ⚠️ **INCORRECT: Browser Tab** 
- **Window Title**: "LLM Dev Agent - Browser Mode ⚠️"
- **Welcome Message**: "🌐 **Browser Fallback Mode** - Limited functionality"  
- **Yellow Background**: Header has warning colors
- **CORS Errors**: "Load failed" when chatting
- **No File Operations**: Only basic chat

## 🔧 Troubleshooting

### Problem: Getting "Browser Mode" instead of "Desktop Mode"
**Solution**: Look for the **native desktop application window** that Tauri opens. Don't use any browser tabs.

### Problem: "Load failed" or CORS errors
**Solution**: You're in browser mode. Close browser and find the desktop app window.

### Problem: No desktop window appears
```bash
# Kill any stuck processes
pkill -f tauri

# Try again
npm run dev

# Should see: "Tauri app initialized successfully"
# Then a desktop window should appear
```

## 🎬 Test Commands

Once you see **"Desktop Mode ✅"**, try these:

1. **"Hello!"** → Should get normal Claude response
2. **"List files in current directory"** → Should show actual files  
3. **"Read the file package.json"** → Should show file contents

## 📞 Success Indicators

✅ Window title shows "Desktop Mode ✅"  
✅ Green border on header  
✅ File operations work  
✅ No CORS errors  
✅ Claude has tool access  

---

**If you see ⚠️ Browser Mode**: Close browser, find the desktop app window that `npm run dev` opened!