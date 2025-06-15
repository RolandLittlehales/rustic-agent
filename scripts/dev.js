#!/usr/bin/env node

const { spawn } = require('child_process');
const fs = require('fs');
const path = require('path');

// Parse command line arguments
const args = process.argv.slice(2);
let apiKey = null;

// Look for --key flag
for (let i = 0; i < args.length; i++) {
    if (args[i] === '--key' && i + 1 < args.length) {
        apiKey = args[i + 1];
        break;
    }
    if (args[i].startsWith('--key=')) {
        apiKey = args[i].split('=')[1];
        break;
    }
}

// Check for environment variable
if (!apiKey) {
    apiKey = process.env.CLAUDE_API_KEY;
}

if (!apiKey) {
    console.error('❌ No Claude API key provided!');
    console.error('');
    console.error('Usage:');
    console.error('  npm run dev -- --key YOUR_API_KEY');
    console.error('  or');
    console.error('  CLAUDE_API_KEY=your_key npm run dev');
    console.error('');
    console.error('Get your API key from: https://console.anthropic.com');
    process.exit(1);
}

console.log('🔑 Claude API key detected');
console.log('🚀 Starting Tauri development server...');

// Inject the API key into the HTML file
const htmlPath = path.join(__dirname, '../ui/index.html');
const htmlContent = fs.readFileSync(htmlPath, 'utf8');

// Check if script tag already exists
if (!htmlContent.includes('window.CLAUDE_API_KEY')) {
    // Add the script tag before closing head
    const injectedContent = htmlContent.replace(
        '</head>',
        `    <script>window.CLAUDE_API_KEY = "${apiKey}";</script>\n</head>`
    );
    fs.writeFileSync(htmlPath, injectedContent);
    
    console.log('✅ API key injected into frontend');
    
    // Clean up on exit
    process.on('exit', () => cleanupApiKey());
    process.on('SIGINT', () => {
        cleanupApiKey();
        process.exit(0);
    });
    process.on('SIGTERM', () => {
        cleanupApiKey();
        process.exit(0);
    });
}

// Start Tauri
const tauriProcess = spawn('npx', ['tauri', 'dev'], {
    stdio: 'inherit',
    shell: true
});

tauriProcess.on('close', (code) => {
    cleanupApiKey();
    process.exit(code);
});

function cleanupApiKey() {
    try {
        const htmlContent = fs.readFileSync(htmlPath, 'utf8');
        const cleanedContent = htmlContent.replace(
            /\s*<script>window\.CLAUDE_API_KEY = ".*?";<\/script>\n/,
            ''
        );
        fs.writeFileSync(htmlPath, cleanedContent);
        console.log('🧹 Cleaned up API key from frontend');
    } catch (error) {
        console.warn('⚠️ Could not clean up API key:', error.message);
    }
}