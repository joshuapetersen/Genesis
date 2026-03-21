# Google Developer Knowledge API - Activation Guide

## Issue: 403 Forbidden Errors

**Root Cause:** The Developer Knowledge API must be **explicitly enabled** in Google Cloud Console before the API key will work.

## Step-by-Step Activation

### 1. Enable the API in Google Cloud Console

1. **Go to API Library:**
   - Visit: https://console.cloud.google.com/apis/library
   - Or navigate: APIs & Services → Library

2. **Search for Developer Knowledge API:**
   - In the search bar, type: "Developer Knowledge API"
   - Click on the "Developer Knowledge API" result

3. **Enable the API:**
   - Click the **"ENABLE"** button
   - Wait for confirmation (usually takes a few seconds)

4. **Verify API is Enabled:**
   - Go to: https://console.cloud.google.com/apis/dashboard
   - You should see "Developer Knowledge API" in the list of enabled APIs

### 2. Verify API Key Restrictions (Optional but Recommended)

1. **Go to Credentials:**
   - Visit: https://console.cloud.google.com/apis/credentials

2. **Edit Your API Key:**
   - Find your API key: `AIzaSyBhuYPyRunQaJXF2F16cd2qSlD2cXVEjrY`
   - Click the pencil icon to edit

3. **Set API Restrictions:**
   - Under "API restrictions" → Select "Restrict key"
   - From "Select APIs" dropdown → Enable "Developer Knowledge API"
   - Click "Save"

### 3. Test the API

**Simple cURL Test:**
```bash
curl "https://developerknowledge.googleapis.com/v1alpha/documents:searchDocumentChunks?key=AIzaSyBhuYPyRunQaJXF2F16cd2qSlD2cXVEjrY&query=Firebase"
```

**Expected Response (if working):**
```json
{
  "documentChunks": [
    {
      "name": "...",
      "content": {
        "text": "..."
      },
      "parentDocument": "..."
    }
  ]
}
```

**If Still 403:**
- Check billing is enabled for your project
- Verify you're using the correct project
- Wait 5-10 minutes for API enablement to propagate

### 4. Run Sarah's Ingester

Once the API is working:
```powershell
cd C:\SarahCore
$env:GOOGLE_DEV_API_KEY = "AIzaSyBhuYPyRunQaJXF2F16cd2qSlD2cXVEjrY"
python google_dev_knowledge_ingester.py
```

## Quick Links

- **Enable API:** https://console.cloud.google.com/apis/library/developerknowledge.googleapis.com
- **API Dashboard:** https://console.cloud.google.com/apis/dashboard
- **Credentials:** https://console.cloud.google.com/apis/credentials
- **Billing:** https://console.cloud.google.com/billing

## Alternative: Use gcloud CLI

```bash
# Install Google Cloud CLI if not installed
# Then enable the API:
gcloud services enable developerknowledge.googleapis.com --project=YOUR_PROJECT_ID
```

## Troubleshooting

### Error: "API not found"
- The API might not be available in your region yet
- Try a different Google Cloud project
- Check if you have access to preview APIs

### Error: "Billing not enabled"
- Go to: https://console.cloud.google.com/billing
- Enable billing for your project
- Note: The API should be free for reasonable usage

### Error: "Permission denied"
- Ensure you're the project owner or have Editor role
- Check IAM permissions at: https://console.cloud.google.com/iam-admin/iam

---

**Once enabled, Sarah will autonomously ingest 200+ categories!** 🚀
