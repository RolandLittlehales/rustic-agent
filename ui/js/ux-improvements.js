/**
 * UX Improvements for LLM Dev Agent
 * Enhanced user experience features and interactions
 */

class UXEnhancements {
    constructor(app) {
        this.app = app;
        this.init();
    }

    init() {
        this.setupAdvancedKeyboardShortcuts();
        this.setupTooltips();
        this.setupProgressIndicators();
        this.setupMessageActions();
        this.setupDragAndDrop();
        this.setupContextMenus();
        this.setupAutoSave();
        this.setupThemeToggle();
        this.setupSearch();
        this.setupNotifications();
    }

    // Enhanced keyboard shortcuts
    setupAdvancedKeyboardShortcuts() {
        const shortcuts = {
            'Ctrl+/': () => this.focusInput(),
            'Ctrl+Shift+K': () => this.clearChat(),
            'Ctrl+B': () => this.toggleSidebar(),
            'Ctrl+,': () => this.openSettings(),
            'Ctrl+F': () => this.openSearch(),
            'Ctrl+N': () => this.newChat(),
            'Escape': () => this.handleEscape(),
            'Ctrl+Enter': () => this.sendMessage(),
            'Ctrl+Shift+C': () => this.copyLastResponse(),
            'Ctrl+Z': () => this.undoLastMessage(),
            'F1': () => this.showHelp()
        };

        document.addEventListener('keydown', (e) => {
            const key = this.getShortcutKey(e);
            if (shortcuts[key]) {
                e.preventDefault();
                shortcuts[key]();
            }
        });

        // Add keyboard shortcut hints
        this.addShortcutHints();
    }

    getShortcutKey(e) {
        const parts = [];
        if (e.ctrlKey) parts.push('Ctrl');
        if (e.shiftKey) parts.push('Shift');
        if (e.altKey) parts.push('Alt');
        if (e.metaKey) parts.push('Meta');
        parts.push(e.key);
        return parts.join('+');
    }

    // Tooltip system
    setupTooltips() {
        this.tooltip = document.createElement('div');
        this.tooltip.className = 'tooltip';
        this.tooltip.style.cssText = `
            position: absolute;
            background: var(--gray-900);
            color: white;
            padding: var(--space-2) var(--space-3);
            border-radius: var(--radius-md);
            font-size: var(--font-size-xs);
            z-index: 1000;
            pointer-events: none;
            opacity: 0;
            transform: translateY(4px);
            transition: all 200ms;
            max-width: 200px;
            word-wrap: break-word;
        `;
        document.body.appendChild(this.tooltip);

        // Add tooltips to elements with title or data-tooltip
        document.addEventListener('mouseenter', (e) => {
            const target = e.target.closest('[title], [data-tooltip]');
            if (target) {
                this.showTooltip(target, target.title || target.dataset.tooltip);
            }
        }, true);

        document.addEventListener('mouseleave', (e) => {
            const target = e.target.closest('[title], [data-tooltip]');
            if (target) {
                this.hideTooltip();
            }
        }, true);
    }

    showTooltip(element, text) {
        if (!text) return;
        
        this.tooltip.textContent = text;
        const rect = element.getBoundingClientRect();
        const tooltipRect = this.tooltip.getBoundingClientRect();
        
        let top = rect.bottom + 8;
        let left = rect.left + (rect.width / 2) - (tooltipRect.width / 2);
        
        // Keep tooltip in viewport
        if (left < 8) left = 8;
        if (left + tooltipRect.width > window.innerWidth - 8) {
            left = window.innerWidth - tooltipRect.width - 8;
        }
        if (top + tooltipRect.height > window.innerHeight - 8) {
            top = rect.top - tooltipRect.height - 8;
        }
        
        this.tooltip.style.left = left + 'px';
        this.tooltip.style.top = top + 'px';
        this.tooltip.style.opacity = '1';
        this.tooltip.style.transform = 'translateY(0)';
    }

    hideTooltip() {
        this.tooltip.style.opacity = '0';
        this.tooltip.style.transform = 'translateY(4px)';
    }

    // Progress indicators for long operations
    setupProgressIndicators() {
        this.progressBar = document.createElement('div');
        this.progressBar.className = 'progress-bar';
        this.progressBar.style.cssText = `
            position: fixed;
            top: 0;
            left: 0;
            width: 0%;
            height: 3px;
            background: linear-gradient(90deg, var(--primary-500), var(--success-500));
            z-index: 1001;
            transition: width 300ms ease;
            opacity: 0;
        `;
        document.body.appendChild(this.progressBar);
    }

