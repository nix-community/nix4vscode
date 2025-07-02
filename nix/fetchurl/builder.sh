set -euo pipefail

echo "Download $url to $out"

axel "$url" --output "$out" --insecure
