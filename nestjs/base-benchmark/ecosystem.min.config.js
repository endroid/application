module.exports = {
  apps: [{
    name: "min",
    script: "./application/dist/min.js",
    instances: 4,
    exec_mode: "cluster"
  }]
}
