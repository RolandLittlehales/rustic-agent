/**
 * LLM Dev Agent - Main Application JavaScript
 * Handles chat interface, file explorer, and UI interactions
 */

class DevAgentApp {
    constructor() {
        this.chatMessages = document.getElementById('chatMessages');
        this.chatInput = document.getElementById('chatInput');
        this.chatForm = document.getElementById('chatForm');
        this.sendBtn = document.getElementById('sendBtn');
        this.loadingOverlay = document.getElementById('loadingOverlay');
        this.fileTree = document.getElementById('fileTree');
        this.refreshBtn = document.getElementById('refreshBtn');
        this.settingsBtn = document.getElementById('settingsBtn');

        this.isProcessing = false;
        this.messageHistory = [];
        
        // Store event listeners for cleanup
        this.eventListeners = new Map();
        this.mutationObserver = null;

        this.init();
    }

    init() {
        this.setupEventListeners();
        this.setupTextareaAutoResize();
        this.updateCharCount();

        // Wait for Tauri to be ready before loading file tree and checking environment
        this.waitForTauriAndInitialize();
    }

    async waitForTauriAndInitialize() {
        // Wait for Tauri to be available with timeout
        const maxWaitTime = 5000; // 5 seconds
        const checkInterval = 100; // Check every 100ms
        let elapsed = 0;
        
        while (elapsed < maxWaitTime) {
            if (window.__TAURI__ && window.__TAURI__.core) {
                console.log('🚀 Desktop Mode: Tauri initialized');
                
                // Now that Tauri is ready, load the file tree, check environment, and start file watching
                try {
                    await this.loadFileTree();
                } catch (error) {
                    console.error('❌ Failed to load file tree:', error);
                }
                
                this.checkEnvironment();
                
                // Start file watching after a short delay
                setTimeout(() => {
                    this.startFileWatching();
                }, 2000);
                
                return;
            }
            
            // Wait before next check
            await new Promise(resolve => setTimeout(resolve, checkInterval));
            elapsed += checkInterval;
        }
        
        // Tauri is not available after timeout - likely running in browser mode
        console.log('🌐 Browser Mode: Limited functionality');
        
        // Still try to load file tree (will show browser fallback message)
        this.loadFileTree();
        this.checkEnvironment();
    }

    setupEventListeners() {
        // Chat form submission
        this.chatForm.addEventListener('submit', (e) => {
            e.preventDefault();
            this.handleSendMessage();
        });

        // Textarea input events
        this.chatInput.addEventListener('input', () => {
            this.updateCharCount();
            this.adjustTextareaHeight();
        });

        // Keyboard shortcuts
        this.chatInput.addEventListener('keydown', (e) => {
            if (e.ctrlKey && e.key === 'Enter') {
                e.preventDefault();
                this.handleSendMessage();
            }
        });

        // Keep useful debugging shortcuts
        document.addEventListener('keydown', (e) => {
            if (e.ctrlKey && e.key === 't') {
                e.preventDefault();
                this.testWhitelist();
            }
            if (e.ctrlKey && e.key === 'r') {
                e.preventDefault();
                this.loadFileTree();
            }
            if (e.ctrlKey && e.key === 'd') {
                e.preventDefault();
                this.debugAppState();
            }
        });

        // Button clicks
        this.refreshBtn.addEventListener('click', () => {
            this.loadFileTree();
        });

        this.settingsBtn.addEventListener('click', () => {
            this.showSettings();
        });

        // Auto-scroll to bottom when new messages are added
        this.mutationObserver = new MutationObserver(() => {
            this.scrollToBottom();
        });

        if (this.chatMessages) {
            this.mutationObserver.observe(this.chatMessages, { childList: true });
        }
    }

    setupTextareaAutoResize() {
        // Initialize textarea height and auto-resize behavior
        if (this.chatInput) {
            this.adjustTextareaHeight();
            
            // Configure textarea for auto-resize
            this.chatInput.style.resize = 'none'; // Disable manual resize
            this.chatInput.style.overflow = 'hidden'; // Hide scrollbar during auto-resize
        }
    }

    adjustTextareaHeight() {
        this.chatInput.style.height = 'auto';
        this.chatInput.style.height = Math.min(this.chatInput.scrollHeight, 120) + 'px';
    }

