#!/usr/bin/env -S deno run --allow-all

// Build script for Algorithm Shield

import * as esbuild from 'esbuild'
import { copy, ensureDir } from '@std/fs'
import { join } from '@std/path'

const ROOT = Deno.cwd()
const DIST = join(ROOT, 'dist')

console.log('🛡️ Building Algorithm Shield...\n')

// Clean dist
try {
  await Deno.remove(DIST, { recursive: true })
} catch {
  // Ignore if doesn't exist
}
await ensureDir(DIST)

// Step 1: Compile ReScript
console.log('1️⃣ Compiling ReScript...')
const rescriptBuild = new Deno.Command('npx', {
  args: ['rescript', 'build'],
  cwd: ROOT
})
const rescriptResult = await rescriptBuild.output()
if (!rescriptResult.success) {
  console.error('❌ ReScript compilation failed')
  Deno.exit(1)
}
console.log('✅ ReScript compiled\n')

// Step 2: Build Rust WASM
console.log('2️⃣ Building Rust WASM...')
const wasmBuild = new Deno.Command('wasm-pack', {
  args: [
    'build',
    '--target', 'web',
    '--out-dir', join(DIST, 'wasm'),
    '--release',
    join(ROOT, 'src', 'rust')
  ],
  cwd: ROOT
})
const wasmResult = await wasmBuild.output()
if (!wasmResult.success) {
  console.error('❌ WASM build failed')
  Deno.exit(1)
}
console.log('✅ WASM built\n')

// Step 3: Bundle extension scripts with esbuild
console.log('3️⃣ Bundling extension scripts...')

await esbuild.build({
  entryPoints: [
    join(ROOT, 'src', 'background.js'),
    join(ROOT, 'src', 'content.js'),
    join(ROOT, 'src', 'ui', 'popup.js')
  ],
  bundle: true,
  outdir: DIST,
  format: 'esm',
  platform: 'browser',
  target: 'chrome120'
})

console.log('✅ Scripts bundled\n')

// Step 4: Copy static files
console.log('4️⃣ Copying static files...')

await copy(join(ROOT, 'public'), DIST, { overwrite: true })
await copy(join(ROOT, 'manifest.json'), join(DIST, 'manifest.json'), { overwrite: true })

console.log('✅ Static files copied\n')

console.log('🎉 Build complete! Extension ready in dist/')
console.log('Load dist/ as unpacked extension in Chrome')

esbuild.stop()
