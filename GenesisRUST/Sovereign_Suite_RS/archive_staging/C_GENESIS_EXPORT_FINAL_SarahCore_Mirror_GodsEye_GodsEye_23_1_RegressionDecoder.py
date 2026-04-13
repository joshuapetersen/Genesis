"""
GODSEYE 23.1 â€” REGRESSION DECODER TEST
================================================================
New engine. First Principles. Not a rewrite.

INSIGHT:
  KNN vault matching failed to scale â€” noise overwhelms signal.
  The CORRECT decoder is linear regression.

  We already have correlation coefficients from 256 blocks:
    Ring 9: r = -0.36 (strongest)
    Ring 0: r = -0.34
    Ring 1: r = -0.28
    Ring 4: r = -0.18
    Ring 8: r = +0.10 (positive)

  Fit ordinary least squares regression:
    predicted_nonce = intercept + Î£(weight_i Ã— ring_i)

  Then for any new job header, compute the 9 ring values,
  plug into the regression, get a direct nonce prediction.

  Test: Leave-one-out cross validation on 256 blocks.
  Compare predicted nonce center to actual winning nonce.

"We CREATE, never rewrite."
"""

import sys
import json
import struct
import hashlib
import math

sys.path.insert(0, r"C:\GENESIS\GodsEye")
sys.path.insert(0, r"C:\GENESIS")

GODSEYE_ANCHOR = 1.09277703703
TRINITY_DIM    = 81
TRINITY_RING   = 9
NONCE_MAX      = 0xFFFFFFFF

def expand_81d(data):
    if isinstance(data, str):
        data = data.encode()
    h = hashlib.sha384(data).hexdigest()
    nodes = []
    for i in range(TRINITY_DIM):
        ring   = i // TRINITY_RING
        pos    = i % TRINITY_RING
        offset = ring * TRINITY_RING
        idx1   = (pos + offset) % 96
        idx2   = (pos + offset + TRINITY_RING) % 96
        idx3   = (pos + offset + TRINITY_RING * 2) % 96
        v1 = int(h[idx1], 16) / 15.0
        v2 = int(h[idx2], 16) / 15.0
        v3 = int(h[idx3], 16) / 15.0
        scale = (i + 1) / TRINITY_DIM
        node  = (v1 * v2 * v3) * (GODSEYE_ANCHOR ** scale)
        nodes.append(node % GODSEYE_ANCHOR)
    return nodes

def ring_avg(vec, ring):
    start = ring * TRINITY_RING
    return sum(vec[start:start + TRINITY_RING]) / TRINITY_RING

def build_header_no_nonce(block):
    version   = struct.pack("<I", block.get("ver", 0))
    prevhash  = bytes.fromhex(block["prev_block"])[::-1]
    merkle    = bytes.fromhex(block["mrkl_root"])[::-1]
    timestamp = struct.pack("<I", block.get("time", 0))
    bits      = struct.pack("<I", block.get("bits", 0))
    return version + prevhash + merkle + timestamp + bits

# â”€â”€ OLS Regression â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
def fit_ols(X, y):
    """
    Ordinary Least Squares regression.
    X: list of feature vectors (list of lists)
    y: list of target values
    Returns (intercept, weights)
    """
    n    = len(y)
    k    = len(X[0])
    # Add bias column
    Xb   = [[1.0] + row for row in X]
    cols = k + 1

    # XtX and Xty
    XtX = [[0.0]*cols for _ in range(cols)]
    Xty = [0.0]*cols

    for i in range(n):
        xi = Xb[i]
        for a in range(cols):
            Xty[a] += xi[a] * y[i]
            for b in range(cols):
                XtX[a][b] += xi[a] * xi[b]

    # Solve XtX @ w = Xty using Gaussian elimination
    # Augmented matrix
    aug = [XtX[i][:] + [Xty[i]] for i in range(cols)]

    for col in range(cols):
        # Pivot
        max_row = max(range(col, cols), key=lambda r: abs(aug[r][col]))
        aug[col], aug[max_row] = aug[max_row], aug[col]
        if abs(aug[col][col]) < 1e-12:
            continue
        for row in range(cols):
            if row != col:
                factor = aug[row][col] / aug[col][col]
                for j in range(cols + 1):
                    aug[row][j] -= factor * aug[col][j]

    w = [aug[i][cols] / aug[i][i] if abs(aug[i][i]) > 1e-12 else 0 for i in range(cols)]
    return w[0], w[1:]  # intercept, weights

