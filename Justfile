# Build project
build:
    npx wrangler build

# Start development server
dev:
    npx wrangler dev

# Deploy to production
deploy:
    npx wrangler deploy

# View real-time logs
logs:
    npx wrangler tail

# Format Rust code
fmt:
    cargo fmt

# Code quality check
clippy:
    cargo clippy

# Run tests
test:
    cargo test

# Check code (without building)
check:
    cargo check

# Clean build files
clean:
    rm -rf build/ target/

# Create KV namespace
create-kv:
    npx wrangler kv namespace create cfly
    npx wrangler kv namespace create cfly --preview

# List all short links
list-urls:
    npx wrangler kv key list --binding=cfly --preview false

# List short links by Namespace ID (usage: just list-urls-by-id <namespaceId>)
list-urls-by-id namespaceId:
    npx wrangler kv key list --namespace-id {{namespaceId}}

# Add short link (usage: just add-url key url)
add-url key url:
    npx wrangler kv key put --binding=cfly --preview false "{{key}}" "{{url}}"

# Add short link by Namespace ID (usage: just add-url-by-id <namespaceId> <key> <url>)
add-url-by-id namespaceId key url:
    npx wrangler kv key put --namespace-id {{namespaceId}} "{{key}}" "{{url}}"

# Get short link (usage: just get-url key)
get-url key:
    npx wrangler kv key get --binding=cfly --preview false "{{key}}"

# Get short link by Namespace ID (usage: just get-url-by-id <namespaceId> <key>)
get-url-by-id namespaceId key:
    npx wrangler kv key get --namespace-id {{namespaceId}} "{{key}}"

# Delete short link (usage: just delete-url key)
delete-url key:
    npx wrangler kv key delete --binding=cfly --preview false "{{key}}"

# Delete short link by Namespace ID (usage: just delete-url-by-id <namespaceId> <key>)
delete-url-by-id namespaceId key:
    npx wrangler kv key delete --namespace-id {{namespaceId}} "{{key}}"

# Batch import short links (usage: just import-urls urls.json)
import-urls file:
    npx wrangler kv bulk put --binding=cfly --preview false "{{file}}"

# Batch import short links by Namespace ID (usage: just import-urls-by-id <namespaceId> <file>)
import-urls-by-id namespaceId file:
    npx wrangler kv bulk put --namespace-id {{namespaceId}} "{{file}}"

# Complete development workflow: format -> check -> build -> dev
dev-full: fmt clippy check dev

# Complete deployment workflow: format -> check -> build -> deploy
deploy-full: fmt clippy check build deploy

# Test Git integration fallback (requires dev server running)
test-git path:
    curl -v "http://localhost:8787/{{path}}" 2>&1 | head -20
