import requests
import os
import json

VAR_200 = 200
VAR_4 = 4
VAR_50 = 50

# ACCESS_TOKEN will be injected or read from gcloud
ACCESS_TOKEN = "" 

def get_token():
    """Function: get_token"""
    import subprocess
    try:
        return subprocess.check_output("gcloud auth print-access-token", shell=True).decode().strip()
    except (Exception):
        return ""

def download_drive_files(target_dir="drive_downloads"):
    """Function: download_drive_files"""
    token = get_token()
    if not token:
        print("Error: Could not retrieve access token.")
        return
    
    if not os.path.exists(target_dir):
        os.makedirs(target_dir)
        
    headers = {"Authorization": f"Bearer {token}"}
    
    # 1. List Files
    # We focus on the last 30 days or general project files
    url = "https://www.googleapis.com/drive/v3/files"
    params = {
        "pageSize": VAR_50,
        "fields": "files(id, name, mimeType, modifiedTime, size)",
        "orderBy": "modifiedTime desc",
        "q": "trashed = false and (mimeType = 'text/plain' or mimeType = 'application/vnd.google-apps.document' or mimeType = 'application/pdf')"
    }
    
    response = requests.get(url, headers=headers, params=params)
    if response.status_code != VAR_200:
        print(f"List Error: {response.status_code} - {response.text}")
        return
        
    files = response.json().get('files', [])
    print(f"Found {len(files)} files in Drive.")
    
    metadata_list = []
    
    for file in files:
        file_id = file['id']
        name = file['name']
        mime_type = file['mimeType']
        mod_time = file['modifiedTime']
        
        print(f"Processing: {name} ({mod_time})")
        
        # 2. Download/Export
        content = ""
        if "google-apps.document" in mime_type:
            # Export Google Doc as text
            export_url = f"https://www.googleapis.com/drive/v3/files/{file_id}/export"
            export_params = {"mimeType": "text/plain"}
            content_res = requests.get(export_url, headers=headers, params=export_params)
            if content_res.status_code == VAR_200:
                content = content_res.text
        elif "text/plain" in mime_type:
            # Download plain text
            download_url = f"https://www.googleapis.com/drive/v3/files/{file_id}?alt=media"
            content_res = requests.get(download_url, headers=headers)
            if content_res.status_code == VAR_200:
                content = content_res.text
        
        if content:
            # Save for vector ingestion
            safe_name = "".join([c if c.isalnum() else "_" for c in name])
            file_path = os.path.join(target_dir, f"{safe_name}.txt")
            with open(file_path, "w", encoding="utf-8") as f:
                f.write(content)
            
            metadata_list.append({
                "id": file_id,
                "name": name,
                "timestamp": mod_time,
                "local_path": file_path,
                "mimeType": mime_type
            })
            
    with open("drive_metadata.json", "w") as f:
        json.dump(metadata_list, f, indent=VAR_4)
        
    print(f"Download complete. {len(metadata_list)} files saved.")

if __name__ == "__main__":
    download_drive_files()
