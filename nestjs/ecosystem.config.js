module.exports = {
  apps: [{
    name: "application",
    script: "./application/dist/main.js",
    instances: 2,
    exec_mode: "cluster"
  }]
}
