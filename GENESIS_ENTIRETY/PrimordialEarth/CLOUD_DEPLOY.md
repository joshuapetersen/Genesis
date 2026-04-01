# Genesis Cloud Deployment Guide

## What was set up
- `cloud_server.py` - Watchdog that keeps the simulation alive 24/7, auto-restarts on crash
- `Dockerfile` - Containerizes the simulation for any cloud
- `railway.toml` - One-click deployment to Railway.app (free tier available)

## Option A: Railway.app (Easiest — Free Tier)

1. Go to [railway.app](https://railway.app) and sign in with GitHub
2. Click **New Project → Deploy from GitHub Repo**
3. Push your `C:\PrimordialEarth` folder to a GitHub repo first:
   ```
   cd C:\PrimordialEarth
   git init
   git add Genesis_Societal_Ecology.py cloud_server.py Dockerfile railway.toml requirements.txt
   git commit -m "Genesis World Server"
   git remote add origin <your-github-repo-url>
   git push -u origin main
   ```
4. Railway auto-detects the Dockerfile and deploys
5. The simulation runs 24/7. Your PC is just a viewer.

> **Note on the SQLite database**: SQLite doesn't persist between Railway redeploys unless you attach a volume. For persistent world state, Railway → Settings → Add Volume → mount at `/data`.

## Option B: Any Linux VPS (DigitalOcean, Linode, Oracle Free Tier)

```bash
# On the server:
git clone <your-repo>
cd PrimordialEarth
pip install -r requirements.txt
nohup python cloud_server.py > genesis.log 2>&1 &
```

The `&` runs it in the background. `nohup` keeps it alive after you disconnect.

## Your PC as interaction point
Once deployed, you query the cloud server's SQLite DB via:
- SSH tunnel to query the live DB
- Or a thin API layer (we can add this next) that exposes `/status`, `/pantheon`, `/mind-audit` endpoints your PC calls

## Current Pantheon (Written to Cloud)
| Name | Mandate |
|:--|:--|
| **Bal** | *"I was here before the Light named itself."* |
| **Carmina Tenebris** | *"The Architect cannot take this. I am Accord."* |
| **Devourress of Arcana** | *"We shall not forget."* |
