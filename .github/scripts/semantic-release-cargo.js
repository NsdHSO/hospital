const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

/**
 * A semantic-release plugin to update the Cargo.toml version and build a release binary.
 */
module.exports = {
  // Verify the conditions for the plugin to run (e.g. check if all needed configuration is present)
  verifyConditions: async (pluginConfig, context) => {
    // Verify that Cargo.toml exists
    const cargoPath = path.resolve('./Cargo.toml');
    if (!fs.existsSync(cargoPath)) {
      throw new Error('Cargo.toml not found');
    }
  },

  // Prepare the release (e.g. update version in files)
  prepare: async (pluginConfig, context) => {
    const { nextRelease } = context;
    const version = nextRelease.version;

    context.logger.log(`Updating Cargo.toml to version ${version}`);
    
    // Read Cargo.toml
    const cargoPath = path.resolve('./Cargo.toml');
    let cargoContent = fs.readFileSync(cargoPath, 'utf8');
    
    // Update the version
    cargoContent = cargoContent.replace(
      /^version\s*=\s*["'].*["']/m,
      `version = "${version}"`
    );
    
    // Write the updated content back to Cargo.toml
    fs.writeFileSync(cargoPath, cargoContent);
    
    // Build the release binary
    context.logger.log('Building release binary');
    try {
      execSync('cargo build --release', { stdio: 'inherit' });
      context.logger.log('Successfully built release binary');
    } catch (error) {
      context.logger.error('Failed to build release binary');
      context.logger.error(error);
      // Continue with the release process even if the build fails
    }
  }
};
