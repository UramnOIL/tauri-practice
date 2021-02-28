module.exports = {
  webpack(config, options) {
    config.resolve.alias['@src'] = require('path').join(__dirname, 'src')
    return config
  },
}
