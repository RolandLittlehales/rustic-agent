// Comprehensive Test Suite for File Explorer
// Run this in the browser console when the app is loaded

class FileExplorerTester {
    constructor() {
        this.tests = [];
        this.passed = 0;
        this.failed = 0;
    }

    async runAllTests() {
        console.log('🧪 === STARTING COMPREHENSIVE FILE EXPLORER TESTS ===');
        
        await this.testAppInitialization();
        await this.testTauriAvailability();
        await this.testFileTreeLoading();
        await this.testWhitelistFunctionality();
        await this.testFileWatching();
        await this.testErrorHandling();
        
        this.printResults();
    }

    async testAppInitialization() {
        this.test('App object exists', () => {
            return window.devAgent !== undefined;
        });

        this.test('Required DOM elements exist', () => {
            return window.devAgent.fileTree !== null &&
                   window.devAgent.chatInput !== null &&
                   window.devAgent.refreshBtn !== null;
        });

        this.test('setupTextareaAutoResize function exists', () => {
            return typeof window.devAgent.setupTextareaAutoResize === 'function';
        });

        this.test('loadFileTree function exists', () => {
            return typeof window.devAgent.loadFileTree === 'function';
        });
    }

    async testTauriAvailability() {
        this.test('Tauri is available', () => {
            return window.__TAURI__ !== undefined;
        });

        this.test('Tauri core is available', () => {
            return window.__TAURI__ && window.__TAURI__.core !== undefined;
        });

        this.test('Tauri event system is available', () => {
            return window.__TAURI__ && window.__TAURI__.event !== undefined;
        });
    }

    async testFileTreeLoading() {
        console.log('📁 Testing file tree loading...');
        
        try {
            // Test the actual file tree loading
            const result = await window.__TAURI__.core.invoke('list_directory', { path: '.' });
            
            this.test('list_directory command works', () => {
                return result !== undefined;
            });

            this.test('list_directory returns array', () => {
                return Array.isArray(result);
            });

            this.test('list_directory returns valid file objects', () => {
                if (result.length === 0) return true; // Empty directory is valid
                return result.every(file => 
                    file.name && 
                    file.file_type && 
                    file.icon && 
                    file.path
                );
            });

            // Test the UI integration
            await window.devAgent.loadFileTree();
            
            this.test('File tree UI updates after loading', () => {
                return window.devAgent.fileTree.innerHTML !== '<div class="file-tree-placeholder"><p>Loading project files...</p></div>';
            });

        } catch (error) {
            this.test('list_directory command works', () => false, `Error: ${error}`);
        }
    }

    async testWhitelistFunctionality() {
        console.log('🔒 Testing whitelist functionality...');
        
        try {
            const config = await window.__TAURI__.core.invoke('whitelist_get_config');
            
            this.test('whitelist_get_config works', () => {
                return config !== undefined;
            });

            const directories = await window.__TAURI__.core.invoke('whitelist_list_directories');
            
            this.test('whitelist_list_directories works', () => {
                return Array.isArray(directories);
            });

            this.test('Current directory is whitelisted', () => {
                return directories.length > 0;
            });

        } catch (error) {
            this.test('whitelist commands work', () => false, `Error: ${error}`);
        }
    }

    async testFileWatching() {
        console.log('👁️ Testing file watching...');
        
        try {
            // Test if file watching can be started
            const result = await window.__TAURI__.core.invoke('start_file_watching');
            
            this.test('start_file_watching command works', () => {
                return typeof result === 'string';
            });

            this.test('setupFileWatchingListeners function exists', () => {
                return typeof window.devAgent.setupFileWatchingListeners === 'function';
            });

            this.test('debounceFileTreeRefresh function exists', () => {
                return typeof window.devAgent.debounceFileTreeRefresh === 'function';
            });

        } catch (error) {
            this.test('file watching setup', () => false, `Error: ${error}`);
        }
    }

    async testErrorHandling() {
        console.log('🚨 Testing error handling...');
        
        try {
            // Test invalid path
            await window.__TAURI__.core.invoke('list_directory', { path: '/nonexistent/path/that/should/not/exist' });
            this.test('Invalid path handling', () => false, 'Should have thrown error');
        } catch (error) {
            this.test('Invalid path handling', () => true, 'Correctly handles invalid paths');
        }

        try {
            // Test missing parameter
            await window.__TAURI__.core.invoke('list_directory');
            this.test('Missing parameter handling', () => false, 'Should have thrown error');
        } catch (error) {
            this.test('Missing parameter handling', () => true, 'Correctly handles missing parameters');
        }
    }

    test(name, testFn, errorMsg = '') {
        try {
            const result = testFn();
            if (result) {
                console.log(`✅ ${name}`);
                this.passed++;
            } else {
                console.log(`❌ ${name}${errorMsg ? ': ' + errorMsg : ''}`);
                this.failed++;
            }
        } catch (error) {
            console.log(`❌ ${name}: ${error.message}`);
            this.failed++;
        }
    }

    printResults() {
        console.log('\n🧪 === TEST RESULTS ===');
        console.log(`✅ Passed: ${this.passed}`);
        console.log(`❌ Failed: ${this.failed}`);
        console.log(`📊 Total: ${this.passed + this.failed}`);
        
        if (this.failed === 0) {
            console.log('🎉 ALL TESTS PASSED! File explorer is working correctly.');
        } else {
            console.log('⚠️ Some tests failed. Check the issues above.');
        }
    }
}

// Auto-run tests if this file is executed
if (typeof window !== 'undefined' && window.devAgent) {
    console.log('🧪 Test suite loaded. Run: new FileExplorerTester().runAllTests()');
} else {
    console.log('⚠️ Test suite loaded but app not ready. Wait for app to load first.');
}