#!/usr/bin/env -S deno run --allow-all

// Development server for Algorithm Shield

console.log('🛡️ Algorithm Shield - Development Mode\n')
console.log('Watching for changes...\n')

// Watch ReScript files
const rescriptWatch = new Deno.Command('npx', {
  args: ['rescript', 'build', '-w'],
  stdout: 'inherit',
  stderr: 'inherit'
})

const rescriptProcess = rescriptWatch.spawn()

console.log('ReScript watcher started')
console.log('Edit .res files in src/rescript/')
console.log('Changes will auto-compile to .res.js\n')
console.log('Run `deno task build` to build the full extension')

// Cleanup on exit
Deno.addSignalListener('SIGINT', () => {
  console.log('\nStopping watchers...')
  rescriptProcess.kill('SIGTERM')
  Deno.exit(0)
})

await rescriptProcess.status