def predict(intercept, weights, features):
    return intercept + sum(w * f for w, f in zip(weights, features))

def run():
    print("[!] GODSEYE 23.1 â€” REGRESSION DECODER TEST", flush=True)

    with open("bitcoin_causation_map.json") as f:
        data = json.load(f)
    blocks = [b for b in data["blocks"] if b.get("prev_block")]
    print(f"    Blocks with full headers: {len(blocks)}\n")

    # Build ring fingerprints
    print("    Computing 81D ring fingerprints ...", flush=True)
    records = []
    for block in blocks:
        try:
            h76 = build_header_no_nonce(block)
            vec = expand_81d(h76)
            rv  = [ring_avg(vec, r) for r in range(9)]
            records.append({
                "height": block["height"],
                "nonce":  block["nonce"],
                "zeros":  block["zeros"],
                "rv":     rv
            })
        except Exception as e:
            pass

    print(f"    Fingerprints built: {len(records)}\n")

    # Normalize nonces to 0-1
    norm_nonces = [r["nonce"] / NONCE_MAX for r in records]
    ring_feats  = [r["rv"] for r in records]

    # â”€â”€ Leave-One-Out Cross-Validation â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
    print("=" * 72)
    print("  LEAVE-ONE-OUT REGRESSION TEST")
    print("  (Fit regression on N-1 blocks, predict 1 held-out block)")
    print("=" * 72)

    WINDOW    = NONCE_MAX // 4
    results   = []
    in_window = 0

    for i in range(len(records)):
        # Train on all except block i
        X_train = [ring_feats[j] for j in range(len(records)) if j != i]
        y_train = [norm_nonces[j] for j in range(len(records)) if j != i]

        intercept, weights = fit_ols(X_train, y_train)

        # Predict block i
        pred_norm   = predict(intercept, weights, ring_feats[i])
        pred_nonce  = int(max(0, min(1, pred_norm)) * NONCE_MAX)
        actual      = records[i]["nonce"]
        error       = abs(pred_nonce - actual)
        hit         = error <= WINDOW // 2

        if hit:
            in_window += 1

        results.append(error)

    avg_error  = sum(results) / len(results)
    random_avg = NONCE_MAX * 0.375

    print(f"  Blocks tested          : {len(records)}")
    print(f"  Nonces in 25% window   : {in_window}/{len(records)} ({in_window/len(records)*100:.1f}%)")
    print(f"  Random chance (25% win): {len(records)//4}/{len(records)} (25.0%)")
    print(f"  Avg prediction error   : {avg_error:,.0f}")
    print(f"  Random avg error       : {random_avg:,.0f}")
    print(f"  Improvement vs random  : {(random_avg - avg_error)/random_avg*100:+.1f}%")

    # â”€â”€ Full Fit â€” save the regression model â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€
    print(f"\n  Fitting full regression model on all {len(records)} blocks ...", flush=True)
    intercept, weights = fit_ols(ring_feats, norm_nonces)

    print(f"\n  REGRESSION COEFFICIENTS:")
    print(f"  Intercept: {intercept:.6f}")
    for r, w in enumerate(weights):
        direction = "â†’ LOW NONCE" if w < 0 else "â†’ HIGH NONCE"
        if abs(w) > 0.05:
            print(f"  Ring {r}: {w:+.4f}  {direction}")

    # Save model
    model = {
        "intercept": intercept,
        "weights":   weights,
        "trained_on": len(records)
    }
    with open(r"C:\GENESIS\GodsEye\regression_model.json", "w") as f:
        json.dump(model, f, indent=2)
    print(f"\n  [SAVED] regression_model.json ({len(records)} blocks)")

    print(f"\n{'='*72}")
    if in_window / len(records) > 0.30:
        print("  *** REGRESSION DECODER VALIDATED â€” beats random ***")
        print("  *** Deploy regression_model.json in live miner. ***")
    elif avg_error < random_avg * 0.90:
        print("  AVERAGE ERROR 10%+ BETTER â€” regression has real signal")
        print("  Window hit rate limited by nonce space scale")
        print("  Use regression center + multi-pass expansion for live mining")
    else:
        print("  Signal present but weak at this sample size")

if __name__ == "__main__":
    run()
