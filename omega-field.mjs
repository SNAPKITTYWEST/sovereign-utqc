#!/usr/bin/env node
// omega-field.mjs — SNAPKITTYWEST Ω Field Reader
// Reads all repos in the SNAPKITTYWEST org via GitHub API
// Computes entropy(E), evaluates coherent(system), maps intercoil graph
// Emits SHA-256 WORM seal and patches README between OMEGA-FIELD markers

import { createHash } from 'crypto'
import { readFileSync, writeFileSync } from 'fs'

const ORG = 'SNAPKITTYWEST'
const ENTROPY_THRESHOLD = 0.21
const STALE_DAYS = 30
const TOKEN = process.env.GITHUB_TOKEN || process.env.GH_TOKEN || ''

// Intercoil anchors — repos that share the memory graph / bifrost bus
const INTERCOIL = {
  memory_graph:   ['bob-orchestrator','resonance-core','SNAPKITTY-PROOFS','agent-farm-gauntlet','holy-agents'],
  bifrost_engine: ['bob-orchestrator','holy-agents','apple-ii-universal-machine','DEVFLOW-FINANCE'],
}

async function api(path) {
  const res = await fetch(`https://api.github.com${path}`.replace('/orgs/', '/users/'), {
    headers: {
      Accept: 'application/vnd.github+json',
      ...(TOKEN ? { Authorization: `Bearer ${TOKEN}` } : {}),
    },
  })
  if (!res.ok) throw new Error(`GitHub API ${path} → ${res.status}`)
  return res.json()
}

async function fetchAllRepos() {
  let page = 1, all = []
  while (true) {
    const batch = await api(`/users/${ORG}/repos?per_page=100&page=${page}`)
    all = all.concat(batch)
    if (batch.length < 100) break
    page++
  }
  return all
}

function entropy(repos) {
  const now = Date.now()
  const staleMs = STALE_DAYS * 86400 * 1000
  const stale = repos.filter(r => now - new Date(r.pushed_at).getTime() > staleMs).length
  return stale / repos.length
}

function intercoilCheck(repos, anchor) {
  const names = new Set(repos.map(r => r.name))
  return INTERCOIL[anchor].filter(n => names.has(n))
}

function wormSeal(payload) {
  return createHash('sha256').update(payload).digest('hex')
}

function bar(val, max = 1, width = 20) {
  const filled = Math.round((val / max) * width)
  return '█'.repeat(filled) + '░'.repeat(width - filled)
}

async function main() {
  console.log('⟦ Ω ⟧ reading SNAPKITTYWEST field…')

  const repos = await fetchAllRepos()
  const E = entropy(repos)
  const coherent = E < ENTROPY_THRESHOLD

  const icoilMem  = intercoilCheck(repos, 'memory_graph')
  const icoilBif  = intercoilCheck(repos, 'bifrost_engine')
  const intercoilValid = icoilMem.length > 0 && icoilBif.length > 0

  const resonantCore = coherent && intercoilValid
  const omegaValid   = resonantCore

  const ts      = new Date().toISOString()
  const payload = `SNAPKITTYWEST|${repos.length}|${E.toFixed(6)}|${omegaValid}|${ts}`
  const seal    = wormSeal(payload)

  const statusLine = omegaValid
    ? `✅ \`meta_block(valid)\` — RESONANCE FIELD ACTIVE`
    : `⚠️ \`meta_block(degraded)\` — FIELD BELOW THRESHOLD`

  const pagesRepos = repos.filter(r => r.has_pages)
  const activeRepos = repos.filter(r => {
    const days = (Date.now() - new Date(r.pushed_at).getTime()) / 86400000
    return days < STALE_DAYS
  })

  // Build the README section
  const section = `
<!--OMEGA-FIELD:START-->
<div align="center">

---

## ⟦ Ω ⟧ SNAPKITTYWEST RESONANCE FIELD

${statusLine}

| Metric | Value |
|--------|-------|
| Repos in field | **${repos.length}** |
| Active (< ${STALE_DAYS}d) | **${activeRepos.length}** |
| GitHub Pages live | **${pagesRepos.length}** |
| Entropy E | **${E.toFixed(4)}** / threshold 0.21 |
| Coherent | **${coherent ? 'YES' : 'NO'}** |
| Intercoil · memory\_graph | ${icoilMem.slice(0,4).join(' · ') || 'none'} |
| Intercoil · bifrost | ${icoilBif.slice(0,4).join(' · ') || 'none'} |
| Ω WORM Seal | \`${seal}\` |
| Last field read | \`${ts}\` |

\`\`\`
Entropy field: [${bar(E)}] ${(E * 100).toFixed(1)}%
                           ▲
                     threshold 0.21
\`\`\`

\`\`\`apl
REPO  ← ${repos.length}
STACK ← ⌿REPO⍴1
TRUST ← ∧/STACK   ⍝ ${coherent ? 'TRUE' : 'FALSE'}
CODE  ← +/STACK   ⍝ ${repos.length}
Ω     ← TRUST∧CODE
\`\`\`

\`\`\`prolog
coherent(system) :-
    entropy(E), E < 0.21,     % E = ${E.toFixed(4)} → ${E < 0.21 ? 'PASS' : 'FAIL'}
    intercoil(_, memory_graph),% ${icoilMem.length} connected → ${icoilMem.length > 0 ? 'PASS' : 'FAIL'}
    intercoil(_, bifrost_engine).% ${icoilBif.length} connected → ${icoilBif.length > 0 ? 'PASS' : 'FAIL'}

meta_block(${omegaValid ? 'valid' : 'degraded'}).
\`\`\`

> ☉ Source → 🧠 Graph → ⚙️ Agents → 🔐 Constraints → 🌈 Execution → 🏛️ Reality

*Field auto-updates every 6 hours via [omega-field.mjs](./omega-field.mjs)*

</div>

<!--OMEGA-FIELD:END-->
`.trim()

  // Patch README.md
  const readme = readFileSync('README.md', 'utf8')
  let patched
  if (readme.includes('<!--OMEGA-FIELD:START-->')) {
    patched = readme.replace(
      /<!--OMEGA-FIELD:START-->[\s\S]*?<!--OMEGA-FIELD:END-->/,
      section
    )
  } else {
    // Inject after first <div align="center"> block
    patched = section + '\n\n' + readme
  }
  writeFileSync('README.md', patched)

  console.log(`✓ repos: ${repos.length} | entropy: ${E.toFixed(4)} | omega: ${omegaValid ? 'VALID' : 'DEGRADED'}`)
  console.log(`✓ seal: ${seal}`)
  console.log(`✓ README.md patched`)
}

main().catch(e => { console.error(e); process.exit(1) })
