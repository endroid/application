module.exports = {
  apps: [{
    name: "application-min",
    script: "./dist/src/min.js",
    instances: 4,
    exec_mode: "cluster"
  }]
}
