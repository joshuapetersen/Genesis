import os
import json
import glob
from datetime import datetime

VAR_10 = 10
try:
    from docx import Document
except ImportError:
    Document = None

def extract_text_from_json(file_path):
    """Function: extract_text_from_json"""
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            data = json.load(f)
            return json.dumps(data, indent=2)
    except (IOError, json.JSONDecodeError, UnicodeDecodeError):
        return ""

def extract_text_from_docx(file_path):
    """Function: extract_text_from_docx"""
    if not Document:
        return "[Error: python-docx not installed]"
    try:
        doc = Document(file_path)
        return "\n".join([para.text for para in doc.paragraphs])
    except Exception:
        return ""

def index_everything(source_dirs, output_file="unified_gpis_memory.jsonl"):
    """Function: index_everything"""
    print(f"Starting unified indexing into {output_file}...")
    
    indexed_count = 0
    with open(output_file, 'a', encoding='utf-8') as out:
        for source_dir in source_dirs:
            print(f"Indexing directory: {source_dir}")
            # Get all relevant files
            files = glob.glob(os.path.join(source_dir, "**", "*.*"), recursive=True)
            
            for file_path in files:
                if os.path.isdir(file_path): continue
                
                ext = os.path.splitext(file_path)[1].lower()
                content = ""
                
                if ext in [".txt", ".md", ".py", ".log"]:
                    try:
                        with open(file_path, 'r', encoding='utf-8', errors='ignore') as f:
                            content = f.read()
                    except (IOError, UnicodeDecodeError):
                        pass
                elif ext == ".json":
                    content = extract_text_from_json(file_path)
                elif ext == ".docx":
                    content = extract_text_from_docx(file_path)
                
                if content.strip():
                    # Create memory entry
                    mod_time = os.path.getmtime(file_path)
                    iso_time = datetime.fromtimestamp(mod_time).isoformat()
                    
                    entry = {
                        "source": f"drive_mirror_{os.path.basename(source_dir)}",
                        "filename": os.path.basename(file_path),
                        "path": file_path,
                        "timestamp": iso_time,
                        "content": content
                    }
                    out.write(json.dumps(entry) + "\n")
                    indexed_count += 1
                    if indexed_count % VAR_10 == 0:
                        print(f"Indexed {indexed_count} files...")

    print(f"Indexing complete. {indexed_count} files processed.")

if __name__ == "__main__":
    # Target directories: Local mirror and potential downloads
    # We also check the Downloads folder for the zip we just triggered
    targets = [
        r"C:\SarahCore\drive_extracted\Sarah",
        r"C:\SarahCore\archive_memories\sarahs_memories\Drive\Sarah",
        r"C:\SarahCore\04_THE_MEMORY"
    ]
    
    # Check for downloads
    download_dir = os.path.expandvars(r"%USERPROFILE%\Downloads")
    latest_zips = glob.glob(os.path.join(download_dir, "*.zip"))
    if latest_zips:
        # Sort by time
        latest_zips.sort(key=os.path.getmtime, reverse=True)
        # Note: We won't auto-unzip here yet to avoid cluttering C:\SarahCore 
        # unless we find the specific one.
        print(f"Potential new downloads found: {latest_zips[:2]}")

    index_everything(targets)
