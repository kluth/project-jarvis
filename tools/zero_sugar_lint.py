import sys
import json
import re

def check_zero_sugar(file_path):
    with open(file_path, 'r') as f:
        lines = f.readlines()
    
    violations = []
    
    # Pre-process: Strip comments for grammar check
    clean_content = ""
    for line in lines:
        if '//' in line:
            clean_content += line.split('//')[0] + "\n"
        else:
            clean_content += line
    
    # Rule 1: Syntax minimalism
    # Rule 2: No 'clever' increments like 'i++' in code
    if '++' in clean_content or '--' in clean_content:
        violations.append("Implicit increment/decrement detected in executable code (use explicit assignment for Agent-First regularity)")

    # Rule 3: Check for missing 'verify' blocks for functions (Mandate 1.2)
    # Re-calculating on clean content
    functions = re.findall(r'func\s+(\w+)\s*\(', clean_content)
    verifies = re.findall(r'verify\s*\{', clean_content)
    if len(functions) > len(verifies):
         violations.append(f"eTDD Violation: Found {len(functions)} functions but only {len(verifies)} verify blocks.")

    return violations

if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python3 zero_sugar_lint.py <file.jrv>")
        sys.exit(1)
    
    file_to_check = sys.argv[1]
    errors = check_zero_sugar(file_to_check)
    
    if errors:
        print(json.dumps({"status": "VIOLATION", "errors": errors}, indent=2))
        sys.exit(1)
    else:
        print(json.dumps({"status": "CLEAN"}, indent=2))
