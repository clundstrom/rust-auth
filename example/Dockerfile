# Minimal python image
FROM python:3.10-slim

# Set the working directory
WORKDIR /app

# Copy the current directory contents into the container at /app
COPY . /app

# Install any needed packages specified in requirements.txt
RUN pip install --no-cache-dir -r requirements.txt

CMD ["uvicorn", "api:app", "--host", "0.0.0.0", "--port", "8000"]
