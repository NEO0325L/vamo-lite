// Reproduce the attribution inventory from Cargo metadata for the selected target.
// No local paths, session profiles or build logs are written to the notice file.
import { execFileSync } from 'node:child_process';
import { readFileSync, writeFileSync, readdirSync, existsSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';

const target = process.argv[2] ?? 'x86_64-unknown-linux-gnu';
if (!/^[a-z0-9_-]+$/.test(target)) throw new Error('Invalid target triple');
const metadata = JSON.parse(execFileSync('cargo', [
  'metadata', '--locked', '--format-version', '1', '--filter-platform', target,
  '--manifest-path', 'src-tauri/Cargo.toml'
], { encoding: 'utf8', maxBuffer: 32 * 1024 * 1024 }));
const root = metadata.packages.find(p => p.name === 'vamo-lite');
const lines = [
  `Vamo Lite ${root.version} - Third-party dependency notices (${target})`,
  'Original project: Copyright (c) 2026 NEO0325L. See LICENSE.',
  'This inventory includes Cargo-resolved dependencies, including build tools; not every entry is linked into the executable.',
  'System libraries installed by the Linux package manager retain their own notices and licenses in /usr/share/doc.',
  'On Windows, Microsoft WebView2 and Visual C++ components retain their own redistribution terms; they are not licensed under the project MIT license.'
];
let count = 0;
for (const pkg of metadata.packages.filter(p => p.name !== 'vamo-lite').sort((a,b) => a.name.localeCompare(b.name) || a.version.localeCompare(b.version))) {
  const folder = dirname(pkg.manifest_path);
  lines.push('\n================================================================', `${pkg.name} ${pkg.version}`,
    `Declared license: ${pkg.license ?? 'See upstream license file'}`,
    `Source: https://crates.io/crates/${pkg.name}/${pkg.version}`);
  if (pkg.repository) lines.push(`Repository: ${pkg.repository}`);
  const files = new Set();
  if (existsSync(folder)) {
    for (const entry of readdirSync(folder, { withFileTypes: true })) {
      if (entry.isFile() && /^(LICENSE|LICENCE|COPYING|NOTICE)([.\-_]|$)/i.test(entry.name)) files.add(join(folder, entry.name));
    }
    if (pkg.license_file) {
      const declared = resolve(folder, pkg.license_file);
      if (existsSync(declared)) files.add(declared);
    }
  }
  for (const file of [...files].sort()) lines.push('\n--- Upstream notice ---', readFileSync(file, 'utf8'));
  if (files.size === 0) lines.push('No standalone notice file was available in the Cargo package. See the declared license and upstream source above.');
  count++;
}
writeFileSync('THIRD_PARTY_NOTICES.txt', lines.join('\n') + '\n', 'utf8');
console.log(`Generated notices for ${count} Cargo packages (${target}).`);
