# LLM Dev Agent UI

A clean, modern frontend interface for the LLM Dev Agent developer tool. This UI provides a chat interface for interacting with AI assistance and a file explorer for navigating project files.

## Features

- **Modern Chat Interface**: Clean, responsive chat UI with message formatting support
- **File Explorer**: Sidebar file tree navigation with project structure visualization
- **Responsive Design**: Works on desktop, tablet, and mobile devices
- **Developer-Focused**: Optimized for coding tasks with syntax highlighting support
- **Vanilla JavaScript**: No frameworks - lightweight and fast
- **Professional Styling**: Clean design using CSS Grid, Flexbox, and modern CSS features

## File Structure

```
ui/
├── index.html          # Main HTML file
├── css/
│   └── styles.css      # Complete CSS styles with CSS variables
├── js/
│   ├── app.js          # Main application logic
│   └── config.js       # Configuration settings
├── assets/             # Static assets (images, icons)
└── README.md          # This file
```

## Getting Started

1. **Open the UI**: Simply open `index.html` in your web browser
2. **Local Development**: Use a local server for best experience:
   ```bash
   # Using Python
   python -m http.server 8000
   
   # Using Node.js
   npx serve .
   
   # Using PHP
   php -S localhost:8000
   ```
3. **Navigate to**: `http://localhost:8000`

## Configuration

The UI can be configured by modifying `js/config.js`:

- **API endpoints**: Update backend connection settings
- **UI behavior**: Customize message limits, animations, etc.
- **File explorer**: Configure file types, icons, and display options
- **Theme settings**: Light/dark mode preferences
- **Keyboard shortcuts**: Customize hotkeys

## Key Components

### Chat Interface
- Message history with timestamps
- Auto-resizing text input
- Character counter
- Keyboard shortcuts (Ctrl+Enter to send)
- Message formatting (markdown-like)
- Loading states and animations

### File Explorer
- Tree view of project structure
- File type icons
- Click to select files
- Refresh functionality
- Responsive collapsing on mobile

### Responsive Design
- Desktop: Full sidebar + chat layout
- Tablet: Collapsible sidebar
- Mobile: Single column with hidden sidebar

## Styling

The CSS uses a design system with:
- **CSS Custom Properties**: For consistent theming
- **Modern Layout**: CSS Grid and Flexbox
- **Typography**: Inter font for UI, JetBrains Mono for code
- **Color System**: Semantic color tokens
- **Spacing System**: Consistent spacing scale
- **Component-Based**: Modular CSS architecture

## JavaScript Architecture

- **Class-based**: Main `DevAgentApp` class managing state
- **Event-driven**: Clean event handling and delegation
- **Modular**: Separated concerns with clear methods
- **Extensible**: Easy to add new features
- **Error handling**: Robust error states and user feedback

## Browser Support

- Chrome/Edge 88+
- Firefox 85+
- Safari 14+
- Mobile browsers with modern CSS support

## Customization

### Adding New File Types
Update the `iconMap` in `config.js`:
```javascript
iconMap: {
    '.newext': '🆕',
    // ... other extensions
}
```

### Modifying Themes
Update CSS custom properties in `:root`:
```css
:root {
    --primary-500: #your-color;
    /* ... other variables */
}
```

### Adding Keyboard Shortcuts
Add to the shortcuts configuration and implement in `app.js`:
```javascript
shortcuts: {
    newShortcut: 'Ctrl+Shift+N'
}
```

## Integration

The UI is designed to integrate with a backend API. Key integration points:

1. **Chat API**: Replace `simulateAPICall()` with real API calls
2. **File System**: Replace mock file tree with real file system API
3. **Settings**: Connect settings panel to backend configuration
4. **Authentication**: Add user authentication if needed

## Performance

- Lazy loading for large file trees
- Virtual scrolling for chat history (can be added)
- Optimized animations and transitions
- Minimal JavaScript bundle size
- Efficient DOM manipulation

## Accessibility

- Semantic HTML structure
- ARIA labels and roles
- Keyboard navigation support
- Focus management
- Screen reader friendly
- High contrast support

## Development Notes

- Uses modern ES6+ JavaScript features
- No build process required for basic usage
- Can be enhanced with bundlers (webpack, vite, etc.)
- TypeScript definitions can be added for better development experience
- Service worker support can be added for offline functionality

## Future Enhancements

- Dark mode toggle
- File editing capabilities
- Code syntax highlighting
- Auto-completion
- Voice input
- Drag and drop file upload
- Split panes for multiple views
- Plugin system for extensions