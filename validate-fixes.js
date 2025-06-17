#!/usr/bin/env node

// Validation script to check that all fixes are working
// Run this with: node validate-fixes.js

const fs = require('fs');
const path = require('path');

console.log('🔍 === VALIDATION SCRIPT FOR FILE EXPLORER FIXES ===\n');

// Test 1: Check if setupTextareaAutoResize function exists
console.log('📋 Test 1: Checking for setupTextareaAutoResize function...');
const appJsPath = path.join(__dirname, 'ui', 'js', 'app.js');
const appJsContent = fs.readFileSync(appJsPath, 'utf8');

if (appJsContent.includes('setupTextareaAutoResize()')) {
    console.log('✅ setupTextareaAutoResize function found in app.js');
} else {
    console.log('❌ setupTextareaAutoResize function NOT found in app.js');
    process.exit(1);
}

// Test 2: Check if function is properly defined
if (appJsContent.includes('setupTextareaAutoResize() {')) {
    console.log('✅ setupTextareaAutoResize function is properly defined');
} else {
    console.log('❌ setupTextareaAutoResize function is called but not defined');
    process.exit(1);
}

// Test 3: Check Tauri command parameter format
console.log('\n📋 Test 2: Checking Tauri command parameter format...');
if (appJsContent.includes("invoke('list_directory', { path: '.' })")) {
    console.log('✅ list_directory command uses correct parameter format');
} else if (appJsContent.includes("invoke('list_directory', '.')")) {
    console.log('❌ list_directory command uses incorrect parameter format (should be object)');
    process.exit(1);
} else {
    console.log('❌ list_directory command not found');
    process.exit(1);
}

// Test 4: Check for syntax errors by trying to parse JavaScript
console.log('\n📋 Test 3: Checking JavaScript syntax...');
try {
    // Basic syntax check (won't catch runtime errors but will catch syntax errors)
    new Function(appJsContent);
    console.log('✅ JavaScript syntax is valid');
} catch (error) {
    console.log('❌ JavaScript syntax error:', error.message);
    process.exit(1);
}

// Test 5: Check if all required functions exist
console.log('\n📋 Test 4: Checking for all required functions...');
const requiredFunctions = [
    'setupEventListeners',
    'setupTextareaAutoResize',
    'updateCharCount',
    'loadFileTree',
    'renderFileTree',
    'testWhitelist',
    'startFileWatching',
    'debugAppState'
];

let allFunctionsFound = true;
for (const funcName of requiredFunctions) {
    if (appJsContent.includes(`${funcName}() {`) || appJsContent.includes(`${funcName}(`)) {
        console.log(`✅ ${funcName} function found`);
    } else {
        console.log(`❌ ${funcName} function NOT found`);
        allFunctionsFound = false;
    }
}

if (!allFunctionsFound) {
    console.log('\n❌ Some required functions are missing');
    process.exit(1);
}

// Test 6: Check that Rust code compiles
console.log('\n📋 Test 5: Checking Rust compilation...');
const { execSync } = require('child_process');

try {
    execSync('cargo check', { cwd: path.join(__dirname, 'src-tauri'), stdio: 'pipe' });
    console.log('✅ Rust code compiles successfully');
} catch (error) {
    console.log('❌ Rust compilation failed:', error.message);
    process.exit(1);
}

// All tests passed
console.log('\n🎉 === ALL VALIDATION TESTS PASSED! ===');
console.log('');
console.log('✅ setupTextareaAutoResize function restored');
console.log('✅ Tauri command parameter format fixed');
console.log('✅ JavaScript syntax is valid');
console.log('✅ All required functions exist');
console.log('✅ Rust code compiles successfully');
console.log('');
console.log('🚀 The file explorer should now work correctly!');
console.log('');
console.log('💡 To test the app:');
console.log('   1. Run: npm run dev -- --key test-key');
console.log('   2. Open browser console');
console.log('   3. Run: new FileExplorerTester().runAllTests()');
console.log('   4. Try Ctrl+R to manually refresh file tree');
console.log('   5. Try Ctrl+D to debug app state');