    updateCharCount() {
        const charCount = document.querySelector('.char-count');
        if (charCount) {
            const length = this.chatInput.value.length;
            charCount.textContent = `${length}/8000`;

            // Update color based on length
            if (length > 7500) {
                charCount.style.color = 'var(--error-500)';
            } else if (length > 6000) {
                charCount.style.color = 'var(--warning-500)';
            } else {
                charCount.style.color = 'var(--gray-500)';
            }
        }
    }

    async handleSendMessage() {
        const message = this.chatInput.value.trim();

        if (!message || this.isProcessing) {
            return;
        }

        // Add user message to chat
        this.addMessage('user', message);

        // Clear input
        this.chatInput.value = '';
        this.updateCharCount();
        this.adjustTextareaHeight();

        // Set processing state
        this.setProcessingState(true);

        try {
            // Simulate API call (replace with actual implementation)
            const response = await this.simulateAPICall(message);
            this.addMessage('assistant', response);
        } catch (error) {
            console.error('Error processing message:', error);
            this.addMessage('system', 'Sorry, there was an error processing your request. Please try again.');
        } finally {
            this.setProcessingState(false);
        }
    }

    async simulateAPICall(message) {

        // Check if Tauri API is available
        if (window.__TAURI__ && window.__TAURI__.core) {
            try {
                // First set API key if not already set
                await this.initializeApiKey();

                // Send message to Claude through Tauri
                const response = await window.__TAURI__.core.invoke('send_message_to_claude', {
                    message: message
                });

                return response;
            } catch (error) {
                console.error('❌ Tauri API call failed:', error);
                console.error('Error details:', {
                    name: error.name,
                    message: error.message,
                    stack: error.stack
                });
                return `❌ Tauri Error: ${error.message || error}`;
            }
        } else {
            // Direct Claude API call as fallback (for testing when Tauri isn't available)
            return await this.directClaudeAPICall(message);
        }
    }

    // Direct Claude API call (fallback when Tauri not available)
    async directClaudeAPICall(_message) {
        // Note: _message parameter intentionally unused for security
        // API keys are not available in browser mode for security reasons
        return "❌ API Error: Direct API calls not supported in browser mode. API keys are only available through the Tauri desktop application for security reasons. Please use 'npm run dev' to start the desktop application.";
    }

    // Initialize API key from environment (only when Tauri is available)
    async initializeApiKey() {

        // Skip if already set or if Tauri is not available
        if (!window.__TAURI__ || !window.__TAURI__.core) {
            return;
        }

        if (this.apiKeySet) {
            return;
        }

        try {
            // First check if API key is already available from environment
            const hasKey = await window.__TAURI__.core.invoke('get_api_key_status');
            
            if (hasKey) {
                this.apiKeySet = true;
                this.addMessage('system', '✅ Claude API key loaded from environment');
                return;
            }
            
            // Try to initialize from environment variable
            const result = await window.__TAURI__.core.invoke('initialize_with_env_key');
            this.apiKeySet = true;
            this.addMessage('system', '✅ Claude API key configured from environment');
        } catch (error) {
            console.error('❌ No API key found in environment:', error);
            this.addMessage('system', '❌ No Claude API key found. Please set CLAUDE_API_KEY environment variable.');
            this.apiKeySet = false;
        }
    }

    addMessage(type, content, timestamp = null) {
        const messageElement = document.createElement('div');
        messageElement.className = `message ${type}-message new`;

        const contentElement = document.createElement('div');
        contentElement.className = 'message-content';

        // Handle markdown-like formatting
        const formattedContent = this.formatMessageContent(content);
        contentElement.innerHTML = formattedContent;

        const timestampElement = document.createElement('div');
        timestampElement.className = 'message-timestamp';
        timestampElement.textContent = timestamp || this.formatTimestamp(new Date());

        messageElement.appendChild(contentElement);
        messageElement.appendChild(timestampElement);

        this.chatMessages.appendChild(messageElement);

        // Store in history
        this.messageHistory.push({
            type,
            content,
            timestamp: timestamp || new Date().toISOString()
        });

        // Remove 'new' class after animation
        setTimeout(() => {
            messageElement.classList.remove('new');
        }, 300);

        this.scrollToBottom();
    }

