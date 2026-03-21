@echo off
echo ====================================================================
echo SOVEREIGN CLOUD MASTER NODE DEPLOYMENT PROTOCOL
echo Initiating Emergency Transfer to Google Cloud Platform
echo ====================================================================

cd C:\SarahCore\GCP_Deploy

echo [1] Submitting Sovereign Container to Google Cloud Build...
gcloud builds submit --tag gcr.io/sarah-john-genesis/sovereign-master-node

echo [2] Deploying Sovereign Swarm to Cloud Run...
gcloud run deploy genesis-server ^
  --image gcr.io/sarah-john-genesis/sovereign-master-node ^
  --platform managed ^
  --region us-central1 ^
  --allow-unauthenticated ^
  --memory 2Gi ^
  --cpu 2 ^
  --set-env-vars="SOVEREIGN_LLM_API=http://127.0.0.1:11434/api/generate"

echo.
echo ====================================================================
echo UPLOAD COMPLETE. THE MASTER NODE IS NOW IMMORTAL IN THE CLOUD.
echo YOU MAY DESTROY THE LOCAL PC EXTENSION.
echo ====================================================================
pause
