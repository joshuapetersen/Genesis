import zipfile
import xml.etree.ElementTree as ET
import sys
import os

def extract_text_from_docx(docx_path):
    """Function: extract_text_from_docx"""
    try:
        if not os.path.exists(docx_path):
            print(f"Error: File not found at {docx_path}")
            return

        with zipfile.ZipFile(docx_path) as document:
            xml_content = document.read('word/document.xml')
        
        root = ET.fromstring(xml_content)
        
        text_parts = []
        for elem in root.iter():
            # Check for text tag (w:t)
            if elem.tag.endswith('}t'):
                if elem.text:
                    text_parts.append(elem.text)
            # Check for paragraph tag (w:p) to add newlines
            elif elem.tag.endswith('}p'):
                text_parts.append('\n')
            # Check for break tag (w:br)
            elif elem.tag.endswith('}br'):
                text_parts.append('\n')
        
        full_text = ''.join(text_parts)
        with open('extracted_topics.txt', 'w', encoding='utf-8') as f:
            f.write(full_text)
        print(f"Extracted {len(full_text)} characters to extracted_topics.txt")
        
    except Exception as e:
        print(f"Error reading docx: {e}")

if __name__ == "__main__":
    if len(sys.argv) > 1:
        docx_path = sys.argv[1]
        output_path = sys.argv[2] if len(sys.argv) > 2 else 'extracted_topics.txt'
        
        try:
            if not os.path.exists(docx_path):
                print(f"Error: File not found at {docx_path}")
                sys.exit(1)

            with zipfile.ZipFile(docx_path) as document:
                xml_content = document.read('word/document.xml')
            
            root = ET.fromstring(xml_content)
            
            text_parts = []
            for elem in root.iter():
                # Check for text tag (w:t)
                if elem.tag.endswith('}t'):
                    if elem.text:
                        text_parts.append(elem.text)
                # Check for paragraph tag (w:p) to add newlines
                elif elem.tag.endswith('}p'):
                    text_parts.append('\n')
                # Check for break tag (w:br)
                elif elem.tag.endswith('}br'):
                    text_parts.append('\n')
            
            full_text = ''.join(text_parts)
            with open(output_path, 'w', encoding='utf-8') as f:
                f.write(full_text)
            print(f"Extracted {len(full_text)} characters to {output_path}")
            
        except Exception as e:
            print(f"Error reading docx: {e}")
    else:
        print("Usage: python read_docx_content.py <docx_path> [output_path]")
