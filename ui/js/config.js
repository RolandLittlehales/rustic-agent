/**
 * Configuration file for LLM Dev Agent UI
 */

const CONFIG = {
    // API Configuration
    api: {
        baseUrl: 'http://localhost:3000', // Update with your backend URL
        endpoints: {
            chat: '/api/chat',
            files: '/api/files',
            settings: '/api/settings'
        },
        timeout: 30000 // 30 seconds
    },
    
    // UI Configuration
    ui: {
        maxMessageLength: 8000,
        maxMessagesDisplayed: 100,
        autoScrollDelay: 100,
        typingIndicatorDelay: 500,
        messageAnimationDuration: 300
    },
    
    // File Explorer Configuration
    fileExplorer: {
        maxDepth: 10,
        showHiddenFiles: false,
        supportedExtensions: [
            '.rs', '.js', '.ts', '.html', '.css', '.json', '.toml', '.md',
            '.py', '.java', '.cpp', '.c', '.h', '.go', '.php', '.rb',
            '.yml', '.yaml', '.xml', '.txt', '.log'
        ],
        iconMap: {
            '.rs': '🦀',
            '.js': '📜',
            '.ts': '📘',
            '.html': '🌐',
            '.css': '🎨',
            '.json': '📋',
            '.toml': '⚙️',
            '.md': '📝',
            '.py': '🐍',
            '.java': '☕',
            '.cpp': '⚡',
            '.c': '🔧',
            '.h': '📄',
            '.go': '🐹',
            '.php': '🐘',
            '.rb': '💎',
            '.yml': '📄',
            '.yaml': '📄',
            '.xml': '📄',
            '.txt': '📄',
            '.log': '📋',
            'folder': '📁',
            'default': '📄'
        }
    },
    
    // Theme Configuration
    theme: {
        default: 'light',
        available: ['light', 'dark'],
        preference: 'system' // 'light', 'dark', or 'system'
    },
    
    // Feature Flags
    features: {
        fileExplorer: true,
        codeHighlighting: true,
        messageFormatting: true,
        keyboardShortcuts: true,
        autoComplete: false, // Future feature
        voiceInput: false    // Future feature
    },
    
    // Keyboard Shortcuts
    shortcuts: {
        sendMessage: 'Ctrl+Enter',
        clearChat: 'Ctrl+Shift+K',
        focusInput: 'Ctrl+/',
        toggleSidebar: 'Ctrl+B',
        openSettings: 'Ctrl+,'
    },
    
    // Error Messages
    messages: {
        connectionError: 'Unable to connect to the server. Please check your connection.',
        processingError: 'An error occurred while processing your request.',
        fileLoadError: 'Unable to load the file tree.',
        messageTooLong: 'Message is too long. Please keep it under 8000 characters.',
        emptyMessage: 'Please enter a message before sending.'
    }
};

// Make config available globally
if (typeof window !== 'undefined') {
    window.CONFIG = CONFIG;
}

// Export for Node.js environments
if (typeof module !== 'undefined' && module.exports) {
    module.exports = CONFIG;
}