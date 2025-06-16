#!/usr/bin/env node

const { spawn } = require('child_process');

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

// Start Tauri with the API key as an environment variable
const env = { ...process.env, CLAUDE_API_KEY: apiKey };
const tauriProcess = spawn('npx', ['tauri', 'dev'], {
    stdio: 'inherit',
    shell: true,
    env: env
});

tauriProcess.on('close', (code) => {
    process.exit(code);
});