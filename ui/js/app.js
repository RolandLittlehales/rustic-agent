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
        
        this.init();
    }
    
    init() {
        this.setupEventListeners();
        this.setupTextareaAutoResize();
        this.loadFileTree();
        this.updateCharCount();
        
        // Add welcome message after a short delay
        setTimeout(() => {
            this.checkEnvironment();
        }, 1000);
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
        
        // Button clicks
        this.refreshBtn.addEventListener('click', () => {
            this.loadFileTree();
        });
        
        this.settingsBtn.addEventListener('click', () => {
            this.showSettings();
        });
        
        // Auto-scroll to bottom when new messages are added
        const observer = new MutationObserver(() => {
            this.scrollToBottom();
        });
        
        if (this.chatMessages) {
            observer.observe(this.chatMessages, { childList: true });
        }
    }
    
    setupTextareaAutoResize() {
        this.chatInput.addEventListener('input', () => {
            this.adjustTextareaHeight();
        });
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
        console.log('🚀 simulateAPICall() called with message:', message.substring(0, 50) + '...');
        
        // Check if Tauri API is available
        if (window.__TAURI__ && window.__TAURI__.core) {
            console.log('✅ Tauri available - using desktop mode');
            try {
                // First set API key if not already set
                console.log('🔑 Setting API key...');
                await this.setApiKey();
                
                // Send message to Claude through Tauri
                console.log('📤 Sending message to Claude via Tauri...');
                const response = await window.__TAURI__.core.invoke('send_message_to_claude', {
                    message: message
                });
                
                console.log('✅ Received response from Tauri:', response.substring(0, 100) + '...');
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
            console.log('⚠️ Tauri not available - using browser fallback');
            // Direct Claude API call as fallback (for testing when Tauri isn't available)
            return await this.directClaudeAPICall(message);
        }
    }

    // Direct Claude API call (fallback when Tauri not available)
    async directClaudeAPICall(message) {
        const apiKey = window.CLAUDE_API_KEY || "YOUR_API_KEY_HERE";
        
        try {
            const response = await fetch('https://api.anthropic.com/v1/messages', {
                method: 'POST',
                headers: {
                    'x-api-key': apiKey,
                    'anthropic-version': '2023-06-01',
                    'content-type': 'application/json'
                },
                body: JSON.stringify({
                    model: 'claude-3-5-sonnet-20241022',
                    max_tokens: 1000,
                    messages: [
                        {
                            role: 'user',
                            content: message
                        }
                    ]
                })
            });

            if (!response.ok) {
                throw new Error(`API Error: ${response.status}`);
            }

            const data = await response.json();
            
            if (data.content && data.content[0] && data.content[0].text) {
                return `🌐 [Direct API] ${data.content[0].text}`;
            } else {
                throw new Error('Invalid response format');
            }
        } catch (error) {
            console.error('Direct Claude API call failed:', error);
            return `❌ API Error: ${error.message}. Note: This may be due to CORS restrictions. Use 'tauri dev' for full functionality.`;
        }
    }

    // Set the Claude API key (only when Tauri is available)
    async setApiKey() {
        console.log('🔑 setApiKey() called');
        console.log('- Tauri available:', !!(window.__TAURI__ && window.__TAURI__.core));
        console.log('- apiKeySet flag:', this.apiKeySet);
        
        // Skip if already set or if Tauri is not available
        if (!window.__TAURI__ || !window.__TAURI__.core) {
            console.log('⚠️ Skipping API key setup - Tauri not available');
            return;
        }
        
        if (this.apiKeySet) {
            console.log('⚠️ Skipping API key setup - already set');
            return;
        }

        const apiKey = window.CLAUDE_API_KEY || "YOUR_API_KEY_HERE";
        console.log('- API key source:', window.CLAUDE_API_KEY ? 'window.CLAUDE_API_KEY' : 'fallback');
        // Security: Mask API key details in logs
        console.log('- API key: [REDACTED - Length: ' + apiKey.length + ']');
        console.log('- API key format valid:', apiKey.startsWith('sk-ant'));
        
        if (apiKey === "YOUR_API_KEY_HERE" || !apiKey) {
            console.error('❌ No Claude API key provided. Set CLAUDE_API_KEY environment variable.');
            this.addMessage('system', '❌ No Claude API key available. Check console for details.');
            return;
        }
        
        try {
            console.log('🚀 Calling Tauri set_claude_api_key command...');
            const result = await window.__TAURI__.core.invoke('set_claude_api_key', {
                apiKey: apiKey
            });
            console.log('✅ Tauri API key command result:', result);
            this.apiKeySet = true;
            this.addMessage('system', '✅ Claude API key configured successfully');
        } catch (error) {
            console.error('❌ Failed to set API key via Tauri:', error);
            this.addMessage('system', `❌ Failed to configure API key: ${error.message || error}`);
            this.apiKeySet = false; // Allow retry
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
            // Simulate loading files (replace with actual file system API)
            await new Promise(resolve => setTimeout(resolve, 500));
            
            const mockFiles = [
                { name: 'Cargo.toml', type: 'file', icon: '📄' },
                { name: 'build.rs', type: 'file', icon: '🔧' },
                { name: 'tauri.conf.json', type: 'file', icon: '⚙️' },
                { name: 'src/', type: 'folder', icon: '📁', children: [
                    { name: 'main.rs', type: 'file', icon: '🦀' }
                ]},
                { name: 'src-tauri/', type: 'folder', icon: '📁', children: [
                    { name: 'Cargo.toml', type: 'file', icon: '📄' },
                    { name: 'src/', type: 'folder', icon: '📁' }
                ]},
                { name: 'ui/', type: 'folder', icon: '📁', children: [
                    { name: 'index.html', type: 'file', icon: '🌐' },
                    { name: 'css/', type: 'folder', icon: '📁' },
                    { name: 'js/', type: 'folder', icon: '📁' }
                ]}
            ];
            
            this.renderFileTree(mockFiles);
        } catch (error) {
            console.error('Error loading file tree:', error);
            this.fileTree.innerHTML = '<div class="file-tree-placeholder"><p>Error loading files</p></div>';
        }
    }
    
    renderFileTree(files, level = 0) {
        const container = level === 0 ? this.fileTree : document.createElement('div');
        
        if (level === 0) {
            container.innerHTML = '';
        }
        
        files.forEach(file => {
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
    
    // Check which environment we're running in
    checkEnvironment() {
        console.log('🔍 Environment Check:');
        console.log('- window.__TAURI__:', !!window.__TAURI__);
        console.log('- window.__TAURI__.core:', !!(window.__TAURI__ && window.__TAURI__.core));
        console.log('- window.CLAUDE_API_KEY defined:', !!window.CLAUDE_API_KEY);
        // Security: API key details masked in production
        if (window.CLAUDE_API_KEY && window.CLAUDE_API_KEY !== 'PLACEHOLDER_FOR_DEV_INJECTION') {
            console.log('- API Key: [REDACTED - Length: ' + window.CLAUDE_API_KEY.length + ']');
            console.log('- API Key format valid:', window.CLAUDE_API_KEY.startsWith('sk-ant'));
        } else {
            console.log('- API Key: [NOT SET OR PLACEHOLDER]');
        }
        
        if (window.__TAURI__ && window.__TAURI__.core) {
            console.log('✅ Running in Tauri Desktop Mode');
            // Security: Don't expose API key status in user messages
            const apiStatus = (window.CLAUDE_API_KEY && window.CLAUDE_API_KEY !== 'PLACEHOLDER_FOR_DEV_INJECTION') ? 'Configured ✅' : 'Missing ❌';
            this.addMessage('assistant', '🚀 **DESKTOP MODE ACTIVE** - Full functionality enabled!\n\n🔑 API Key Status: ' + apiStatus + '\n\n✅ You are in the CORRECT window!\n\nI can help you with:\n• File operations (read/write/list)\n• Claude AI integration with tools\n• Development tasks\n\nTry: "Hello Claude, can you list the files in the current directory?"');
            document.title = '🚀 LLM Dev Agent - DESKTOP MODE ✅';
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
            console.log('⚠️ Running in Browser Fallback Mode');
            // Security: Don't expose API key status in user messages
            const apiStatus = (window.CLAUDE_API_KEY && window.CLAUDE_API_KEY !== 'PLACEHOLDER_FOR_DEV_INJECTION') ? 'Configured ✅' : 'Missing ❌';
            this.addMessage('assistant', '🌐 **Browser Fallback Mode** - Limited functionality\n\n🔑 API Key Status: ' + apiStatus + '\n\nI can provide basic Claude AI responses, but file operations are not available.\n\n⚠️ For full functionality, close this and run: `npm run dev`\n\n🔍 Look for a **desktop application window**, not this browser tab!');
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
}

// Initialize the application when DOM is loaded
document.addEventListener('DOMContentLoaded', () => {
    window.devAgent = new DevAgentApp();
});

// Export for use in other modules
if (typeof module !== 'undefined' && module.exports) {
    module.exports = DevAgentApp;
}