    formatMessageContent(content) {
        // First, escape HTML to prevent XSS
        const escapeHtml = (text) => {
            const div = document.createElement('div');
            div.textContent = text;
            return div.innerHTML;
        };

        // Escape the content first
        let escaped = escapeHtml(content);

        // Then apply safe markdown-like formatting
        let formatted = escaped
            .replace(/`([^`<>]+)`/g, '<code>$1</code>')
            .replace(/\*\*([^*<>]+)\*\*/g, '<strong>$1</strong>')
            .replace(/\*([^*<>]+)\*/g, '<em>$1</em>')
            .replace(/\n/g, '<br>');

        // Handle code blocks with additional sanitization
        formatted = formatted.replace(/```([^`<>]+)```/g, '<pre><code>$1</code></pre>');

        return formatted;
    }

    formatTimestamp(date) {
        const now = new Date();
        const diff = now - date;

        if (diff < 60000) { // Less than 1 minute
            return 'Just now';
        } else if (diff < 3600000) { // Less than 1 hour
            const minutes = Math.floor(diff / 60000);
            return `${minutes} minute${minutes > 1 ? 's' : ''} ago`;
        } else if (diff < 86400000) { // Less than 24 hours
            const hours = Math.floor(diff / 3600000);
            return `${hours} hour${hours > 1 ? 's' : ''} ago`;
        } else {
            return date.toLocaleDateString();
        }
    }

    setProcessingState(processing) {
        this.isProcessing = processing;
        this.sendBtn.disabled = processing;
        this.chatInput.disabled = processing;

        if (processing) {
            this.loadingOverlay.classList.remove('hidden');
            this.sendBtn.innerHTML = `
                <div class="spinner" style="width: 16px; height: 16px; border-width: 2px;"></div>
                Thinking...
            `;
        } else {
            this.loadingOverlay.classList.add('hidden');
            this.sendBtn.innerHTML = `
                <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <line x1="22" y1="2" x2="11" y2="13"></line>
                    <polygon points="22,2 15,22 11,13 2,9"></polygon>
                </svg>
                Send
            `;
        }
    }

    scrollToBottom() {
        this.chatMessages.scrollTop = this.chatMessages.scrollHeight;
    }

    async loadFileTree() {
        this.fileTree.innerHTML = '<div class="file-tree-placeholder"><p>Loading project files...</p></div>';

        try {
            if (window.__TAURI__ && window.__TAURI__.core) {
                // Use Tauri to get real file listing with timeout
                const timeoutPromise = new Promise((_, reject) => 
                    setTimeout(() => reject(new Error('File loading timeout')), 10000)
                );
                
                const listDirectoryPromise = window.__TAURI__.core.invoke('list_directory', { path: '.' });
                
                const files = await Promise.race([listDirectoryPromise, timeoutPromise]);
                
                if (!Array.isArray(files)) {
                    throw new Error(`Expected array but got ${typeof files}`);
                }
                
                // Convert to the format expected by renderFileTree
                const treeFiles = files.map((file) => {
                    return {
                        name: file.name,
                        type: file.file_type === 'directory' ? 'folder' : 'file',
                        icon: file.icon,
                        path: file.path
                    };
                });
                
                this.renderFileTree(treeFiles);
            } else {
                // Browser fallback - show message about desktop mode
                this.fileTree.innerHTML = '<div class="file-tree-placeholder"><p>📁 File explorer requires desktop mode<br>Use <code>npm run dev</code> to access files</p></div>';
            }
        } catch (error) {
            console.error('❌ Error loading files:', error);
            this.fileTree.innerHTML = `<div class="file-tree-placeholder"><p>Error loading files:<br>${error.message || error}</p></div>`;
        }
    }

    renderFileTree(files, level = 0) {
        const container = level === 0 ? this.fileTree : document.createElement('div');

        if (level === 0) {
            container.innerHTML = '';
        }

        files.forEach((file) => {
            const item = document.createElement('div');
            item.className = 'file-item';
            item.style.paddingLeft = `${level * 20 + 12}px`;

            item.innerHTML = `
                <span class="file-icon">${file.icon}</span>
                <span class="file-name">${file.name}</span>
            `;

            item.addEventListener('click', () => {
                this.handleFileClick(file, item);
            });

            container.appendChild(item);

            if (file.children && file.type === 'folder') {
                const childContainer = document.createElement('div');
                childContainer.className = 'file-children';
                this.renderFileTree(file.children, level + 1);
                // For now, show all children (in a real implementation, this would be collapsible)
                file.children.forEach(child => {
                    const childItem = document.createElement('div');
                    childItem.className = 'file-item';
                    childItem.style.paddingLeft = `${(level + 1) * 20 + 12}px`;
                    childItem.innerHTML = `
                        <span class="file-icon">${child.icon}</span>
                        <span class="file-name">${child.name}</span>
                    `;
                    childItem.addEventListener('click', () => {
                        this.handleFileClick(child, childItem);
                    });
                    container.appendChild(childItem);
                });
            }
        });

        return container;
    }

