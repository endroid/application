module.exports = {
  apps: [{
    name: "application",
    script: "./dist/src/main.js",
    instances: 4,
    exec_mode: "cluster"
  }]
}