    showProgress(percentage = 0) {
        this.progressBar.style.opacity = '1';
        this.progressBar.style.width = percentage + '%';
    }

    hideProgress() {
        this.progressBar.style.width = '100%';
        setTimeout(() => {
            this.progressBar.style.opacity = '0';
            this.progressBar.style.width = '0%';
        }, 300);
    }

    // Message actions (copy, edit, delete)
    setupMessageActions() {
        document.addEventListener('mouseenter', (e) => {
            const message = e.target.closest('.message:not(.system-message)');
            if (message && !message.querySelector('.message-actions')) {
                this.addMessageActions(message);
            }
        });
    }

    addMessageActions(message) {
        const actions = document.createElement('div');
        actions.className = 'message-actions';
        actions.style.cssText = `
            position: absolute;
            top: var(--space-2);
            right: var(--space-2);
            display: flex;
            gap: var(--space-1);
            opacity: 0;
            transition: opacity 200ms;
            background: rgba(255, 255, 255, 0.9);
            border-radius: var(--radius-md);
            padding: var(--space-1);
            backdrop-filter: blur(4px);
        `;

        const copyBtn = this.createActionButton('📋', 'Copy message', () => {
            this.copyMessageContent(message);
        });

        const editBtn = this.createActionButton('✏️', 'Edit message', () => {
            this.editMessage(message);
        });

        const deleteBtn = this.createActionButton('🗑️', 'Delete message', () => {
            this.deleteMessage(message);
        });

        actions.appendChild(copyBtn);
        if (message.classList.contains('user-message')) {
            actions.appendChild(editBtn);
        }
        actions.appendChild(deleteBtn);

        message.style.position = 'relative';
        message.appendChild(actions);

        // Show actions on hover
        message.addEventListener('mouseenter', () => {
            actions.style.opacity = '1';
        });

        message.addEventListener('mouseleave', () => {
            actions.style.opacity = '0';
        });
    }

    createActionButton(icon, tooltip, onClick) {
        const btn = document.createElement('button');
        btn.textContent = icon;
        btn.title = tooltip;
        btn.className = 'message-action-btn';
        btn.style.cssText = `
            background: none;
            border: none;
            cursor: pointer;
            padding: var(--space-1);
            border-radius: var(--radius-sm);
            font-size: 12px;
            line-height: 1;
            transition: background-color 150ms;
        `;
        btn.addEventListener('click', onClick);
        btn.addEventListener('mouseenter', () => {
            btn.style.backgroundColor = 'rgba(0, 0, 0, 0.1)';
        });
        btn.addEventListener('mouseleave', () => {
            btn.style.backgroundColor = 'transparent';
        });
        return btn;
    }

    // Drag and drop for files
    setupDragAndDrop() {
        const dropZone = document.querySelector('.chat-messages');
        
        ['dragenter', 'dragover', 'dragleave', 'drop'].forEach(eventName => {
            dropZone.addEventListener(eventName, this.preventDefaults, false);
        });

        ['dragenter', 'dragover'].forEach(eventName => {
            dropZone.addEventListener(eventName, () => this.highlight(dropZone), false);
        });

        ['dragleave', 'drop'].forEach(eventName => {
            dropZone.addEventListener(eventName, () => this.unhighlight(dropZone), false);
        });

        dropZone.addEventListener('drop', (e) => this.handleDrop(e), false);
    }

    preventDefaults(e) {
        e.preventDefault();
        e.stopPropagation();
    }

    highlight(element) {
        element.style.background = 'var(--primary-25)';
        element.style.border = '2px dashed var(--primary-300)';
    }

    unhighlight(element) {
        element.style.background = '';
        element.style.border = '';
    }

    handleDrop(e) {
        const files = Array.from(e.dataTransfer.files);
        if (files.length > 0) {
            this.processDroppedFiles(files);
        }
    }

    processDroppedFiles(files) {
        files.forEach(file => {
            if (file.type.startsWith('text/') || file.name.match(/\.(js|ts|html|css|json|md|txt|rs|py|java|cpp|c|h|go|php|rb|yml|yaml|xml|log)$/i)) {
                this.readFileContent(file);
            } else {
                this.showNotification(`File type not supported: ${file.name}`, 'warning');
            }
        });
    }

