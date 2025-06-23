/**
 * Frontend Configuration Constants
 * 
 * Centralized configuration for the UI layer.
 * Uses the same patterns as the backend for consistency.
 */

// ============================================================================
// APPLICATION CONFIGURATION
// ============================================================================

export const APP_CONFIG = {
    // Application metadata
    name: 'LLM Dev Agent',
    version: '0.1.0',
    
    // API Configuration (for direct calls when Tauri unavailable)
    api: {
        baseUrl: 'http://localhost:3000',
        timeout: 30000, // 30 seconds
        endpoints: {
            chat: '/api/chat',
            files: '/api/files',
            settings: '/api/settings'
        }
    },
    
    // UI Timing Configuration
    timing: {
        autoScrollDelay: 100,
        typingIndicatorDelay: 500,
        messageAnimationDuration: 300,
        fileTreeRefreshDebounce: 500,
        tauriInitTimeout: 5000,
        tauriCheckInterval: 100,
        fileWatchingStartDelay: 2000
    }
};

// ============================================================================
// VALIDATION LIMITS (must match backend constants)
// ============================================================================

export const VALIDATION = {
    // Message limits
    messageMaxChars: 50000,  // 50KB for coding helper
    nameMaxChars: 100,
    pathMaxChars: 4096,
    
    // Character count warning thresholds
    messageWarningThreshold: 40000,  // 80% of max
    messageDangerThreshold: 47500,   // 95% of max
    
    // UI display limits
    maxMessagesDisplayed: 100,
    maxDirectoryEntries: 1000
};

// ============================================================================
// FILE SYSTEM CONFIGURATION
// ============================================================================

export const FILE_SYSTEM = {
    // Allowed file extensions
    allowedExtensions: [
        'rs', 'js', 'ts', 'jsx', 'tsx',
        'json', 'toml', 'yaml', 'yml',
        'md', 'txt', 'html', 'css',
        'py', 'go', 'java', 'cpp', 'c', 'h', 'hpp',
        'sh', 'bat', 'ps1'
    ],
    
    // File type icons mapping
    iconMap: {
        // Directories
        'directory': '📁',
        
        // Programming languages
        'rs': '🦀',
        'js': '📄',
        'ts': '📘',
        'jsx': '⚛️',
        'tsx': '⚛️',
        'py': '🐍',
        'java': '☕',
        'cpp': '⚡',
        'c': '⚡',
        'h': '⚡',
        'hpp': '⚡',
        'go': '🐹',
        
        // Configuration files
        'json': '⚙️',
        'toml': '⚙️',
        'yaml': '⚙️',
        'yml': '⚙️',
        
        // Documentation
        'md': '📝',
        'txt': '📄',
        
        // Web files
        'html': '🌐',
        'css': '🎨',
        
        // Scripts
        'sh': '📜',
        'bat': '📜',
        'ps1': '📜',
        
        // Default
        'default': '📄'
    },
    
    // File size display formatting
    sizeUnits: ['B', 'KB', 'MB', 'GB']
};

// ============================================================================
// UI APPEARANCE CONFIGURATION
// ============================================================================

export const UI = {
    // Layout dimensions
    window: {
        defaultWidth: 1200,
        defaultHeight: 800,
        minWidth: 800,
        minHeight: 600
    },
    
    // Chat interface
    chat: {
        maxTextareaHeight: 120, // pixels
        charCountWarningColor: '#F59E0B',
        charCountDangerColor: '#EF4444',
        charCountNormalColor: '#6B7280'
    },
    
    // File tree
    fileTree: {
        indentPerLevel: 20, // pixels
        basePadding: 12     // pixels
    },
    
    // Message types
    messageTypes: {
        user: 'user-message',
        assistant: 'assistant-message',
        system: 'system-message'
    }
};

// ============================================================================
// ENVIRONMENT DETECTION
// ============================================================================

export const ENVIRONMENT = {
    // Runtime environment detection
    isTauriAvailable: () => !!(window.__TAURI__ && window.__TAURI__.core),
    isDesktopMode: () => ENVIRONMENT.isTauriAvailable(),
    isBrowserMode: () => !ENVIRONMENT.isTauriAvailable(),
    
    // Development mode detection
    isDevelopment: () => {
        return window.location.hostname === 'localhost' || 
               window.location.hostname === '127.0.0.1' ||
               window.location.hostname === '';
    }
};

// ============================================================================
// KEYBOARD SHORTCUTS
// ============================================================================

