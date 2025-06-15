#!/usr/bin/env node

const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

// Icon sizes needed for different platforms
const ICON_SIZES = {
  // PNG sizes for various uses
  png: [16, 20, 24, 29, 30, 32, 40, 44, 48, 50, 55, 57, 58, 60, 64, 66, 72, 76, 80, 87, 88, 100, 114, 120, 128, 144, 152, 167, 172, 180, 196, 216, 256, 512, 1024],
  // ICO needs multiple sizes embedded
  ico: [16, 20, 24, 32, 40, 48, 64, 96, 128, 256],
  // ICNS for macOS (will be generated from PNGs)
  icns: [16, 32, 64, 128, 256, 512, 1024]
};

const PATHS = {
  source: path.join(__dirname, '..', 'src-tauri', 'icons', 'icon.svg'),
  iconsDir: path.join(__dirname, '..', 'src-tauri', 'icons'),
  tempDir: path.join(__dirname, '..', 'temp-icons')
};

console.log('🚀 Generating icons for Rustic Agent...\n');

// Check if ImageMagick is available
function checkImageMagick() {
  try {
    execSync('convert -version', { stdio: 'ignore' });
    return true;
  } catch (error) {
    return false;
  }
}

// Check if librsvg is available (for better SVG conversion)
function checkLibrsvg() {
  try {
    execSync('rsvg-convert --version', { stdio: 'ignore' });
    return true;
  } catch (error) {
    return false;
  }
}

// Create directory if it doesn't exist
function ensureDir(dir) {
  if (!fs.existsSync(dir)) {
    fs.mkdirSync(dir, { recursive: true });
  }
}

// Convert SVG to PNG using the best available tool
function svgToPng(inputSvg, outputPng, size) {
  const width = size;
  const height = size;
  
  if (checkLibrsvg()) {
    // Use librsvg for best quality SVG conversion
    execSync(`rsvg-convert -w ${width} -h ${height} -o "${outputPng}" "${inputSvg}"`);
  } else if (checkImageMagick()) {
    // Fallback to ImageMagick
    execSync(`convert -background transparent -size ${width}x${height} "${inputSvg}" "${outputPng}"`);
  } else {
    throw new Error('Neither rsvg-convert nor ImageMagick (convert) is available. Please install one of them.');
  }
}

// Generate PNG icons
function generatePngs() {
  console.log('📱 Generating PNG icons...');
  ensureDir(PATHS.tempDir);
  
  const pngSizes = [...new Set([...ICON_SIZES.png, ...ICON_SIZES.ico, ...ICON_SIZES.icns])];
  
  pngSizes.forEach(size => {
    const outputPath = path.join(PATHS.tempDir, `icon-${size}.png`);
    try {
      svgToPng(PATHS.source, outputPath, size);
      console.log(`  ✅ Generated ${size}x${size} PNG`);
    } catch (error) {
      console.error(`  ❌ Failed to generate ${size}x${size} PNG:`, error.message);
    }
  });
  
  // Copy the 128x128 as the main icon.png
  const mainPngSrc = path.join(PATHS.tempDir, 'icon-128.png');
  const mainPngDest = path.join(PATHS.iconsDir, 'icon.png');
  if (fs.existsSync(mainPngSrc)) {
    fs.copyFileSync(mainPngSrc, mainPngDest);
    console.log('  ✅ Updated main icon.png (128x128)');
  }
}

// Generate ICO file (Windows)
function generateIco() {
  console.log('🪟 Generating ICO file for Windows...');
  
  const pngFiles = ICON_SIZES.ico.map(size => 
    path.join(PATHS.tempDir, `icon-${size}.png`)
  ).filter(file => fs.existsSync(file));
  
  if (pngFiles.length === 0) {
    console.error('  ❌ No PNG files found for ICO generation');
    return;
  }
  
  try {
    if (checkImageMagick()) {
      const outputPath = path.join(PATHS.iconsDir, 'icon.ico');
      const inputFiles = pngFiles.join(' ');
      execSync(`convert ${inputFiles} "${outputPath}"`);
      console.log('  ✅ Generated Windows ICO file');
    } else {
      console.warn('  ⚠️  ImageMagick not available, ICO generation skipped');
    }
  } catch (error) {
    console.error('  ❌ Failed to generate ICO:', error.message);
  }
}