    readFileContent(file) {
        const reader = new FileReader();
        reader.onload = (e) => {
            const content = e.target.result;
            const message = `Here's the content of ${file.name}:\n\n\`\`\`\n${content}\n\`\`\``;
            this.app.addMessage('user', `Uploaded file: ${file.name}`);
            this.app.addMessage('system', message);
        };
        reader.readAsText(file);
    }

    // Context menus
    setupContextMenus() {
        document.addEventListener('contextmenu', (e) => {
            const message = e.target.closest('.message');
            if (message) {
                e.preventDefault();
                this.showContextMenu(e, message);
            }
        });

        document.addEventListener('click', () => {
            this.hideContextMenu();
        });
    }

    showContextMenu(e, message) {
        this.hideContextMenu();
        
        const menu = document.createElement('div');
        menu.className = 'context-menu';
        menu.style.cssText = `
            position: fixed;
            background: white;
            border: 1px solid var(--gray-200);
            border-radius: var(--radius-lg);
            box-shadow: var(--shadow-lg);
            z-index: 1002;
            min-width: 160px;
            padding: var(--space-2);
        `;

        const actions = [
            { label: 'Copy', icon: '📋', action: () => this.copyMessageContent(message) },
            { label: 'Quote Reply', icon: '💬', action: () => this.quoteMessage(message) },
            { label: 'Save to File', icon: '💾', action: () => this.saveMessage(message) },
        ];

        if (message.classList.contains('user-message')) {
            actions.splice(1, 0, { label: 'Edit', icon: '✏️', action: () => this.editMessage(message) });
        }

        actions.forEach(({ label, icon, action }) => {
            const item = document.createElement('button');
            item.textContent = `${icon} ${label}`;
            item.className = 'context-menu-item';
            item.style.cssText = `
                display: block;
                width: 100%;
                background: none;
                border: none;
                padding: var(--space-2) var(--space-3);
                text-align: left;
                cursor: pointer;
                border-radius: var(--radius-md);
                font-size: var(--font-size-sm);
                transition: background-color 150ms;
            `;
            item.addEventListener('click', () => {
                action();
                this.hideContextMenu();
            });
            item.addEventListener('mouseenter', () => {
                item.style.backgroundColor = 'var(--gray-100)';
            });
            item.addEventListener('mouseleave', () => {
                item.style.backgroundColor = 'transparent';
            });
            menu.appendChild(item);
        });

        menu.style.left = e.clientX + 'px';
        menu.style.top = e.clientY + 'px';
        document.body.appendChild(menu);
        this.currentContextMenu = menu;

        // Adjust position if menu goes off screen
        const rect = menu.getBoundingClientRect();
        if (rect.right > window.innerWidth) {
            menu.style.left = (e.clientX - rect.width) + 'px';
        }
        if (rect.bottom > window.innerHeight) {
            menu.style.top = (e.clientY - rect.height) + 'px';
        }
    }

    hideContextMenu() {
        if (this.currentContextMenu) {
            this.currentContextMenu.remove();
            this.currentContextMenu = null;
        }
    }

    // Auto-save drafts
    setupAutoSave() {
        const chatInput = document.getElementById('chatInput');
        let saveTimeout;

        chatInput.addEventListener('input', () => {
            clearTimeout(saveTimeout);
            saveTimeout = setTimeout(() => {
                this.saveDraft(chatInput.value);
            }, 1000);
        });

        // Restore draft on load
        const draft = this.loadDraft();
        if (draft) {
            chatInput.value = draft;
            this.app.updateCharCount();
            this.app.adjustTextareaHeight();
        }
    }

    saveDraft(content) {
        if (content.trim().length > 0) {
            localStorage.setItem('chat-draft', content);
        } else {
            localStorage.removeItem('chat-draft');
        }
    }

    loadDraft() {
        return localStorage.getItem('chat-draft');
    }

    clearDraft() {
        localStorage.removeItem('chat-draft');
    }

    // Theme toggle
    setupThemeToggle() {
        const themeToggle = document.createElement('button');
        themeToggle.innerHTML = '🌙';
        themeToggle.className = 'btn btn-icon theme-toggle';
        themeToggle.title = 'Toggle dark mode';
        themeToggle.style.marginRight = 'var(--space-2)';
        
        const headerActions = document.querySelector('.header-actions');
        headerActions.insertBefore(themeToggle, headerActions.firstChild);

        themeToggle.addEventListener('click', () => {
            this.toggleTheme();
        });

        // Set initial theme
        const savedTheme = localStorage.getItem('theme') || 'light';
        this.setTheme(savedTheme);
    }

