module.exports = {
  apps: [{
    name: "application-min",
    script: "./dist/src/main-min.js",
    instances: 4,
    exec_mode: "cluster"
  }]
}