// Generate ICNS file (macOS)
function generateIcns() {
  console.log('🍎 Generating ICNS file for macOS...');
  
  // Create iconset directory structure
  const iconsetDir = path.join(PATHS.tempDir, 'icon.iconset');
  ensureDir(iconsetDir);
  
  // macOS iconset naming convention
  const iconsetFiles = [
    { size: 16, name: 'icon_16x16.png' },
    { size: 32, name: 'icon_16x16@2x.png' },
    { size: 32, name: 'icon_32x32.png' },
    { size: 64, name: 'icon_32x32@2x.png' },
    { size: 128, name: 'icon_128x128.png' },
    { size: 256, name: 'icon_128x128@2x.png' },
    { size: 256, name: 'icon_256x256.png' },
    { size: 512, name: 'icon_256x256@2x.png' },
    { size: 512, name: 'icon_512x512.png' },
    { size: 1024, name: 'icon_512x512@2x.png' }
  ];
  
  // Copy PNGs to iconset directory with correct names
  iconsetFiles.forEach(({ size, name }) => {
    const srcPath = path.join(PATHS.tempDir, `icon-${size}.png`);
    const destPath = path.join(iconsetDir, name);
    
    if (fs.existsSync(srcPath)) {
      fs.copyFileSync(srcPath, destPath);
    }
  });
  
  try {
    // Generate ICNS file using iconutil (macOS) or png2icns (Linux alternative)
    const outputPath = path.join(PATHS.iconsDir, 'icon.icns');
    
    if (process.platform === 'darwin') {
      execSync(`iconutil -c icns "${iconsetDir}" -o "${outputPath}"`);
      console.log('  ✅ Generated macOS ICNS file');
    } else {
      // Try to use ImageMagick as fallback
      if (checkImageMagick()) {
        const pngFiles = iconsetFiles
          .map(({ size }) => path.join(PATHS.tempDir, `icon-${size}.png`))
          .filter(file => fs.existsSync(file))
          .join(' ');
        
        if (pngFiles) {
          execSync(`convert ${pngFiles} "${outputPath}"`);
          console.log('  ✅ Generated ICNS file (using ImageMagick)');
        }
      } else {
        console.warn('  ⚠️  ICNS generation requires macOS iconutil or ImageMagick');
      }
    }
  } catch (error) {
    console.error('  ❌ Failed to generate ICNS:', error.message);
  }
  
  // Clean up iconset directory
  try {
    fs.rmSync(iconsetDir, { recursive: true, force: true });
  } catch (error) {
    console.warn('  ⚠️  Failed to clean up iconset directory:', error.message);
  }
}

// Generate additional platform-specific sizes
function generatePlatformSpecific() {
  console.log('🌐 Generating platform-specific icon sizes...');
  
  const specialSizes = [
    { size: 16, name: 'icon-16.png', desc: 'Small app icon' },
    { size: 32, name: 'icon-32.png', desc: 'Medium app icon' },
    { size: 64, name: 'icon-64.png', desc: 'Large app icon' },
    { size: 256, name: 'icon-256.png', desc: 'Hi-res app icon' },
    { size: 512, name: 'icon-512.png', desc: 'Retina app icon' }
  ];
  
  specialSizes.forEach(({ size, name, desc }) => {
    const srcPath = path.join(PATHS.tempDir, `icon-${size}.png`);
    const destPath = path.join(PATHS.iconsDir, name);
    
    if (fs.existsSync(srcPath)) {
      fs.copyFileSync(srcPath, destPath);
      console.log(`  ✅ Generated ${name} (${desc})`);
    }
  });
}

// Clean up temporary files
function cleanup() {
  try {
    if (fs.existsSync(PATHS.tempDir)) {
      fs.rmSync(PATHS.tempDir, { recursive: true, force: true });
      console.log('🧹 Cleaned up temporary files');
    }
  } catch (error) {
    console.warn('⚠️  Failed to clean up temporary files:', error.message);
  }
}

// Main execution
async function main() {
  try {
    // Check prerequisites
    console.log('🔍 Checking prerequisites...');
    
    if (!fs.existsSync(PATHS.source)) {
      throw new Error(`Source SVG not found: ${PATHS.source}`);
    }
    
    const hasLibrsvg = checkLibrsvg();
    const hasImageMagick = checkImageMagick();
    
    if (!hasLibrsvg && !hasImageMagick) {
      console.error('❌ Neither rsvg-convert nor ImageMagick found!');
      console.log('\nTo install the required tools:');
      console.log('Ubuntu/Debian: sudo apt-get install librsvg2-bin imagemagick');
      console.log('macOS: brew install librsvg imagemagick');
      console.log('Or run: npm run icons:install-deps');
      process.exit(1);
    }
    
    console.log(`✅ Using ${hasLibrsvg ? 'librsvg' : 'ImageMagick'} for SVG conversion`);
    
    // Generate all icon formats
    generatePngs();
    generateIco();
    generateIcns();
    generatePlatformSpecific();
    
    console.log('\n✨ Icon generation completed successfully!');
    console.log('\nGenerated files:');
    console.log(`  📁 ${PATHS.iconsDir}/`);
    
    // List generated files
    const files = fs.readdirSync(PATHS.iconsDir);
    files.forEach(file => {
      const filePath = path.join(PATHS.iconsDir, file);
      const stats = fs.statSync(filePath);
      const size = (stats.size / 1024).toFixed(1);
      console.log(`     ${file} (${size} KB)`);
    });
    
  } catch (error) {
    console.error('\n❌ Icon generation failed:', error.message);
    process.exit(1);
  } finally {
    cleanup();
  }
}

// Handle script termination
process.on('SIGINT', () => {
  console.log('\n⏹️  Icon generation interrupted');
  cleanup();
  process.exit(1);
});

process.on('SIGTERM', () => {
  console.log('\n⏹️  Icon generation terminated');
  cleanup();
  process.exit(1);
});

if (require.main === module) {
  main();
}