    toggleTheme() {
        const currentTheme = document.documentElement.dataset.theme || 'light';
        const newTheme = currentTheme === 'light' ? 'dark' : 'light';
        this.setTheme(newTheme);
    }

    setTheme(theme) {
        document.documentElement.dataset.theme = theme;
        localStorage.setItem('theme', theme);
        
        const toggle = document.querySelector('.theme-toggle');
        toggle.innerHTML = theme === 'light' ? '🌙' : '☀️';
        toggle.title = theme === 'light' ? 'Switch to dark mode' : 'Switch to light mode';
    }

    // Search functionality
    setupSearch() {
        this.searchOverlay = document.createElement('div');
        this.searchOverlay.className = 'search-overlay hidden';
        this.searchOverlay.innerHTML = `
            <div class="search-container">
                <input type="text" placeholder="Search messages..." class="search-input" />
                <div class="search-results"></div>
            </div>
        `;
        document.body.appendChild(this.searchOverlay);

        const searchInput = this.searchOverlay.querySelector('.search-input');
        searchInput.addEventListener('input', (e) => {
            this.performSearch(e.target.value);
        });

        this.searchOverlay.addEventListener('click', (e) => {
            if (e.target === this.searchOverlay) {
                this.closeSearch();
            }
        });
    }

    openSearch() {
        this.searchOverlay.classList.remove('hidden');
        this.searchOverlay.querySelector('.search-input').focus();
    }

    closeSearch() {
        this.searchOverlay.classList.add('hidden');
    }

    performSearch(query) {
        // Implementation for searching through messages
        const results = this.app.messageHistory.filter(msg => 
            msg.content.toLowerCase().includes(query.toLowerCase())
        );
        this.displaySearchResults(results, query);
    }

    displaySearchResults(results, query) {
        const container = this.searchOverlay.querySelector('.search-results');
        container.innerHTML = results.map(result => `
            <div class="search-result">
                <span class="search-result-type">${result.type}</span>
                <div class="search-result-content">${this.highlightSearchTerm(result.content, query)}</div>
            </div>
        `).join('');
    }

    highlightSearchTerm(content, query) {
        if (!query) return content;
        const regex = new RegExp(`(${query})`, 'gi');
        return content.replace(regex, '<mark>$1</mark>');
    }

    // Notification system
    setupNotifications() {
        this.notificationContainer = document.createElement('div');
        this.notificationContainer.className = 'notification-container';
        this.notificationContainer.style.cssText = `
            position: fixed;
            top: var(--space-4);
            right: var(--space-4);
            z-index: 1003;
            pointer-events: none;
        `;
        document.body.appendChild(this.notificationContainer);
    }

    showNotification(message, type = 'info', duration = 3000) {
        const notification = document.createElement('div');
        notification.className = `notification notification-${type}`;
        notification.style.cssText = `
            background: white;
            border: 1px solid var(--gray-200);
            border-radius: var(--radius-lg);
            padding: var(--space-3) var(--space-4);
            margin-bottom: var(--space-2);
            box-shadow: var(--shadow-lg);
            pointer-events: auto;
            transform: translateX(100%);
            transition: transform 300ms ease;
            max-width: 300px;
        `;

        const colors = {
            info: 'var(--primary-500)',
            success: 'var(--success-500)',
            warning: 'var(--warning-500)',
            error: 'var(--error-500)'
        };

        notification.style.borderLeftColor = colors[type];
        notification.style.borderLeftWidth = '4px';
        notification.textContent = message;

        this.notificationContainer.appendChild(notification);

        // Animate in
        setTimeout(() => {
            notification.style.transform = 'translateX(0)';
        }, 10);

        // Auto remove
        setTimeout(() => {
            notification.style.transform = 'translateX(100%)';
            setTimeout(() => {
                if (notification.parentNode) {
                    notification.parentNode.removeChild(notification);
                }
            }, 300);
        }, duration);

        // Click to dismiss
        notification.addEventListener('click', () => {
            notification.style.transform = 'translateX(100%)';
            setTimeout(() => {
                if (notification.parentNode) {
                    notification.parentNode.removeChild(notification);
                }
            }, 300);
        });
    }

    // Utility methods
    focusInput() {
        document.getElementById('chatInput').focus();
    }

