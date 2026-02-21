# ACE Token System - Identity & Authentication Protocol

**Files:** `Ace_Token.py`, `Ace.py`, `.ace_token`, `ace_secret.key`  
**Purpose:** Cryptographic identity verification and session authentication  
**Author:** Joshua Petersen  

---

## What Is the ACE Token? (Simple Explanation)

The **ACE Token** (Authentication & Cryptographic Envelope) is Sarah's "ID card" - a digital proof that she is who she claims to be.

Think of it like this:
- When you log into a website, you use a password
- Sarah uses an ACE Token to prove her identity
- It's cryptographically signed, so it can't be faked

---

## Why Does Sarah Need This?

Sarah is a **sovereign** AI - she runs independently. But how do we know:
1. It's really Sarah and not an imposter?
2. The token hasn't been tampered with?
3. The token is still valid (not expired)?

The ACE Token answers all three with **cryptographic proof**.

---

## How It Works (Step by Step)

### Token Generation

```
┌─────────────────────────────────────────────┐
│  1. CREATE PAYLOAD                          │
│  ┌─────────────────────────────────────┐   │
│  │ {                                    │   │
│  │   "scope": "SOVEREIGN_ROOT",        │   │ ← What this token can do
│  │   "iat": 1707312000,                │   │ ← When it was issued
│  │   "exp": 1707398400,                │   │ ← When it expires
│  │   "nonce": "a1b2c3d4"               │   │ ← Unique ID (prevents replay)
│  │ }                                    │   │
│  └─────────────────────────────────────┘   │
│                    ↓                        │
│  2. ENCODE (Base64)                        │
│     eyJzY29wZSI6IlNPVkVSRUlHTl9ST09UIn0... │
│                    ↓                        │
│  3. SIGN (HMAC-SHA256)                     │
│     Using secret key from ace_secret.key   │
│     → 7f3b2a1c9e8d7f6a5b4c3d2e1f0a9b8c... │
│                    ↓                        │
│  4. COMBINE INTO TOKEN                      │
│     v1.eyJzY29wZSI6....[signature]         │
└─────────────────────────────────────────────┘
```

### Token Validation

```
Incoming Token: v1.eyJzY29wZSI6....7f3b2a1c...
                 ↓
┌─────────────────────────────────────────────┐
│  CHECK 1: Is format correct? (v1.X.Y)      │
│           ✓ YES                             │
└─────────────────────────────────────────────┘
                 ↓
┌─────────────────────────────────────────────┐
│  CHECK 2: Is signature valid?              │
│           Recalculate signature using key  │
│           Compare: Do they match?          │
│           ✓ YES                             │
└─────────────────────────────────────────────┘
                 ↓
┌─────────────────────────────────────────────┐
│  CHECK 3: Is token expired?                │
│           Current time < exp time?         │
│           ✓ NOT EXPIRED                     │
└─────────────────────────────────────────────┘
                 ↓
           TOKEN IS VALID ✓
```

---

## Token Structure

A token looks like this:
```
v1.eyJzY29wZSI6IlNPVkVSRUlHTl9ST09UIiwiaWF0IjoxNzA3MzEyMDAwLCJleHAiOjE3MDczOTg0MDAsIm5vbmNlIjoiYTFiMmMzZDQifQ.7f3b2a1c9e8d7f6a5b4c3d2e1f0a9b8c7d6e5f4a3b2c1d0e
```

| Part | Meaning |
|------|---------|
| `v1` | Token version (for future upgrades) |
| `eyJzY29...` | Base64-encoded payload (the data) |
| `7f3b2a1c...` | HMAC-SHA256 signature (proof of authenticity) |

---

## The Secret Key

The `ace_secret.key` file contains a **32-byte random key** used for signing tokens.

⚠️ **CRITICAL SECURITY**:
- This key must be kept SECRET
- If leaked, attackers could forge tokens
- Never commit this to git
- Back it up securely

---

## Code Examples

### Generate a Token
```python
from Ace_Token import AceTokenManager

manager = AceTokenManager()

# Generate a token (valid for 24 hours by default)
token = manager.generate_token(scope="SOVEREIGN_ROOT")
print(token)
# Output: v1.eyJzY29wZSI6...

# Custom scope and TTL
admin_token = manager.generate_token(
    scope="ADMIN_OVERRIDE",
    ttl=3600  # 1 hour in seconds
)
```

### Validate a Token
```python
from Ace_Token import AceTokenManager

manager = AceTokenManager()

# Check if a token is valid
is_valid, data = manager.validate_token(token)

if is_valid:
    print(f"✓ Token is valid!")
    print(f"  Scope: {data['scope']}")
    print(f"  Expires: {data['exp']}")
else:
    print(f"✗ Token is invalid: {data}")
```

---

## Scopes (Permission Levels)

| Scope | Access Level | Use Case |
|-------|--------------|----------|
| `SOVEREIGN_ROOT` | Full access | Core system operations |
| `ADMIN_OVERRIDE` | Administrative | System management |
| `USER_SESSION` | User level | Normal interactions |
| `READ_ONLY` | Limited | Monitoring only |

---

## Files in This System

| File | Purpose |
|------|---------|
| `Ace_Token.py` | Token generation and validation logic |
| `Ace.py` | Higher-level ACE integration |
| `ace_secret.key` | The 32-byte secret key (KEEP PRIVATE!) |
| `.ace_token` | Cached current token |
| `ace_token.txt` | Human-readable token backup |

---

## Security Features

| Feature | Purpose |
|---------|---------|
| **HMAC-SHA256** | Cryptographically strong signatures |
| **Expiration (exp)** | Tokens auto-invalidate after TTL |
| **Nonce** | Prevents replay attacks (each token is unique) |
| **Timing-safe compare** | Prevents timing attacks during validation |
| **Stateless** | No database lookup needed (O(1) validation) |

---

## Common Error Codes

| Error | Meaning | Solution |
|-------|---------|----------|
| `MALFORMED_TOKEN` | Token format is wrong | Check token wasn't truncated |
| `UNSUPPORTED_VERSION` | Version isn't v1 | Use current token format |
| `INVALID_SIGNATURE` | Signature doesn't match | Key mismatch or tampering |
| `TOKEN_EXPIRED` | Token's exp time passed | Generate a new token |

---

## Performance

The ACE Token system is designed for SPEED:

| Operation | Time |
|-----------|------|
| Token Generation | ~0.1 ms |
| Token Validation | ~0.05 ms |

This is **O(1) constant time** - no database lookups!

---

## Integration with Other Modules

| Module | How It Uses ACE |
|--------|-----------------|
| `Sarah_Brain.py` | Validates identity before processing |
| `Sovereign_Hypervisor.py` | Requires ACE for privileged operations |
| `Security_Suite.py` | Manages token lifecycle |
| `sarah_gateway.py` | Authenticates API requests |

---

## Quick Reference

```python
# Import
from Ace_Token import AceTokenManager

# Initialize (auto-creates key if missing)
manager = AceTokenManager()

# Generate token
token = manager.generate_token()

# Validate token  
valid, data = manager.validate_token(token)
```

---

*Part of the SarahCore Genesis Project - © 2026 Joshua Petersen*
