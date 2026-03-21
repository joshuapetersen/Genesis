FROM python:3.11-slim

WORKDIR /app

# Copy the full simulation and sovereign mind structures
COPY Genesis_Societal_Ecology.py .
COPY Genesis_Cloud_Server.py .
COPY Sovereign_Cloud_Mind.py .
COPY World_Data_Bridge.py .
COPY Sovereign_Supabase.py .
COPY requirements_cloud.txt .
COPY .env .

# Copy DB if present (pre-seeded world) via wildcard so it doesn't fail if missing
COPY *Genesis_Soul_Vault.sqlite /app/

RUN pip install --no-cache-dir -r requirements_cloud.txt
RUN pip install python-dotenv supabase psutil

EXPOSE 8080
ENV PORT=8080

CMD ["python", "Genesis_Cloud_Server.py"]
