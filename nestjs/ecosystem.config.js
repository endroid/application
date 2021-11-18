module.exports = {
  apps: [{
    name: "application",
    script: "./application/dist/main.js",
    instances: 2,
    watch: ["application"],
    exec_mode: "cluster"
  }]
}
