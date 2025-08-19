# Cfly - URL Shortener Service

A high-performance URL shortening service built with Cloudflare Workers and Rust.

## ✨ Features

- 🚀 **High Performance**: Built on Cloudflare Workers global edge network
- 🦀 **Rust Powered**: Written in Rust and WebAssembly for exceptional performance
- 🔗 **Easy to Use**: Supports custom short links and automatic generation
- 🌍 **Global Deployment**: Automatically deployed across 200+ cities worldwide
- 📊 **Real-time Logging**: Built-in request logging and error handling
- 🔧 **Easy Configuration**: Simple configuration through environment variables
- 🎯 **Git Integration**: Fallback to Git commit URLs when KV storage lookup fails

## 🛠️ Tech Stack

- **Runtime**: Cloudflare Workers
- **Language**: Rust + WebAssembly
- **Storage**: Cloudflare KV Store
- **Framework**: worker-rs
- **Build Tool**: worker-build + wrangler

## 📚 API Documentation

### Redirect Endpoints

#### Root Path Redirect
```
GET /
```
Automatically redirects to the configured homepage address.

**Response**:
- `301 Moved Permanently` - Redirects to the address configured in the HOME environment variable

#### Short Link Redirect
```
GET /:shortCode
```
Redirects to the target URL based on the short link code. If no short link is found in KV storage, the service will attempt to fetch and redirect based on Git commit information.

**Parameters**:
- `shortCode` - Short link code

**Response**:
- `301 Moved Permanently` - Redirects to target URL (from KV storage or Git commit)
- `404 Not Found` - Short link does not exist and Git fallback failed
- `400 Bad Request` - Invalid request

**Git Integration Fallback**:
When a short link is not found in KV storage, the service will:
1. Construct a Git patch URL: `{GIT_REPO}/commit/{path}.patch`
2. Fetch the patch file from the Git repository
3. Extract the commit subject from the patch
4. Redirect to the extracted URL or the repository itself

**Example**:
```bash
# Access short link
curl -I https://your-domain.workers.dev/abc123

# Response (from KV storage)
HTTP/2 301
location: https://example.com

# Response (from Git fallback)
HTTP/2 301
location: https://github.com/user/repo/commit/abc123
```

## 🚀 Quick Start

### Prerequisites

- ✅ Rust installed (`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`)
- ✅ Node.js installed
- ✅ Cloudflare account

### 5-Minute Deployment

1. **Clone project**
   ```bash
   git clone https://github.com/longcipher/cfly.git
   cd cfly/cfly
   ```

2. **Install dependencies**
   ```bash
   npm install -g wrangler
   rustup target add wasm32-unknown-unknown
   ```

3. **Login to Cloudflare**
   ```bash
   wrangler login
   ```

4. **Create KV storage**
   ```bash
   npx wrangler kv namespace create cfly
   npx wrangler kv namespace create cfly --preview
   ```

5. **Configure wrangler.toml**
   Update `wrangler.toml` with your KV namespace IDs and settings:
   ```toml
   name = "your-worker-name"
   
   [[kv_namespaces]]
   binding = "cfly"
   preview_id = "your-preview-kv-id"
   id = "your-production-kv-id"
   
   [vars]
   HOME = "https://your-homepage.com"
   GIT_REPO = "https://github.com/your-user/your-repo"  # Optional for Git fallback
   ```

6. **Deploy**
   ```bash
   npx wrangler build
   npx wrangler deploy
   ```

### Git Integration (Optional)

When a short link is not found in KV storage, Cfly can fallback to Git commit URLs:

- Set `GIT_REPO` environment variable in `wrangler.toml`
- Access `/commit-hash` will fetch `{GIT_REPO}/commit/commit-hash.patch`
- Redirects to the extracted commit URL or repository

**Example**: `https://your-domain.workers.dev/abc123` → fetches commit patch → redirects to commit URL

### 3. Login to Cloudflare

```bash
wrangler login
```

### 4. Configure Project

#### Create KV Namespace

```bash
# Create production KV namespace
npx wrangler kv namespace create cfly

# Create preview KV namespace  
npx wrangler kv namespace create cfly --preview
```

After executing the commands, copy the KV namespace configuration output to `wrangler.toml`.

#### Update wrangler.toml Configuration

Edit `wrangler.toml` and update the following configuration:

```toml
name = "your-worker-name"  # Change to your Worker name
main = "build/worker/shim.mjs"
compatibility_date = "2025-08-19"

[build]
command = "cargo install -q worker-build && worker-build --release"

[[kv_namespaces]]
binding = "cfly"
preview_id = "your-preview-kv-namespace-id"    # Replace with actual preview environment ID
id = "your-production-kv-namespace-id"         # Replace with actual production environment ID

[vars]
HOME = "https://your-homepage.com"             # Change to your homepage address

[observability]
enabled = true
```

