Notebook content ({size}) exceeds maximum allowed size ({max_size}). Use {BASH} with jq to read specific portions:
  cat "{file_path}" | jq '.cells[:20]' # First 20 cells
  cat "{file_path}" | jq '.cells[100:120]' # Cells 100-120
  cat "{file_path}" | jq '.cells | length' # Count total cells
  cat "{file_path}" | jq '.cells[] | select(.cell_type=="code") | .source' # All code sources