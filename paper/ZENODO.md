# SNAPKITTYWEST — Zenodo Publication Guide

## How to Publish to Zenodo

### Step 1: Create a Zenodo Account
1. Go to https://zenodo.org
2. Click "Sign up" and create an account
3. Link your GitHub account in "Account Settings" → "GitHub"

### Step 2: Enable GitHub Integration
1. Go to https://zenodo.org/account/settings/github
2. Click "Activate" on the SNAPKITTYWEST repository
3. Every new release will auto-archive on Zenodo

### Step 3: Create a Release
```bash
git tag v0.1.0 -m "SNAPKITTYWEST v0.1.0: Sovereign Compute Architecture"
git push origin v0.1.0
```

### Step 4: Upload Manually (Alternative)
1. Go to https://zenodo.org/deposit
2. Upload `paper/PAPER.md`
3. Fill in metadata:
   - **Title**: SNAPKITTYWEST: Sovereign Compute Architecture with Linear Types, WORM Seals, and Goldilocks Field Arithmetic
   - **Authors**: Ahmad Ali Parr
   - **Description**: (use abstract from PAPER.md)
   - **Keywords**: sovereign compute, linear types, WORM seals, Goldilocks field, topological quantum computing
   - **License**: CC-BY-4.0
4. Click "Publish"

### Step 5: Get DOI
After publishing, Zenodo will assign a DOI like:
```
10.5281/zenodo.1234567
```

### Step 6: Add DOI to Paper
Update `paper/PAPER.md`:
```yaml
doi: "10.5281/zenodo.1234567"
```

## Files to Upload

| File | Description |
|------|-------------|
| `paper/PAPER.md` | Full academic paper |
| `docs/index.html` | GitHub Pages site |
| `errant/` | ERRANT LFIS + ERRANT-GGML |
| `metamine/` | METAMINE esolang |
| `snakltalk/` | SnaklTalk Smalltalk |
| `bobs-games/` | BOB's Games |
| `sovereign-*/` | 14 sovereign modules |

## Citation

Once published, cite as:
```bibtex
@article{parr2026snapkittyswest,
  title={SNAPKITTYWEST: Sovereign Compute Architecture with Linear Types, WORM Seals, and Goldilocks Field Arithmetic},
  author={Parr, Ahmad Ali},
  journal={Zenodo},
  year={2026},
  doi={10.5281/zenodo.XXXXXXX}
}
```