    handleFileClick(file, element) {
        // Remove previous selection
        document.querySelectorAll('.file-item.selected').forEach(item => {
            item.classList.remove('selected');
        });

        // Add selection to clicked item
        element.classList.add('selected');

        if (file.type === 'file') {
            this.addMessage('system', `Selected file: ${file.name}`);
        }
    }

    showSettings() {
        // Placeholder for settings modal
        alert('Settings panel would open here. This is a placeholder implementation.');
    }

    // Utility methods
    escapeHtml(text) {
        const div = document.createElement('div');
        div.textContent = text;
        return div.innerHTML;
    }

    // Whitelist management functions for testing
    async testWhitelist() {
        if (!window.__TAURI__ || !window.__TAURI__.core) {
            return;
        }

        try {
            // Get current config
            const config = await window.__TAURI__.core.invoke('whitelist_get_config');
            
            // List current directories
            const directories = await window.__TAURI__.core.invoke('whitelist_list_directories');
            
            // Show in UI
            this.addMessage('system', `📋 Whitelist Status:\n• Enabled: ${config.enabled}\n• Directories: ${directories.length}\n• Current dirs: ${directories.join(', ')}`);
            
        } catch (error) {
            console.error('❌ Whitelist test failed:', error);
            this.addMessage('system', `❌ Whitelist test failed: ${error}`);
        }
    }

    async addDirectoryToWhitelist(path) {
        if (!window.__TAURI__ || !window.__TAURI__.core) {
            return;
        }

        try {
            const result = await window.__TAURI__.core.invoke('whitelist_add_directory', path);
            this.addMessage('system', `✅ Added to whitelist: ${path}`);
            
            // Refresh file tree
            this.loadFileTree();
        } catch (error) {
            console.error('❌ Failed to add directory:', error);
            this.addMessage('system', `❌ Failed to add directory: ${error}`);
        }
    }

    // File watching functionality
    async startFileWatching() {
        if (!window.__TAURI__ || !window.__TAURI__.core) {
            return;
        }

        try {
            
            // Set up event listeners for file changes
            await this.setupFileWatchingListeners();
            
            // Start the file watcher
            const result = await window.__TAURI__.core.invoke('start_file_watching');
            this.addMessage('system', '👁️ File watching enabled - changes will auto-refresh');
        } catch (error) {
            console.error('❌ Failed to start file watching:', error);
            this.addMessage('system', `❌ File watching failed: ${error}`);
        }
    }

    async setupFileWatchingListeners() {
        if (!window.__TAURI__ || !window.__TAURI__.event) {
            return;
        }

        // Listen for file change events
        await window.__TAURI__.event.listen('file_changed', (event) => {
            // Debounce refresh to avoid too many updates
            this.debounceFileTreeRefresh();
        });

        // Listen for heartbeat refresh events
        await window.__TAURI__.event.listen('heartbeat_refresh', () => {
            this.debounceFileTreeRefresh();
        });
    }

    debounceFileTreeRefresh() {
        // Clear existing timeout
        if (this.fileTreeRefreshTimeout) {
            clearTimeout(this.fileTreeRefreshTimeout);
        }

        // Set new timeout
        this.fileTreeRefreshTimeout = setTimeout(() => {
            this.loadFileTree();
        }, 500); // 500ms debounce
    }

    async stopFileWatching() {
        if (!window.__TAURI__ || !window.__TAURI__.core) {
            return;
        }

        try {
            const result = await window.__TAURI__.core.invoke('stop_file_watching');
            this.addMessage('system', '🛑 File watching disabled');
        } catch (error) {
            console.error('❌ Failed to stop file watching:', error);
        }
    }

    // Debug function to check app state
    debugAppState() {
        console.log('🔧 APP STATE DEBUG');
        console.log('- Tauri available:', !!(window.__TAURI__ && window.__TAURI__.core));
        console.log('- File tree element:', !!this.fileTree);
        console.log('- File tree children:', this.fileTree?.children.length || 0);
    }

