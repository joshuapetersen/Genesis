import os

report_path = r"C:\SarahCore\ats_topology_report_v2.md"

with open(report_path, 'r', encoding='utf-8') as f:
    lines = f.readlines()

formatted_lines = []
for i, line in enumerate(lines):
    cleaned = line.strip()
    
    # MD022: Headings padding
    if cleaned.startswith('#'):
        if formatted_lines and formatted_lines[-1] != '\n':
            formatted_lines.append('\n')
        formatted_lines.append(cleaned + '\n')
        if i + 1 < len(lines) and lines[i+1].strip() != '':
            formatted_lines.append('\n')
        continue

    # MD032: Lists padding
    if cleaned.startswith('- '):
        if formatted_lines and not formatted_lines[-1].startswith('- ') and formatted_lines[-1] != '\n':
            formatted_lines.append('\n')
        formatted_lines.append(cleaned + '\n')
        if i + 1 < len(lines) and not lines[i+1].strip().startswith('- ') and lines[i+1].strip() != '':
            formatted_lines.append('\n')
        continue

    # MD058/MD060: Table padding and pipe alignment
    if cleaned.startswith('|'):
        if formatted_lines and not formatted_lines[-1].startswith('|') and formatted_lines[-1] != '\n':
            formatted_lines.append('\n')
        
        # MD060: Compact column style (ensure no extra spaces around pipes if it's supposed to be compact,
        # but usually 'compact' means |Col|Col|, not | Col | Col |)
        # Let's just fix the spacing to be standard and consistent.
        parts = [p.strip() for p in cleaned.split('|')]
        # Filter empty strings from splits if they are at start/end
        if parts[0] == '': parts = parts[1:]
        if parts[-1] == '': parts = parts[:-1]
        
        # MD050: Strong style (convert __ to **)
        fixed_parts = []
        for p in parts:
            p = p.replace('__', '**')
            fixed_parts.append(p)
            
        new_line = '| ' + ' | '.join(fixed_parts) + ' |\n'
        formatted_lines.append(new_line)
        
        if i + 1 < len(lines) and not lines[i+1].strip().startswith('|') and lines[i+1].strip() != '':
            formatted_lines.append('\n')
        continue

    # Default
    if cleaned == '':
        if formatted_lines and formatted_lines[-1] != '\n':
            formatted_lines.append('\n')
    else:
        formatted_lines.append(cleaned + '\n')

with open(report_path, 'w', encoding='utf-8') as f:
    f.writelines(formatted_lines)

print("Restoration Pulse Complete. Substrate Document Hardened. 宣")
