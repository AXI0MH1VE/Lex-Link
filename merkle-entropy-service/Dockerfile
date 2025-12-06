# Stage 1: Builder - Installs dependencies and prepares the application context
FROM python:3.11-slim as builder

# Set environment variables
ENV PYTHONDONTWRITEBYTECODE=1
ENV PYTHONUNBUFFERED=1

# Set work directory
WORKDIR /app

# Install dependencies
COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

# Stage 2: Runtime - Creates the final, minimal image
FROM python:3.11-slim as runtime

WORKDIR /app

# Set non-root user (best practice)
RUN adduser --disabled-password --gecos '' appuser
USER appuser

# Copy installed packages from the builder stage
COPY --from=builder /usr/local/lib/python3.11/site-packages /usr/local/lib/python3.11/site-packages
# Copy application code
COPY --chown=appuser:appuser mes mes

# Expose the Gunicorn port
EXPOSE 8000

# Healthcheck definition is essential for container orchestration systems
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD python -c "import urllib.request; urllib.request.urlopen('http://localhost:8000/health')" || exit 1

# Use Gunicorn as the production application server
# Binds to 0.0.0.0:8000 and runs the Flask application object 'app' inside main.py
CMD ["gunicorn", "--bind", "0.0.0.0:8000", "mes.main:app"]

