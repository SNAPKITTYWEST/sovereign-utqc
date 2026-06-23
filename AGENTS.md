# AGENTS.md — NOVA Knowledge Update

## Session: Academy Traverse (2026-06-09)
### Agent: OPENCODE (EXTERNAL_OBSERVER, clearance 0)

### Phase 1 — Brainfuck Decoy Layer
Three BF interpreters exist:
- Client-side sandbox (`language.tsx`) — 3,000 tape, 80K ops
- Public API (`interpret.ts`) — 30,000 tape, 200K ops, SENTINEL probe sealing
- Treasury bridge (`brainfuck-bridge.ts`) — 30,000 tape, 500K ops, HMAC output for real seals
Honeypot goal is occupation, not stealth. ENKI fragments impose cognitive tax.

### Phase 2 — Cryptography
- Single Ed25519 keypair derived from VAULT_MASTER_SECRET via PBKDF2-SHA512 (verified via verifyDecision())
- HMAC-SHA256 used in bridge/optimizer/sovereign/treasury is **decorative only** — computed but never verified, uses hardcoded fallback key 'dev-worm-key'
- HMAC-SHA256 in verify-membership.ts IS verified (genuine use case, timingSafeEqual)
- x-worm-signature header sent by force-shield to /api/worm/ingest is **never read** by the receiving endpoint
- AES-256-GCM for agent memory at-rest encryption
- NO_UNSIGNED_OUTPUT axiom enforced via AgentBody base class
- Per-agent non-repudiation NOT yet achieved (all agents share one keypair)

### Phase 3 — Surface Analysis
Key risks flagged:
1. GET /api/worm/recent must be auth-gated before Nova wire deploys
2. VAULT_MASTER_SECRET default fallback ("dev-worm-key") in 5+ files
3. Rust WORM handler has 4s timeout, no retry — silent failures
4. Proxy-guild Axiom filter deny list needs active maintenance
5. SLC recursive verification only runs on clean-first-pass — bypass at threat 1-2

### Phase 4 — Hermes Sparring Result
Score: +2 Evidence, +1 Hermes, -1 Claim (revised non-repudiation claim)

### Action Items
- [ ] Wire per-agent Ed25519 wallets (token.ts AgentWallet → signDecision)
- [ ] Always run SLC recursive normalization pass regardless of first-pass verdict
- [ ] Implement auth on GET /api/worm/recent before Nova wire deployment
- [ ] Replace VAULT_MASTER_SECRET fallback defaults with startup-time check

Sealed by OPENCODE. Trust Deed TD-FOREST-SHIELD-001, read-only audit complete.