export const SHORTCUTS = {
    // Chat shortcuts
    sendMessage: { ctrl: true, key: 'Enter' },
    
    // File operations
    refreshFileTree: { ctrl: true, key: 'r' },
    testWhitelist: { ctrl: true, key: 't' },
    debugAppState: { ctrl: true, key: 'd' }
};

// ============================================================================
// ERROR MESSAGES
// ============================================================================

export const ERROR_MESSAGES = {
    api: {
        noTauri: '❌ API Error: Direct API calls not supported in browser mode. API keys are only available through the Tauri desktop application for security reasons. Please use \'npm run dev\' to start the desktop application.',
        timeout: '❌ Request timeout. Please try again.',
        network: '❌ Network error. Please check your connection.',
        generic: '❌ An error occurred. Please try again.'
    },
    
    validation: {
        messageEmpty: 'Message cannot be empty',
        messageTooLong: `Message too long (max ${VALIDATION.messageMaxChars.toLocaleString()} characters)`,
        fileNotFound: 'File not found',
        accessDenied: 'Access denied'
    },
    
    fileSystem: {
        loadFailed: 'Error loading files',
        accessDenied: 'Access denied to file or directory',
        notFound: 'File or directory not found'
    }
};

// ============================================================================
// SUCCESS MESSAGES
// ============================================================================

export const SUCCESS_MESSAGES = {
    api: {
        keyLoaded: '✅ Claude API key loaded from environment',
        keyConfigured: '✅ Claude API key configured from environment'
    },
    
    fileSystem: {
        watchingEnabled: '👁️ File watching enabled - changes will auto-refresh',
        watchingDisabled: '🛑 File watching disabled',
        directoryAdded: '✅ Added to whitelist'
    }
};

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

export const CONFIG_HELPERS = {
    // Get timeout duration for operation type
    getTimeout: (operationType) => {
        const timeouts = {
            api: APP_CONFIG.api.timeout,
            tauri: APP_CONFIG.timing.tauriInitTimeout,
            fileLoad: 10000 // 10 seconds for file operations
        };
        return timeouts[operationType] || APP_CONFIG.api.timeout;
    },
    
    // Format file size
    formatFileSize: (bytes) => {
        if (bytes === 0) return '0 B';
        
        const k = 1024;
        const sizes = FILE_SYSTEM.sizeUnits;
        const i = Math.floor(Math.log(bytes) / Math.log(k));
        
        return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
    },
    
    // Get file icon
    getFileIcon: (fileName, fileType) => {
        if (fileType === 'directory') {
            return FILE_SYSTEM.iconMap.directory;
        }
        
        const extension = fileName.split('.').pop()?.toLowerCase() || '';
        return FILE_SYSTEM.iconMap[extension] || FILE_SYSTEM.iconMap.default;
    },
    
    // Get message warning level
    getMessageWarningLevel: (length) => {
        if (length >= VALIDATION.messageDangerThreshold) {
            return 'danger';
        } else if (length >= VALIDATION.messageWarningThreshold) {
            return 'warning';
        } else {
            return 'ok';
        }
    },
    
    // Get character count color
    getCharCountColor: (length) => {
        const level = CONFIG_HELPERS.getMessageWarningLevel(length);
        switch (level) {
            case 'danger': return UI.chat.charCountDangerColor;
            case 'warning': return UI.chat.charCountWarningColor;
            default: return UI.chat.charCountNormalColor;
        }
    }
};

// ============================================================================
// FEATURE FLAGS
// ============================================================================

export const FEATURES = {
    // Development features
    enableDebugMode: ENVIRONMENT.isDevelopment(),
    enableConsoleLogging: ENVIRONMENT.isDevelopment(),
    
    // UI features
    enableAnimations: true,
    enableFileWatching: true,
    enableKeyboardShortcuts: true,
    
    // API features
    enableDirectApiCalls: false, // Security: disabled by default
    enableApiKeyInBrowser: false // Security: never enable this
};

// Make configuration read-only in production
if (!ENVIRONMENT.isDevelopment()) {
    Object.freeze(APP_CONFIG);
    Object.freeze(VALIDATION);
    Object.freeze(FILE_SYSTEM);
    Object.freeze(UI);
    Object.freeze(SHORTCUTS);
    Object.freeze(ERROR_MESSAGES);
    Object.freeze(SUCCESS_MESSAGES);
    Object.freeze(FEATURES);
}