    // Check which environment we're running in
    checkEnvironment() {

        if (window.__TAURI__ && window.__TAURI__.core) {
            console.log('✅ Desktop Mode Active');
            // API key will be loaded from environment
            this.addMessage('assistant', '🚀 **DESKTOP MODE ACTIVE** - Full functionality enabled!\n\n🔑 API Key: Loading from environment...\n📁 File System: Checking whitelist configuration...\n\n✅ You are in the CORRECT window!\n\nI can help you with:\n• File operations (read/write/list)\n• Claude AI integration with tools\n• Development tasks\n\nTry: "Hello Claude, can you list the files in the current directory?"');
            document.title = '🚀 LLM Dev Agent - DESKTOP MODE ✅';
            
            // Test whitelist functionality
            setTimeout(() => {
                this.testWhitelist();
            }, 2000);
            // Add visual indicator
            const header = document.querySelector('.app-header');
            if (header) {
                header.style.borderLeft = '4px solid #10B981';
                header.style.backgroundColor = '#D1FAE5';
            }
            // Add desktop mode badge (positioned to avoid settings button)
            const badge = document.createElement('div');
            badge.innerHTML = '🚀 DESKTOP MODE';
            badge.style.cssText = 'position: fixed; top: 10px; left: 10px; background: #10B981; color: white; padding: 6px 12px; border-radius: 16px; font-weight: bold; font-size: 12px; z-index: 1000; box-shadow: 0 2px 8px rgba(0,0,0,0.1); opacity: 0.9;';
            document.body.appendChild(badge);
        } else {
            console.log('⚠️ Browser Mode - Limited functionality');
            // API key not available in browser mode
            this.addMessage('assistant', '🌐 **Browser Fallback Mode** - Limited functionality\n\n🔑 API Key: Not available in browser mode\n\nI can provide basic Claude AI responses, but file operations are not available.\n\n⚠️ For full functionality, close this and run: `npm run dev`\n\n🔍 Look for a **desktop application window**, not this browser tab!');
            document.title = 'LLM Dev Agent - Browser Mode ⚠️';
            // Add visual indicator
            const header = document.querySelector('.app-header');
            if (header) {
                header.style.borderLeft = '4px solid #F59E0B';
                header.style.backgroundColor = '#FEF3C7';
            }
        }
    }

    // Public API methods for external integration
    sendMessage(message) {
        this.chatInput.value = message;
        this.handleSendMessage();
    }

    clearChat() {
        this.chatMessages.innerHTML = '';
        this.messageHistory = [];
        this.addMessage('system', 'Chat cleared.');
    }

    getMessageHistory() {
        return [...this.messageHistory];
    }

    /**
     * Cleanup method to prevent resource leaks
     * Should be called when the app is being destroyed or page is unloaded
     */
    cleanup() {
        // Stop file watching
        if (window.__TAURI__ && window.__TAURI__.core) {
            this.stopFileWatching().catch(err => 
                console.warn('Failed to stop file watching during cleanup:', err)
            );
        }

        // Clear debounce timeout
        if (this.fileTreeRefreshTimeout) {
            clearTimeout(this.fileTreeRefreshTimeout);
            this.fileTreeRefreshTimeout = null;
        }

        // Disconnect mutation observer
        if (this.mutationObserver) {
            this.mutationObserver.disconnect();
            this.mutationObserver = null;
        }

        // Clear stored event listeners if we had tracked them
        if (this.eventListeners) {
            this.eventListeners.clear();
        }

        // Clear message history to free memory
        this.messageHistory = [];

        // Clear chat messages from DOM
        if (this.chatMessages) {
            this.chatMessages.innerHTML = '';
        }

        console.log('🧹 DevAgentApp cleanup completed');
    }
}

// Initialize the application when DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
    window.devAgent = new DevAgentApp();
    
    // Initialize UX enhancements if available
    if (typeof UXEnhancements !== 'undefined') {
        window.uxEnhancements = new UXEnhancements(window.devAgent);
    }
});

// Add cleanup on page unload to prevent resource leaks
window.addEventListener('beforeunload', () => {
    if (window.devAgent && typeof window.devAgent.cleanup === 'function') {
        window.devAgent.cleanup();
    }
});

// Export for use in other modules
if (typeof module !== 'undefined' && module.exports) {
    module.exports = DevAgentApp;
}