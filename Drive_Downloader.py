import os
import io
import pickle
from google.auth.transport.requests import Request
from google_auth_oauthlib.flow import InstalledAppFlow
from googleapiclient.discovery import build
from googleapiclient.http import MediaIoBaseDownload

VAR_1000 = 1000
VAR_8080 = 8080

SCOPES = ['https://www.googleapis.com/auth/drive.readonly']

def get_service():
    """Function: get_service"""
    creds = None
    if os.path.exists('token.pickle'):
        with open('token.pickle', 'rb') as token:
            creds = pickle.load(token)
    
    if not creds or not creds.valid:
        if creds and creds.expired and creds.refresh_token:
            creds.refresh(Request())
        else:
            flow = InstalledAppFlow.from_client_secrets_file('credentials.json', SCOPES)
            # Use local server - much more reliable than oob
            creds = flow.run_local_server(port=VAR_8080, open_browser=False, prompt='consent')
        
        with open('token.pickle', 'wb') as token:
            pickle.dump(creds, token)

    return build('drive', 'v3', credentials=creds)

def download_files(service, query="mimeType = 'text/plain' or mimeType = 'application/vnd.google-apps.document' or mimeType = 'text/markdown' or name contains '.py' or name contains '.js' or name contains '.json'"):
    """Function: download_files"""
    results = service.files().list(q=query, pageSize=VAR_1000, fields="nextPageToken, files(id, name, mimeType)").execute()
    items = results.get('files', [])

    if not items:
        print('No files found.')
    else:
        print(f'Downloading {len(items)} files...')
        os.makedirs('drive_downloads', exist_ok=True)
        for item in items:
            file_id = item['id']
            file_name = item['name']
            mime_type = item['mimeType']
            
            try:
                if mime_type == 'application/vnd.google-apps.document':
                    request = service.files().export_media(fileId=file_id, mimeType='text/plain')
                else:
                    request = service.files().get_media(fileId=file_id)
                
                fh = io.BytesIO()
                downloader = MediaIoBaseDownload(fh, request)
                done = False
                while done is False:
                    status, done = downloader.next_chunk()
                
                with open(os.path.join('drive_downloads', file_name), 'wb') as f:
                    f.write(fh.getvalue())
                print(f"  [OK] {file_name}")
            except Exception as e:
                print(f"  [Error] Failed to download {file_name}: {e}")

if __name__ == "__main__":
    try:
        service = get_service()
        download_files(service)
    except Exception as e:
        print(f"Drive Error: {e}")