    clearChat() {
        if (confirm('Are you sure you want to clear the chat?')) {
            this.app.clearChat();
            this.showNotification('Chat cleared', 'info');
        }
    }

    toggleSidebar() {
        const sidebar = document.querySelector('.sidebar');
        sidebar.classList.toggle('open');
    }

    openSettings() {
        this.app.showSettings();
    }

    newChat() {
        this.clearChat();
        this.showNotification('Started new chat', 'info');
    }

    handleEscape() {
        this.hideContextMenu();
        this.closeSearch();
        if (this.searchOverlay && !this.searchOverlay.classList.contains('hidden')) {
            this.closeSearch();
        }
    }

    sendMessage() {
        this.app.handleSendMessage();
        this.clearDraft();
    }

    copyLastResponse() {
        const messages = document.querySelectorAll('.assistant-message');
        if (messages.length > 0) {
            const lastMessage = messages[messages.length - 1];
            this.copyMessageContent(lastMessage);
        }
    }

    copyMessageContent(message) {
        const content = message.querySelector('.message-content').textContent;
        navigator.clipboard.writeText(content).then(() => {
            this.showNotification('Message copied to clipboard', 'success');
        });
    }

    editMessage(message) {
        const content = message.querySelector('.message-content').textContent;
        document.getElementById('chatInput').value = content;
        this.app.updateCharCount();
        this.app.adjustTextareaHeight();
        document.getElementById('chatInput').focus();
    }

    deleteMessage(message) {
        if (confirm('Delete this message?')) {
            message.remove();
            this.showNotification('Message deleted', 'info');
        }
    }

    quoteMessage(message) {
        const content = message.querySelector('.message-content').textContent;
        const quoted = `> ${content.split('\n').join('\n> ')}\n\n`;
        const input = document.getElementById('chatInput');
        input.value = quoted + input.value;
        this.app.updateCharCount();
        this.app.adjustTextareaHeight();
        input.focus();
    }

    saveMessage(message) {
        const content = message.querySelector('.message-content').textContent;
        const blob = new Blob([content], { type: 'text/plain' });
        const url = URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = `message-${Date.now()}.txt`;
        a.click();
        URL.revokeObjectURL(url);
        this.showNotification('Message saved', 'success');
    }

    undoLastMessage() {
        const messages = document.querySelectorAll('.message');
        if (messages.length > 1) { // Keep at least the welcome message
            messages[messages.length - 1].remove();
            this.showNotification('Last message removed', 'info');
        }
    }

    showHelp() {
        const helpContent = `
# LLM Dev Agent - Keyboard Shortcuts

**Chat Controls:**
- Ctrl+Enter: Send message
- Ctrl+/: Focus input
- Ctrl+Shift+K: Clear chat
- Ctrl+N: New chat
- Ctrl+Z: Undo last message

**Navigation:**
- Ctrl+B: Toggle sidebar
- Ctrl+F: Search messages
- Ctrl+,: Open settings
- Escape: Close dialogs

**Message Actions:**
- Ctrl+Shift+C: Copy last response
- Right-click on message: Context menu

**Files:**
- Drag & drop: Upload text files

**Other:**
- F1: Show this help
        `;
        
        this.app.addMessage('system', helpContent);
    }

    addShortcutHints() {
        const inputHint = document.querySelector('.input-hint');
        if (inputHint) {
            inputHint.innerHTML = `
                Press <kbd>Ctrl+Enter</kbd> to send • 
                <kbd>Ctrl+/</kbd> to focus • 
                <kbd>F1</kbd> for help
            `;
        }

        // Add CSS for kbd elements
        const style = document.createElement('style');
        style.textContent = `
            kbd {
                background: var(--gray-100);
                border: 1px solid var(--gray-300);
                border-radius: var(--radius-sm);
                padding: 0.1em 0.3em;
                font-family: var(--font-mono);
                font-size: 0.85em;
                font-weight: 600;
                color: var(--gray-700);
                box-shadow: inset 0 -1px 0 var(--gray-300);
            }
        `;
        document.head.appendChild(style);
    }
}

// Initialize UX enhancements when app is loaded
if (typeof window !== 'undefined') {
    document.addEventListener('DOMContentLoaded', () => {
        setTimeout(() => {
            if (window.devAgent) {
                window.uxEnhancements = new UXEnhancements(window.devAgent);
            }
        }, 100);
    });
}

if (typeof module !== 'undefined' && module.exports) {
    module.exports = UXEnhancements;
}