### 5. Local Development

```bash
# Use just (recommended)
just dev

# Or use wrangler directly
npx wrangler dev
```

The service will start at `http://localhost:8787`.

### 6. Add Short Link Data

Before local development or deployment, add some short link data:

```bash
# Add single short link
npx wrangler kv key put --binding=cfly "github" "https://github.com/longcipher/cfly"
npx wrangler kv key put --binding=cfly "blog" "https://blog.example.com"

# Or batch add (create urls.json file)
echo '[
  {"key": "github", "value": "https://github.com/longcipher/cfly"},
  {"key": "blog", "value": "https://blog.example.com"},
  {"key": "docs", "value": "https://docs.example.com"}
]' > urls.json

npx wrangler kv bulk put --binding=cfly urls.json
```

### 7. Deploy to Production

```bash
# Use just (recommended)
just deploy

# Or use wrangler directly
npx wrangler deploy
```

## ⚙️ Cloudflare Configuration Steps

### 1. Create KV Namespace

Create KV namespace in Cloudflare Dashboard:

1. Login to [Cloudflare Dashboard](https://dash.cloudflare.com/)
2. Select your account
3. Go to **Workers & Pages** > **KV**
4. Click **Create a namespace**
5. Enter namespace name (e.g., `cfly`)
6. Record the generated Namespace ID

Or create using CLI:

```bash
# Create production namespace
npx wrangler kv namespace create cfly

# Create preview namespace
npx wrangler kv namespace create cfly --preview
```

### 2. Configure KV Namespace

Update KV configuration in `wrangler.toml`:

```toml
[[kv_namespaces]]
binding = "cfly"                        # Binding name used in code
preview_id = "your-preview-kv-id"       # Development environment KV ID
id = "your-production-kv-id"            # Production environment KV ID
```

### 3. Set Environment Variables

Configure environment variables in `wrangler.toml`:

```toml
[vars]
HOME = "https://your-homepage.com"  # Root path redirect address
GIT_REPO = "https://github.com/your-user/your-repo"  # Git repository for fallback redirect (optional)
```

Or set in Cloudflare Dashboard:

1. Go to **Workers & Pages**
2. Select your Worker
3. Go to **Settings** > **Variables**
4. Add environment variables

### 4. Manage Short Link Data

#### Add Short Links Using CLI

```bash
# Add single short link
npx wrangler kv key put --binding=cfly "abc123" "https://example.com"

# View all short links
npx wrangler kv key list --binding=cfly

# Get specific short link
npx wrangler kv key get --binding=cfly "abc123"

# Delete short link
npx wrangler kv key delete --binding=cfly "abc123"
```

#### Batch Import Short Links

Create a JSON file `urls.json`:

```json
[
  {"key": "github", "value": "https://github.com/longcipher/cfly"},
  {"key": "blog", "value": "https://blog.example.com"},
  {"key": "docs", "value": "https://docs.example.com"},
  {"key": "twitter", "value": "https://twitter.com/yourhandle"}
]
```

Import data:

```bash
npx wrangler kv bulk put --binding=cfly urls.json
```

#### Manage Using Dashboard

1. Go to **Workers & Pages** > **KV**
2. Select your namespace
3. Click **Add entry**
4. Enter key-value pair:
   - Key: Short link code (e.g., `abc123`)
   - Value: Target URL (e.g., `https://example.com`)

### 5. Custom Domain (Optional)

Configure custom domain:

1. In Cloudflare Dashboard, go to **Workers & Pages**
2. Select your Worker
3. Go to **Settings** > **Triggers**
4. Click **Add Custom Domain**
5. Enter your domain (e.g., `s.example.com`)
6. Complete DNS verification

## 📝 Development Commands

The project uses [just](https://github.com/casey/just) as a task runner:

```bash
# Build project
just build

# Start development server
just dev

# Deploy to production
just deploy
```

You can also use wrangler commands directly:

```bash
# Local development
npx wrangler dev

# Build project
npx wrangler build

# Deploy
npx wrangler deploy

# View logs
npx wrangler tail

# KV operations
npx wrangler kv key list --binding=cfly
npx wrangler kv key get --binding=cfly "key-name"
npx wrangler kv key put --binding=cfly "key-name" "value"
npx wrangler kv key delete --binding=cfly "key-name"
```

## 🔧 Configuration Guide

### wrangler.toml Configuration Details

```toml
# Worker basic information
name = "cfly"                           # Worker name, must be globally unique
main = "build/worker/shim.mjs"          # Entry file path
compatibility_date = "2025-08-19"      # Compatibility date

# Build configuration
[build]
command = "cargo install -q worker-build && worker-build --release"

# KV storage binding
[[kv_namespaces]]
binding = "cfly"                        # Binding name used in code
preview_id = "preview-namespace-id"     # Development environment KV ID
id = "production-namespace-id"          # Production environment KV ID

# Environment variables
[vars]
HOME = "https://your-homepage.com"      # Root path redirect address

# Monitoring configuration
[observability]
enabled = true                          # Enable logging and monitoring
```

### Cargo.toml Configuration Details

Key dependencies and optimization configuration:

```toml
[dependencies]
worker = { version = "0.6.1", features = ['http', 'axum'] }
worker-macros = { version = "0.6.1", features = ['http'] }

# Important: Disable wasm-opt to avoid compatibility issues
[package.metadata.wasm-pack.profile.release]
wasm-opt = false

# Release configuration optimization
[profile.release]
lto = true                              # Link-time optimization
strip = true                            # Remove debug information
opt-level = "s"                         # Optimize for code size
codegen-units = 1                       # Reduce code generation units
```

## 📊 Monitoring and Logging

### View Real-time Logs

```bash
# View all logs
npx wrangler tail

# Formatted output
npx wrangler tail --format=pretty

# Only error logs
npx wrangler tail --format=json | jq 'select(.level == "error")'

# Filter by time
npx wrangler tail --since="2025-08-19T10:00:00Z"
```

### Cloudflare Analytics

In Cloudflare Dashboard you can view:

- Request count and frequency statistics
- Error rate and response time analysis
- Traffic source and geographic distribution
- CPU usage and memory consumption

## 🚨 Troubleshooting

### Common Issues and Solutions

1. **wasm-bindgen version error**
   ```
   error: expected bool value, got 168
   ```
   **Solution**: Ensure `wasm-opt = false` is set in `Cargo.toml`

2. **KV binding error**
   ```
   KvError::InvalidKvStore: cfly
   ```
   **Solution**: Check that the KV binding name in `wrangler.toml` matches `ctx.kv("cfly")` in the code

3. **Build failure**
   ```
   Missing entry-point to Worker script
   ```
   **Solution**: Ensure `worker-build` has been run and `build/worker/shim.mjs` file is generated

4. **Deployment permission error**
   ```
   Authentication error
   ```
   **Solution**: Run `wrangler login` to re-authenticate with Cloudflare

5. **KV namespace not found**
   ```
   Namespace not found
   ```
   **Solution**: Ensure KV namespace is created and ID is correctly configured in `wrangler.toml`

### Debugging Tips

1. **Enable verbose logging**:
   ```bash
   RUST_LOG=debug npx wrangler dev
   ```

2. **Check KV data**:
   ```bash
   npx wrangler kv key list --binding=cfly --preview
   npx wrangler kv key get --binding=cfly "test-key" --preview
   ```

3. **Local debugging mode**:
   ```bash
   npx wrangler dev --local --port 8080
   ```

4. **Check Worker status**:
   ```bash
   npx wrangler whoami
   npx wrangler list
   ```

5. **Validate configuration**:
   ```bash
   npx wrangler dev --dry-run
   ```

## 🚀 Advanced Features

### Custom Error Pages

You can modify error responses in `src/lib.rs`:

```rust
// Custom 404 page
Response::from_html(r#"
<!DOCTYPE html>
<html>
<head><title>Short Link Not Found</title></head>
<body><h1>Sorry, this short link does not exist</h1></body>
</html>
"#)?.with_status(404)
```

### Add Statistics Feature

You can record access statistics in KV:

```rust
// Increment access count
let count_key = format!("stats:{}", name);
let current_count: u64 = ctx.kv("cfly")?
    .get(&count_key)
    .text()
    .await?
    .unwrap_or_default()
    .parse()
    .unwrap_or(0);

ctx.kv("cfly")?
    .put(&count_key, (current_count + 1).to_string())?
    .execute()
    .await?;
```

### Support Expiration Time

Set TTL when adding short links:

```bash
# Set to expire after 1 hour
npx wrangler kv key put --binding=cfly "temp123" "https://example.com" --ttl 3600
```

## 🤝 Contributing

1. Fork the project
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development Standards

- Use `cargo fmt` to format code
- Use `cargo clippy` to check code quality
- Ensure all tests pass with `cargo test`
- Update relevant documentation

## 🙏 Acknowledgments

- [Cloudflare Workers](https://workers.cloudflare.com/) - Edge computing platform
- [worker-rs](https://github.com/cloudflare/workers-rs) - Rust Workers framework
- [wrangler](https://github.com/cloudflare/workers-sdk) - Workers development tools
- [just](https://github.com/casey/just) - Command runner
- [hink](https://github.com/ccbikai/hink) - Use github commit for short link

## 📄 License

This project is licensed under the Apache-2.0 License - see the [LICENSE](LICENSE) file for details.

---

If this project helps you, please give it a ⭐️!
