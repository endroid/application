module.exports = {
  apps: [{
    name: "application",
    script: "./application/dist/main.js",
    instances: 4,
    exec_mode: "cluster"
  }]
}
