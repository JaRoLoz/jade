# Jade
Jade is an opinionated build system designed for the Cfx.re plattform.
Jade makes use of a `jade.json` config file inside each resource folder in order to *build it*. Its current features are:

- Lua bundling
- JS building through a package manager (e.g. npm, pnpm, yarn, bun...)
- `fxmanifest.lua` file generation

Jade comes with the capability of using different config files when building a resource depending of the environment. To use this feature, you will need to prefix the config file like with the env that it belongs to: `xxx.jade.json` (`xxx` being the env name). Then when executing jade, you will need to pass the env name through the `--env` flag, e.g. `--env xxx`. If a resource doesn't contain a config that matches that env, it will default to the `jade.json` config file.