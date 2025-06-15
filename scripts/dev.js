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

// Check if placeholder exists and replace it
if (htmlContent.includes('PLACEHOLDER_FOR_DEV_INJECTION')) {
    // Replace the placeholder with the actual API key
    const injectedContent = htmlContent.replace(
        'PLACEHOLDER_FOR_DEV_INJECTION',
        apiKey
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
        // Replace any actual API key back with the placeholder (improved regex)
        const cleanedContent = htmlContent.replace(
            /window\.CLAUDE_API_KEY = "[^"]*"/g,
            'window.CLAUDE_API_KEY = "PLACEHOLDER_FOR_DEV_INJECTION"'
        );
        fs.writeFileSync(htmlPath, cleanedContent);
        console.log('🧹 Cleaned up API key from frontend');
        
        // Verify cleanup was successful
        const verifyContent = fs.readFileSync(htmlPath, 'utf8');
        if (verifyContent.includes('sk-ant-')) {
            console.error('⚠️ WARNING: API key may still be present in HTML file!');
        }
    } catch (error) {
        console.warn('⚠️ Could not clean up API key:', error.message);
    }
}