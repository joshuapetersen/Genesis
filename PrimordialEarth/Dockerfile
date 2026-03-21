FROM python:3.11-slim

# Set working directory
WORKDIR /app

# Copy simulation files
COPY Genesis_Societal_Ecology.py .
COPY cloud_server.py .
COPY requirements.txt .

# If a pre-seeded Vault is included, copy it
COPY *.sqlite /data/ 2>/dev/null || true

# Simulation data directory
RUN mkdir -p /data
ENV GENESIS_DATA=/data

# Start the watchdog server
CMD ["python", "cloud_server.py"]
