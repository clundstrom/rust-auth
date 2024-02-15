import requests
import os


login_url = os.environ.get("LOGIN_URL", "http://localhost:8080/login")
protected_url = os.environ.get("PROTECTED_URL", "http://localhost:8000/protected")

# JSON object containing the username and password
login_data = {
    'username': 'tester',
    'password': 'password'
}


# First, we try to access the protected endpoint without a token
response = requests.get(protected_url)
print("Protected endpoint response (no token):", response.status_code, response.text)

# Then, we try to login and get a token
response = requests.post(login_url, json=login_data)

# Check if the login was successful
if response.status_code == 200:
    token = response.text

    print("Login successful")
    print("Received token: " + token)

    if token:
        # Headers for the second request, including the Authorization Bearer token
        headers = {
            'Authorization': f'Bearer {token}'
        }

        # Send the second request to the protected endpoint
        protected_response = requests.get(protected_url, headers=headers)

        # Output the response from the protected endpoint
        print("Protected endpoint response:", protected_response.status_code, protected_response.text)
    else:
        print("Login successful, but no token was returned.")
else:
    print("Login failed:", response.status_code, response.text)
